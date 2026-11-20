//! Transactional doctor workspace. Models may stage proposals; only explicit patient saves approve them.
use crate::{
    ApiError,
    database::AccountDatabase,
    evidence::{AddressedRecordRevision, exact_in_transaction},
    identity::{AccessLevel, AccountContext, check_current},
};
use chrono::Utc;
use duet_wire::models::{CanonicalRecordSelection, DoctorPageSave};
use serde_json::{Value, json};
use sqlx::{Postgres, Row, Transaction};
use uuid::Uuid;

/// Durable workspace owner, sharing the existing users and collection identities.
#[derive(Clone)]
pub struct WorkspaceStore {
    /// Account-scoped database.
    pub database: AccountDatabase,
}
impl WorkspaceStore {
    /// Construct the owner.
    pub fn new(database: AccountDatabase) -> Self {
        Self { database }
    }
    /// Return existing consumer DoctorCollection JSON, retaining string source identities.
    pub async fn read(&self, c: &AccountContext) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        collection(&mut tx, c.account_id).await
    }
    /// Select one exact current canonical revision. A mutable search result is never accepted.
    pub async fn select_record(
        &self,
        c: &AccountContext,
        selection: &CanonicalRecordSelection,
    ) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        let row=sqlx::query("select entity,client_record_id from evidence_sync_changes where user_id=$1 and record_id=$2 and not deleted").bind(c.account_id).bind(selection.record_id).fetch_optional(&mut *tx).await?.ok_or(ApiError::NotFound)?;
        let entity = serde_json::from_value(json!(row.get::<String, _>("entity")))
            .map_err(|_| ApiError::NotFound)?;
        let address = AddressedRecordRevision::new(
            selection.record_id,
            entity,
            row.get::<String, _>("client_record_id"),
            selection.revision,
        );
        let exact = exact_in_transaction(&mut tx, c.account_id, &address, true).await?;
        let entry = literal_entry(&exact.payload, false);
        let existing:bool=sqlx::query_scalar("select exists(select 1 from doctor_collection_items where user_id=$1 and source_id=$2 and kind='fact')").bind(c.account_id).bind(selection.record_id.to_string()).fetch_one(&mut *tx).await?;
        if !existing {
            sqlx::query("insert into doctor_collection_items(id,user_id,kind,source_id,title,detail,position,record_selection,page_entry) values($1,$2,'fact',$3,$4,$5,(select count(*) from doctor_collection_items where user_id=$2),$6,$7)")
    .bind(Uuid::new_v4()).bind(c.account_id).bind(selection.record_id.to_string()).bind(entry["title"].as_str().unwrap_or("Record")).bind(entry["lines"].to_string()).bind(json!(address)).bind(&entry).execute(&mut *tx).await?;
            advance(&mut tx, c.account_id).await?;
        }
        let result = collection(&mut tx, c.account_id).await?;
        tx.commit().await?;
        Ok(result)
    }
    /// Immutable device snapshot ingestion. New uploads cannot rewrite selected/approved older snapshots.
    pub async fn upload_device(
        &self,
        c: &AccountContext,
        payload: &Value,
    ) -> Result<Value, ApiError> {
        let id: Uuid = serde_json::from_value(payload["snapshot_id"].clone())
            .map_err(|_| ApiError::Unprocessable("snapshot_id required"))?;
        let generated: chrono::DateTime<Utc> =
            serde_json::from_value(payload["generated_at"].clone())
                .map_err(|_| ApiError::Unprocessable("dated snapshot required"))?;
        let facts = payload["facts"]
            .as_array()
            .ok_or(ApiError::Unprocessable("facts required"))?;
        if facts.len() > 1000 {
            return Err(ApiError::Unprocessable(
                "snapshot exceeds bounded fact count",
            ));
        }
        let mut ids = std::collections::HashSet::new();
        for fact in facts {
            let fact_id = fact["id"]
                .as_str()
                .ok_or(ApiError::Unprocessable("fact identity required"))?;
            if fact_id.is_empty() || !ids.insert(fact_id) {
                return Err(ApiError::Unprocessable(
                    "fact identities must be nonempty and unique",
                ));
            }
            for field in ["range_start", "range_end"] {
                fact[field]
                    .as_str()
                    .and_then(|v| v.parse::<chrono::DateTime<Utc>>().ok())
                    .ok_or(ApiError::Unprocessable("dated device interval required"))?;
            }
        }
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        let generation: i32 =
            sqlx::query_scalar("select apple_health_generation from users where id=$1")
                .bind(c.account_id)
                .fetch_one(&mut *tx)
                .await?;
        if payload["source_generation"].as_i64().unwrap_or(0) != i64::from(generation) {
            return Err(ApiError::Conflict("device source generation changed"));
        }
        sqlx::query("insert into duet_device_snapshots(user_id,id,generated_at,payload) values($1,$2,$3,$4) on conflict(user_id,id) do nothing").bind(c.account_id).bind(id).bind(generated).bind(payload).execute(&mut *tx).await?;
        let stored: Value = sqlx::query_scalar(
            "select payload from duet_device_snapshots where user_id=$1 and id=$2",
        )
        .bind(c.account_id)
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
        if stored != *payload {
            return Err(ApiError::Conflict(
                "snapshot identity already has different bytes",
            ));
        }
        tx.commit().await?;
        Ok(json!({"snapshot_id":id,"accepted":true}))
    }
    /// Read the latest immutable dated device snapshot, or explicit absence (never fabricated measurements).
    pub async fn device_snapshot(&self, c: &AccountContext) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        sqlx::query_scalar("select payload from duet_device_snapshots where user_id=$1 order by received_at desc,id desc limit 1").bind(c.account_id).fetch_optional(&mut *tx).await?.ok_or(ApiError::NotFound)
    }
    /// Select a literal dated fact from the latest device snapshot.
    pub async fn select_fact(&self, c: &AccountContext, fact_id: &str) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        let row=sqlx::query("select id,payload from duet_device_snapshots where user_id=$1 order by received_at desc,id desc limit 1").bind(c.account_id).fetch_optional(&mut *tx).await?.ok_or(ApiError::NotFound)?;
        let payload: Value = row.get("payload");
        let snapshot: Uuid = row.get("id");
        let fact = payload["facts"]
            .as_array()
            .and_then(|facts| facts.iter().find(|f| f["id"].as_str() == Some(fact_id)))
            .ok_or(ApiError::NotFound)?;
        let entry = literal_entry(fact, true);
        let exists:bool=sqlx::query_scalar("select exists(select 1 from doctor_collection_items where user_id=$1 and source_id=$2 and kind='fact')").bind(c.account_id).bind(fact_id).fetch_one(&mut *tx).await?;
        if !exists {
            sqlx::query("insert into doctor_collection_items(id,user_id,kind,source_id,title,detail,position,fact_snapshot,fact_snapshot_id,page_entry) values($1,$2,'fact',$3,$4,$5,(select count(*) from doctor_collection_items where user_id=$2),$6,$7,$8)")
    .bind(Uuid::new_v4()).bind(c.account_id).bind(fact_id).bind(entry["title"].as_str().unwrap_or("Device measurement")).bind(entry["lines"].to_string()).bind(fact).bind(snapshot).bind(&entry).execute(&mut *tx).await?;
            advance(&mut tx, c.account_id).await?;
        }
        let result = collection(&mut tx, c.account_id).await?;
        tx.commit().await?;
        Ok(result)
    }
    /// Require an explicitly saved current generation before PDF/share checks.
    pub async fn approved_read(&self, c: &AccountContext) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        let page = collection(&mut tx, c.account_id).await?;
        let saved:bool=sqlx::query_scalar("select exists(select 1 from duet_doctor_saves where user_id=$1 and (result->>'note_generation')::bigint=$2)").bind(c.account_id).bind(page["note_generation"].as_i64().ok_or(ApiError::Internal("doctor page is missing its generation"))?).fetch_one(&mut *tx).await?;
        if !saved {
            return Err(ApiError::Conflict("explicit page save required"));
        }
        for item in page["items"].as_array().into_iter().flatten() {
            validate_item(&mut tx, c.account_id, item).await?;
        }
        Ok(page)
    }
    /// Read back a specific uncertain save acknowledgement without replaying its mutation.
    /// Only the current approved generation is retained; privacy deletion cannot resurrect older content.
    pub async fn save_receipt(&self, c: &AccountContext, expected: i64) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        sqlx::query_scalar(
            "select result from duet_doctor_saves where user_id=$1 and expected_generation=$2",
        )
        .bind(c.account_id)
        .bind(expected)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ApiError::NotFound)
    }
    /// Reorder the exact current membership without selecting, dropping, or approving new content.
    pub async fn reorder(&self, c: &AccountContext, ids: &[Uuid]) -> Result<Value, ApiError> {
        let current = self.read(c).await?;
        let actual: std::collections::HashSet<Uuid> = current["items"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|i| serde_json::from_value(i["id"].clone()).ok())
            .collect();
        if actual != ids.iter().copied().collect() || actual.len() != ids.len() {
            return Err(ApiError::Conflict("collection membership changed"));
        }
        let mut request = DoctorPageSave::new(
            current["note_generation"]
                .as_i64()
                .ok_or(ApiError::Internal("doctor page is missing its generation"))?
                as i32,
            ids.to_vec(),
            serde_json::from_value(current["page"].clone())
                .map_err(|_| ApiError::Conflict("invalid page"))?,
        );
        request.confirmed_medication_ids = Some(
            current["items"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|i| i["medication_confirmed"] == true)
                .filter_map(|i| serde_json::from_value(i["id"].clone()).ok())
                .collect(),
        );
        self.save(c, &request).await
    }
    /// Save exact membership/order and patient wording with optimistic generation fencing.
    /// Overflow is not a save failure. An identical retry returns its original committed result.
    pub async fn save(
        &self,
        c: &AccountContext,
        request: &DoctorPageSave,
    ) -> Result<Value, ApiError> {
        let value = json!(request);
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, AccessLevel::AcceptedTerms).await?;
        if let Some(row)=sqlx::query("select request,result from duet_doctor_saves where user_id=$1 and expected_generation=$2").bind(c.account_id).bind(i64::from(request.expected_generation)).fetch_optional(&mut *tx).await? {
   if row.get::<Value,_>("request")==value{return Ok(row.get("result"));}return Err(ApiError::Conflict("workspace generation already saved"));
  }
        let before = collection(&mut tx, c.account_id).await?;
        if before["note_generation"].as_i64() != Some(i64::from(request.expected_generation)) {
            return Err(ApiError::Conflict("doctor workspace changed"));
        }
        if request.proposal_id.flatten().is_some() {
            return Err(ApiError::Conflict("proposals retired"));
        }
        let candidates = before["items"].as_array().cloned().unwrap_or_default();
        let ids: std::collections::HashSet<_> = request.item_ids.iter().copied().collect();
        if ids.len() != request.item_ids.len() {
            return Err(ApiError::Unprocessable("duplicate selected item"));
        }
        let confirmed = request.confirmed_medication_ids.clone().unwrap_or_default();
        if confirmed.iter().any(|id| !ids.contains(id)) {
            return Err(ApiError::Conflict("medication confirmation not selected"));
        }
        let mut selected = Vec::new();
        for (position, id) in request.item_ids.iter().enumerate() {
            let mut item = candidates
                .iter()
                .find(|i| i["id"].as_str() == Some(&id.to_string()))
                .cloned()
                .ok_or(ApiError::Conflict("selected membership changed"))?;
            validate_item(&mut tx, c.account_id, &item).await?;
            let medication = item
                .pointer("/page_entry/group")
                .and_then(Value::as_str)
                .is_some_and(|g| g == "recorded_medications" || g == "confirmed_medications");
            if confirmed.contains(id) && !medication {
                return Err(ApiError::Conflict("only medications may be confirmed"));
            }
            item["medication_confirmed"] = json!(confirmed.contains(id));
            if medication {
                item["page_entry"]["group"] = json!(if confirmed.contains(id) {
                    "confirmed_medications"
                } else {
                    "recorded_medications"
                });
            }
            item["position"] = json!(position);
            selected.push(item);
        }
        // Every removed item gets a recovery receipt in this same transaction.
        for item in before["items"].as_array().into_iter().flatten() {
            let id: Uuid = serde_json::from_value(item["id"].clone())
                .map_err(|_| ApiError::Conflict("invalid stored item"))?;
            if !ids.contains(&id) {
                crate::privacy::record_in_transaction(
                    &mut tx,
                    c,
                    crate::privacy::Restriction::DoctorItemDelete,
                    id,
                    i64::from(request.expected_generation),
                )
                .await?;
                sqlx::query("update duet_publications set revoked_at=coalesce(revoked_at,now()) where user_id=$1 and snapshot->'items' @> jsonb_build_array(jsonb_build_object('id',$2::text))").bind(c.account_id).bind(id.to_string()).execute(&mut *tx).await?;
            }
        }
        sqlx::query("delete from duet_doctor_saves where user_id=$1")
            .bind(c.account_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("delete from doctor_collection_items where user_id=$1")
            .bind(c.account_id)
            .execute(&mut *tx)
            .await?;
        for item in &selected {
            insert_item(&mut tx, c.account_id, item).await?;
        }
        let note = request
            .note
            .clone()
            .unwrap_or_else(|| before["note"].as_str().map(str::to_owned));
        if note.is_none() && before["note"].is_string() {
            crate::privacy::record_in_transaction(
                &mut tx,
                c,
                crate::privacy::Restriction::DoctorNoteDelete,
                c.account_id,
                i64::from(request.expected_generation),
            )
            .await?;
        }
        sqlx::query("update users set doctor_note=$2,doctor_page=$3,doctor_note_generation=doctor_note_generation+1,doctor_page_updated_at=now(),doctor_proposal=null where id=$1").bind(c.account_id).bind(note).bind(json!(request.page)).execute(&mut *tx).await?;
        sqlx::query("update duet_doctor_proposals set state='superseded' where user_id=$1 and state='pending'").bind(c.account_id).execute(&mut *tx).await?;
        let result = collection(&mut tx, c.account_id).await?;
        sqlx::query("insert into duet_doctor_saves(user_id,expected_generation,request,result) values($1,$2,$3,$4)").bind(c.account_id).bind(i64::from(request.expected_generation)).bind(value).bind(&result).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}
