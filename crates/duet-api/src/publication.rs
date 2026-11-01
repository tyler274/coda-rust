//! Pinned publication owner for ordinary web and SMART Health Links v1.
//! Capabilities are random; lookup is hashed, keys are encrypted at rest and never logged.
use crate::{
    ApiError,
    database::AccountDatabase,
    identity::{AccountContext, check_current},
    workspace,
};
use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{Aead, Payload},
};
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD as B64},
};
use chrono::{DateTime, Duration, Utc};
use rand::{TryRng, rngs::SysRng};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Row, Transaction};
use std::sync::Arc;
use uuid::Uuid;
/// Explicit publish approval for the exact reviewed workspace generation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishRequest {
    /// Stable retry identity.
    pub operation_id: Uuid,
    /// Reviewed approved generation.
    pub expected_generation: i64,
    /// Lifetime in seconds; defaults to seven days, maximum thirty days.
    #[serde(default = "default_lifetime")]
    pub expires_in_seconds: i64,
}
fn default_lifetime() -> i64 {
    604800
}
/// Patient-only publication control response. Secrets must not enter telemetry or query strings.
#[derive(Clone, Serialize, Deserialize)]
pub struct PublicationView {
    /// Durable publication identity.
    pub id: Uuid,
    /// Exact snapshot generation.
    pub generation: i64,
    /// Absolute expiry.
    pub expires_at: DateTime<Utc>,
    /// Revocation state.
    pub revoked: bool,
    /// Ordinary account-free URL.
    pub web_url: String,
    /// Viewer-prefixed SMART Health Link; encryption key exists only in the fragment.
    pub shl_url: String,
    /// SHA-256 of exact PDF bytes.
    pub content_sha256: String,
}
/// Independent one-minute windows for valid retrievals and failed lookups.
struct RetrievalWindow {
    started: std::time::Instant,
    used: u32,
}

impl RetrievalWindow {
    fn new() -> Self {
        Self {
            started: std::time::Instant::now(),
            used: 0,
        }
    }
}

struct RetrievalBudget {
    hit: RetrievalWindow,
    miss: RetrievalWindow,
}

impl RetrievalBudget {
    fn new() -> Self {
        Self {
            hit: RetrievalWindow::new(),
            miss: RetrievalWindow::new(),
        }
    }
}

