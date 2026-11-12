//! Typed client calls for opening Today/Updates items (marking per-physician
//! seen state and retrieving verified chart evidence) and clearing Today items
//! with `Done`.

use physician_wire::today_updates::{
    ItemWorkEvidenceWire, MarkTodayDoneBody, OpenTodayUpdateItemBody, TodayDoneResponseWire,
};

use crate::{HttpError, PhysicianClient};

impl PhysicianClient {
    /// Opens a Today row or Updates item for the signed-in physician, durably
    /// recording seen state on the server and returning the item's work and
    /// verified chart citations.
    pub async fn open_today_update_item(
        &self,
        patient_workspace_id: &str,
        item_id: &str,
        idempotency_key: &str,
    ) -> Result<ItemWorkEvidenceWire, HttpError> {
        let body = OpenTodayUpdateItemBody {
            patient_workspace_id: patient_workspace_id.to_owned(),
            item_id: item_id.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post("/today-updates/open", &body).await
    }

    /// Clears a concern or non-restartable unhandled Today item with `Done`.
    pub async fn mark_today_item_done(
        &self,
        patient_workspace_id: &str,
        item_id: &str,
        version_id: &str,
        idempotency_key: &str,
    ) -> Result<TodayDoneResponseWire, HttpError> {
        let body = MarkTodayDoneBody {
            patient_workspace_id: patient_workspace_id.to_owned(),
            item_id: item_id.to_owned(),
            version_id: version_id.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post("/today-updates/done", &body).await
    }
}