/// Build the existing collection shape under the account mutation lock.
pub async fn collection(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
) -> Result<Value, ApiError> {
    let row=sqlx::query("select doctor_note,doctor_note_generation,doctor_page,doctor_page_updated_at from users where id=$1").bind(account).fetch_optional(&mut **tx).await?.ok_or(ApiError::AbsentAccount)?;
    let rows=sqlx::query("select to_jsonb(i) as item from doctor_collection_items i where user_id=$1 order by position,saved_at,id").bind(account).fetch_all(&mut **tx).await?;
    let mut items = Vec::new();
    for row in rows {
        let mut item: Value = row.get("item");
        item["record"] = item["record_selection"].clone();
        item["review_required"] = json!(validate_item(tx, account, &item).await.is_err());
        item.as_object_mut().unwrap().remove("user_id");
        item.as_object_mut().unwrap().remove("record_selection");
        items.push(item);
    }
    Ok(
        json!({"items":items,"note":row.get::<Option<String>,_>("doctor_note"),"note_generation":row.get::<i64,_>("doctor_note_generation"),"page":row.get::<Value,_>("doctor_page"),"proposal":Value::Null,"page_updated_at":row.get::<chrono::DateTime<Utc>,_>("doctor_page_updated_at")}),
    )
}
/// Original read-only preparation envelope for older clients, derived only from approved saved state.
pub fn legacy_preparation(c: &Value) -> Value {
    let mut sections = Vec::new();
    let page = &c["page"];
    if page["name"].is_string() || page["date_of_birth"].is_string() {
        sections.push(json!({"title":"Patient","body":format!("{} · DOB: {}",page["name"].as_str().unwrap_or("Name not provided"),page["date_of_birth"].as_str().unwrap_or("Not provided")),"item_ids":[]}));
    }
    if let Some(note) = c["note"].as_str().filter(|n| !n.is_empty()) {
        sections.push(json!({"title":"What I want to discuss","body":note,"item_ids":[]}));
    }
    if let Some(questions) = page["questions"].as_array().filter(|q| !q.is_empty()) {
        sections.push(json!({"title":"My questions","body":questions.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n"),"item_ids":[]}));
    }
    let items = c["items"].as_array().cloned().unwrap_or_default();
    for (group, title) in [
        ("records", "From my records"),
        ("devices", "From my devices"),
        ("confirmed_medications", "What I take (confirmed by me)"),
        ("recorded_medications", "Other medications in my records"),
    ] {
        let selected: Vec<_> = items
            .iter()
            .filter(|i| i["page_entry"]["group"] == group)
            .collect();
        if !selected.is_empty() {
            let body = selected
                .iter()
                .map(|i| {
                    let mut lines = vec![i["page_entry"]["title"].as_str().unwrap_or("")];
                    if let Some(values) = i["page_entry"]["lines"].as_array() {
                        lines.extend(values.iter().filter_map(Value::as_str));
                    }
                    lines.join("\n")
                })
                .collect::<Vec<_>>()
                .join("\n\n");
            sections.push(json!({"title":title,"body":body,"item_ids":selected.iter().map(|i|&i["id"]).collect::<Vec<_>>()}));
        }
    }
    if sections.is_empty() {
        return json!({"state":"absent","revision":null,"draft":null});
    }
    json!({"state":if items.iter().any(|i|i["review_required"]==true){"outdated"}else{"ready"},"revision":c["note_generation"],"draft":{"title":"For my doctor","prepared_at":c["page_updated_at"],"sections":sections,"evidence_items":[],"patient_prepared_notice":"Prepared by me with help from Duet, an AI app, and reviewed by me"}})
}
/// Enforce exact source currentness; device snapshots remain pinned, not silently refreshed.
pub async fn validate_item(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    item: &Value,
) -> Result<(), ApiError> {
    let record = item
        .get("record")
        .filter(|v| !v.is_null())
        .or_else(|| item.get("record_selection").filter(|v| !v.is_null()));
    if let Some(address) = record {
        let address: AddressedRecordRevision = serde_json::from_value(address.clone())
            .map_err(|_| ApiError::Conflict("invalid exact source address"))?;
        let exact = exact_in_transaction(tx, account, &address, true).await?;
        let mut expected = literal_entry(&exact.payload, false);
        if item["medication_confirmed"] == true && expected["group"] == "recorded_medications" {
            expected["group"] = json!("confirmed_medications");
        }
        if item["page_entry"] != expected {
            return Err(ApiError::Conflict("page entry differs from exact source"));
        }
    } else if !item["fact_snapshot_id"].is_null() {
        let id: Uuid = serde_json::from_value(item["fact_snapshot_id"].clone())
            .map_err(|_| ApiError::Conflict("invalid snapshot"))?;
        let payload: Value = sqlx::query_scalar(
            "select payload from duet_device_snapshots where user_id=$1 and id=$2",
        )
        .bind(account)
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(ApiError::Conflict("device source withdrawn"))?;
        if !payload["facts"]
            .as_array()
            .is_some_and(|facts| facts.contains(&item["fact_snapshot"]))
            || item["page_entry"] != literal_entry(&item["fact_snapshot"], true)
        {
            return Err(ApiError::Conflict("device fact differs from snapshot"));
        }
    } else {
        return Err(ApiError::Conflict("item lacks exact source"));
    }
    Ok(())
}
async fn advance(tx: &mut Transaction<'_, Postgres>, id: Uuid) -> Result<(), ApiError> {
    sqlx::query("update users set doctor_note_generation=doctor_note_generation+1,doctor_page_updated_at=now() where id=$1").bind(id).execute(&mut **tx).await?;
    Ok(())
}
async fn insert_item(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    item: &Value,
) -> Result<(), ApiError> {
    sqlx::query("insert into doctor_collection_items(id,user_id,kind,source_id,title,detail,evidence,position,record_selection,fact_snapshot,fact_snapshot_id,page_entry,medication_confirmed) values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)")
 .bind(serde_json::from_value::<Uuid>(item["id"].clone()).map_err(|_|ApiError::Conflict("invalid item"))?).bind(account).bind(item["kind"].as_str().unwrap_or("fact")).bind(item["source_id"].as_str().unwrap_or("")).bind(item["title"].as_str().unwrap_or("")).bind(item["detail"].as_str().unwrap_or("")).bind(item.get("evidence").cloned().unwrap_or(json!([]))).bind(item["position"].as_i64().unwrap_or(0) as i32).bind(item.get("record").filter(|v|!v.is_null())).bind(item.get("fact_snapshot").filter(|v|!v.is_null())).bind(item["fact_snapshot_id"].as_str().and_then(|s|s.parse::<Uuid>().ok())).bind(&item["page_entry"]).bind(item["medication_confirmed"].as_bool().unwrap_or(false)).execute(&mut **tx).await?;
    Ok(())
}
fn is_reference_range_key(key: &str) -> bool {
    let normalized: String = key
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    matches!(
        normalized.as_str(),
        "referencerange"
            | "referenceranges"
            | "refrange"
            | "refranges"
            | "normalrange"
            | "normalranges"
            | "printedreferencerange"
            | "labreferencerange"
    )
}

/// Literal record projection, excluding printed lab reference ranges per safety rules.
pub fn literal_entry(payload: &Value, device: bool) -> Value {
    fn flatten(prefix: &str, value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    if is_reference_range_key(k) {
                        continue;
                    }
                    flatten(
                        &if prefix.is_empty() {
                            k.clone()
                        } else {
                            format!("{prefix}.{k}")
                        },
                        v,
                        out,
                    )
                }
            }
            Value::Array(values) => {
                for (i, v) in values.iter().enumerate() {
                    flatten(&format!("{prefix}[{i}]"), v, out)
                }
            }
            Value::String(s) => out.push(format!("{prefix}: {s}")),
            _ => out.push(format!("{prefix}: {value}")),
        }
    }
    let mut lines = Vec::new();
    flatten("", payload, &mut lines);
    let title = payload["display_name"]
        .as_str()
        .or(payload["title"].as_str())
        .unwrap_or(if device {
            "Device measurement"
        } else {
            "Health record"
        });
    let medication = payload["record_type"]
        .as_str()
        .is_some_and(|s| s == "medication")
        || payload
            .pointer("/clinical_record/resource_type")
            .and_then(Value::as_str)
            .is_some_and(|s| s.starts_with("Medication"));
    json!({"group":if device{"devices"}else if medication{"recorded_medications"}else{"records"},"title":title,"lines":lines})
}

#[cfg(test)]
mod tests {
    use super::literal_entry;
    use serde_json::json;

    #[test]
    fn literal_entry_excludes_lab_printed_reference_ranges_and_keeps_values() {
        let lab = json!({
            "category": "lab",
            "display_name": "Potassium",
            "value": "5.4",
            "unit": "mmol/L",
            "recorded_at": "2026-10-01T14:30:00Z",
            "source_label": "Regional Lab",
            "referenceRange": [{"low": {"value": 3.5}, "high": {"value": 5.0}, "text": "3.5-5.0 mmol/L"}],
            "clinical_record": {
                "resource_type": "Observation",
                "reference_range": "3.5 - 5.0 mmol/L",
                "normal_range": {"low": 3.5, "high": 5.0}
            }
        });
        let entry = literal_entry(&lab, false);
        let joined = entry["lines"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("value: 5.4"), "missing lab value: {joined}");
        assert!(
            joined.contains("unit: mmol/L"),
            "missing lab unit: {joined}"
        );
        assert!(
            !joined.contains("3.5")
                && !joined.contains("reference")
                && !joined.contains("normal_range"),
            "lab printed reference range must never appear in literal_entry: {joined}"
        );
    }
}