/// Shared capability owner. Public responses always revalidate the pinned source selection.
#[derive(Clone)]
pub struct PublicationStore {
    /// Shared account boundary.
    pub database: AccountDatabase,
    origin: String,
    viewer: String,
    master: Arc<[u8; 32]>,
    retrieval_budget: Arc<std::sync::Mutex<RetrievalBudget>>,
}
impl PublicationStore {
    /// Configure canonical HTTPS origin and viewer; localhost HTTP is permitted for isolated proof.
    pub fn new(
        database: AccountDatabase,
        origin: String,
        viewer: String,
        master: [u8; 32],
    ) -> Result<Self, ApiError> {
        let origin = origin.trim_end_matches('/').to_owned();
        let valid = |value: &str| {
            reqwest::Url::parse(value).ok().is_some_and(|url| {
                url.host_str().is_some()
                    && url.username().is_empty()
                    && url.password().is_none()
                    && url.fragment().is_none()
                    && url.query().is_none()
                    && (url.scheme() == "https"
                        || (url.scheme() == "http"
                            && matches!(url.host_str(), Some("localhost" | "127.0.0.1"))))
            })
        };
        if !valid(&origin) || !valid(&viewer) {
            return Err(ApiError::Unprocessable(
                "invalid publication origin or viewer",
            ));
        }
        Ok(Self {
            database,
            origin,
            viewer,
            master: Arc::new(master),
            retrieval_budget: Arc::new(std::sync::Mutex::new(RetrievalBudget::new())),
        })
    }
    /// Create a pinned snapshot/PDF in the same transaction as generation and exact-source checks.
    pub async fn publish(
        &self,
        c: &AccountContext,
        request: &PublishRequest,
    ) -> Result<PublicationView, ApiError> {
        if !(60..=2_592_000).contains(&request.expires_in_seconds) {
            return Err(ApiError::Unprocessable(
                "expiry must be between one minute and thirty days",
            ));
        }
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, false).await?;
        if let Some(row) =
            sqlx::query("select * from duet_publications where user_id=$1 and operation_id=$2")
                .bind(c.account_id)
                .bind(request.operation_id)
                .fetch_optional(&mut *tx)
                .await?
        {
            if row.get::<Value, _>("request") != json!(request) {
                return Err(ApiError::Conflict("publication retry differs"));
            }
            return self.view(&row);
        }
        let mut snapshot = workspace::collection(&mut tx, c.account_id).await?;
        snapshot["proposal"] = Value::Null;
        let approved:bool=sqlx::query_scalar("select exists(select 1 from duet_doctor_saves where user_id=$1 and (result->>'note_generation')::bigint=$2)").bind(c.account_id).bind(request.expected_generation).fetch_one(&mut *tx).await?;
        if !approved {
            return Err(ApiError::Conflict(
                "explicit page save required before publication",
            ));
        }
        if snapshot["note_generation"].as_i64() != Some(request.expected_generation) {
            return Err(ApiError::Conflict("approved page changed"));
        }
        validate_sources(&mut tx, c.account_id, &snapshot).await?;
        let layout = duet_page::layout_approved(&workspace::approved(&snapshot));
        let pdf = layout.pdf().map_err(|_| {
            ApiError::Conflict("page cannot export faithfully; save remains available")
        })?;
        let digest = hash(&pdf);
        let id = Uuid::new_v4();
        let token = random_secret()?;
        let shl = random_secret()?;
        let key = random_secret()?;
        let protected = seal(
            &self.master,
            serde_json::to_vec(&json!({"web":token,"shl":shl,"key":key}))
                .map_err(|_| ApiError::Unavailable("encoding failed"))?
                .as_slice(),
            id.as_bytes(),
        )?;
        let expires: DateTime<Utc> = sqlx::query_scalar("select now()+$1*interval '1 second'")
            .bind(request.expires_in_seconds)
            .fetch_one(&mut *tx)
            .await?;
        let old: Vec<Uuid> = sqlx::query_scalar(
            "select id from duet_publications where user_id=$1 and revoked_at is null",
        )
        .bind(c.account_id)
        .fetch_all(&mut *tx)
        .await?;
        for old in old {
            crate::privacy::record_in_transaction(
                &mut tx,
                c,
                crate::privacy::Restriction::PublicationRevoke,
                old,
                0,
            )
            .await?;
        }
        sqlx::query(
            "update duet_publications set revoked_at=now() where user_id=$1 and revoked_at is null",
        )
        .bind(c.account_id)
        .execute(&mut *tx)
        .await?;
        sqlx::query("insert into duet_publications(id,user_id,operation_id,request,generation,token_hash,shl_hash,protected_capabilities,snapshot,pdf,digest,expires_at) values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)")
   .bind(id).bind(c.account_id).bind(request.operation_id).bind(json!(request)).bind(request.expected_generation).bind(hash(token.as_bytes())).bind(hash(shl.as_bytes())).bind(protected).bind(snapshot).bind(pdf).bind(digest).bind(expires).execute(&mut *tx).await?;
        let row = sqlx::query("select * from duet_publications where id=$1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;
        let view = self.view(&row)?;
        tx.commit().await?;
        Ok(view)
    }
    /// Current control state, not a newly minted link. Restart/retry returns the same capabilities.
    pub async fn status(&self, c: &AccountContext) -> Result<Option<PublicationView>, ApiError> {
        let mut tx = self.database.begin(c.account_id).await?;
        check_current(&mut tx, c, false).await?;
        let row=sqlx::query("select * from duet_publications where user_id=$1 order by created_at desc,id desc limit 1").bind(c.account_id).fetch_optional(&mut *tx).await?;
        row.map(|r| self.view(&r)).transpose()
    }
    fn view(&self, row: &sqlx::postgres::PgRow) -> Result<PublicationView, ApiError> {
        let id: Uuid = row.get("id");
        let secrets: Value = serde_json::from_slice(&open(
            &self.master,
            &row.get::<Vec<u8>, _>("protected_capabilities"),
            id.as_bytes(),
        )?)
        .map_err(|_| ApiError::Unavailable("invalid capability envelope"))?;
        let expires: DateTime<Utc> = row.get("expires_at");
        let shlink = json!({"url":format!("{}/shl/{}/manifest",self.origin,secrets["shl"].as_str().unwrap_or("")),"key":secrets["key"],"exp":expires.timestamp(),"flag":"","label":"For my doctor"});
        Ok(PublicationView {
            id,
            generation: row.get("generation"),
            expires_at: expires,
            revoked: row.get::<Option<DateTime<Utc>>, _>("revoked_at").is_some(),
            web_url: format!(
                "{}/shared/for-my-doctor/{}",
                self.origin,
                secrets["web"].as_str().unwrap_or("")
            ),
            shl_url: format!(
                "{}#shlink:/{}",
                self.viewer,
                B64.encode(
                    serde_json::to_vec(&shlink)
                        .map_err(|_| ApiError::Unavailable("encoding failed"))?
                )
            ),
            content_sha256: row.get("digest"),
        })
    }
    /// Revalidate public capability on every HTML/PDF/manifest/file request, including source corrections.
    pub async fn retrieve(&self, token: &str, shl: bool) -> Result<PublicSnapshot, ApiError> {
        let well_formed = token.len() == 43
            && token
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte));
        if !well_formed {
            self.charge_retrieval(false).await?;
            return Err(ApiError::NotFound);
        }
        let Some((id, account)) = self
            .database
            .publication_owner(&hash(token.as_bytes()), shl)
            .await?
        else {
            self.charge_retrieval(false).await?;
            return Err(ApiError::NotFound);
        };
        match self.load_public_snapshot(id, account).await {
            Ok(snapshot) => {
                self.charge_retrieval(true).await?;
                Ok(snapshot)
            }
            Err(ApiError::NotFound) => {
                self.charge_retrieval(false).await?;
                Err(ApiError::NotFound)
            }
            Err(other) => Err(other),
        }
    }
    async fn load_public_snapshot(
        &self,
        id: Uuid,
        account: Uuid,
    ) -> Result<PublicSnapshot, ApiError> {
        let mut tx = self.database.begin(account).await?;
        sqlx::query("select id from users where id=$1 for update")
            .bind(account)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::NotFound)?;
        let row=sqlx::query("select * from duet_publications where id=$1 and user_id=$2 and revoked_at is null and expires_at>now()").bind(id).bind(account).fetch_optional(&mut *tx).await?.ok_or(ApiError::NotFound)?;
        let snapshot: Value = row.get("snapshot");
        validate_sources(&mut tx, account, &snapshot)
            .await
            .map_err(|_| ApiError::NotFound)?;
        let secrets: Value = serde_json::from_slice(&open(
            &self.master,
            &row.get::<Vec<u8>, _>("protected_capabilities"),
            id.as_bytes(),
        )?)
        .map_err(|_| ApiError::NotFound)?;
        let key: Vec<u8> = B64
            .decode(secrets["key"].as_str().ok_or(ApiError::NotFound)?)
            .map_err(|_| ApiError::NotFound)?;
        Ok(PublicSnapshot {
            id,
            snapshot,
            pdf: row.get("pdf"),
            expires_at: row.get("expires_at"),
            key: key.try_into().map_err(|_| ApiError::NotFound)?,
        })
    }
    /// Charge the hit bucket or the miss bucket. A full miss window does not
    /// affect a valid capability, and the reverse.
    ///
    /// Raw SQL is allowed because this is the existing plpgsql bucket, not
    /// ordinary CRUD.
    async fn charge_retrieval(&self, hit: bool) -> Result<(), ApiError> {
        let admitted = if hit {
            self.database.admit_public_retrieval().await?
        } else {
            self.database.admit_public_retrieval_miss().await?
        };
        if !admitted {
            return Err(ApiError::Unavailable("public retrieval capacity reached"));
        }
        let mut budget = self
            .retrieval_budget
            .lock()
            .map_err(|_| ApiError::NotFound)?;
        let window = if hit {
            &mut budget.hit
        } else {
            &mut budget.miss
        };
        if window.started.elapsed() >= std::time::Duration::from_secs(60) {
            *window = RetrievalWindow::new();
        }
        if window.used >= 600 {
            return Err(ApiError::Unavailable("public retrieval capacity reached"));
        }
        window.used += 1;
        Ok(())
    }
    /// SHL manifest locations are bounded by one hour and publication expiry, and rechecked on fetch.
    pub async fn manifest(&self, token: &str) -> Result<Value, ApiError> {
        let p = self.retrieve(token, true).await?;
        let expires = std::cmp::min(p.expires_at, Utc::now() + Duration::hours(1)).timestamp();
        let location = seal(
            &self.master,
            &serde_json::to_vec(&json!({"token":token,"exp":expires}))
                .map_err(|_| ApiError::NotFound)?,
            b"shl-file-v1",
        )?;
        Ok(
            json!({"files":[{"contentType":"application/fhir+json","location":format!("{}/shl-file/{}",self.origin,B64.encode(location))}]}),
        )
    }
    /// Serve compact JWE (dir/A256GCM) containing a FHIR R4 DocumentReference with exact PDF attachment.
    pub async fn shl_file(&self, location: &str) -> Result<String, ApiError> {
        // A failed decrypt or lookup charges the miss bucket only. A valid file
        // charges one hit inside retrieve.
        let token = match self.shl_location_token(location) {
            Ok(token) => token,
            Err(ApiError::NotFound) => {
                self.charge_retrieval(false).await?;
                return Err(ApiError::NotFound);
            }
            Err(other) => return Err(other),
        };
        let p = self.retrieve(&token, true).await?;
        let resource = json!({"resourceType":"DocumentReference","status":"current","type":{"text":"Patient-prepared For my doctor"},"content":[{"attachment":{"contentType":"application/pdf","data":STANDARD.encode(&p.pdf),"title":"For my doctor"}}]});
        compact_jwe(
            &p.key,
            &serde_json::to_vec(&resource).map_err(|_| ApiError::NotFound)?,
        )
    }
    fn shl_location_token(&self, location: &str) -> Result<String, ApiError> {
        if location.len() > 1024 {
            return Err(ApiError::NotFound);
        }
        let body = open(
            &self.master,
            &B64.decode(location).map_err(|_| ApiError::NotFound)?,
            b"shl-file-v1",
        )?;
        let value: Value = serde_json::from_slice(&body).map_err(|_| ApiError::NotFound)?;
        if value["exp"].as_i64().unwrap_or(0) <= Utc::now().timestamp() {
            return Err(ApiError::NotFound);
        }
        value["token"]
            .as_str()
            .map(str::to_owned)
            .ok_or(ApiError::NotFound)
    }
}
/// Authorized pinned bytes returned only after current retrieval checks.
pub struct PublicSnapshot {
    /// Publication ID.
    pub id: Uuid,
    /// Approved collection, no private memory or unrelated notes.
    pub snapshot: Value,
    /// Exact published PDF.
    pub pdf: Vec<u8>,
    /// Publication expiry.
    pub expires_at: DateTime<Utc>,
    key: [u8; 32],
}
async fn validate_sources(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    snapshot: &Value,
) -> Result<(), ApiError> {
    for item in snapshot["items"].as_array().into_iter().flatten() {
        workspace::validate_item(tx, account, item).await?;
    }
    Ok(())
}
/// Fill `dest` from the operating system's CSPRNG; an entropy failure refuses
/// publication instead of panicking or falling back to a weaker source.
fn os_random(dest: &mut [u8]) -> Result<(), ApiError> {
    SysRng
        .try_fill_bytes(dest)
        .map_err(|_| ApiError::Unavailable("system randomness unavailable"))
}
fn random_secret() -> Result<String, ApiError> {
    let mut b = [0; 32];
    os_random(&mut b)?;
    Ok(B64.encode(b))
}
/// Hash capability lookup without retaining plaintext lookup tokens.
pub fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
fn seal(key: &[u8; 32], bytes: &[u8], aad: &[u8]) -> Result<Vec<u8>, ApiError> {
    let mut nonce = [0; 12];
    os_random(&mut nonce)?;
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|_| ApiError::Unavailable("invalid key"))?;
    let encrypted = cipher
        .encrypt((&nonce).into(), Payload { msg: bytes, aad })
        .map_err(|_| ApiError::Unavailable("encryption failed"))?;
    Ok([nonce.to_vec(), encrypted].concat())
}
fn open(key: &[u8; 32], bytes: &[u8], aad: &[u8]) -> Result<Vec<u8>, ApiError> {
    if bytes.len() < 28 {
        return Err(ApiError::NotFound);
    }
    let nonce: [u8; 12] = bytes[..12].try_into().map_err(|_| ApiError::NotFound)?;
    Aes256Gcm::new_from_slice(key)
        .map_err(|_| ApiError::NotFound)?
        .decrypt(
            (&nonce).into(),
            Payload {
                msg: &bytes[12..],
                aad,
            },
        )
        .map_err(|_| ApiError::NotFound)
}
/// Standards compact JWE: protected header is authenticated, encrypted-key segment empty for dir.
pub fn compact_jwe(key: &[u8; 32], plaintext: &[u8]) -> Result<String, ApiError> {
    let header = B64.encode(br#"{"alg":"dir","enc":"A256GCM","cty":"application/fhir+json"}"#);
    let body = seal(key, plaintext, header.as_bytes())?;
    let tag = body.len() - 16;
    Ok(format!(
        "{}..{}.{}.{}",
        header,
        B64.encode(&body[..12]),
        B64.encode(&body[12..tag]),
        B64.encode(&body[tag..])
    ))
}
/// Escape all user-controlled HTML text; URLs are not accepted from the source payload.
pub fn html(snapshot: &Value) -> String {
    fn escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
    let page = workspace::approved(snapshot);
    let mut out = String::from(
        "<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><title>For my doctor</title><main><h1>For my doctor</h1>",
    );
    for text in [
        page.name,
        page.date_of_birth,
        page.prepared_date,
        page.concern,
    ]
    .into_iter()
    .flatten()
    {
        out.push_str(&format!("<p>{}</p>", escape(&text)));
    }
    for q in page.questions {
        out.push_str(&format!("<p>{}</p>", escape(&q)));
    }
    for entry in page.entries {
        out.push_str(&format!(
            "<section><h2>{}</h2><h3>{}</h3>",
            escape(entry.group.heading()),
            escape(&entry.title)
        ));
        for line in entry.lines {
            out.push_str(&format!("<p>{}</p>", escape(&line)));
        }
        out.push_str("</section>");
    }
    out.push_str(&format!(
        "<footer>{}<br>{}</footer></main></html>",
        escape(duet_page::ATTRIBUTION),
        escape(duet_page::DISCLAIMER)
    ));
    out
}
