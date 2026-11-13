//! Typed client calls for the patient-page `Follow-ups` section:
//! list, create, edit, `Review now`, `Stop`, `Stop and discuss`, `Resume`,
//! `Complete`, `Cancel`, and `"Can't restart"` resolution.

use physician_wire::follow_ups::{
    CannotRestartResolutionWire, CreateFollowUpBody, FollowUpActionCommandBody,
    FollowUpMutationResponseWire, PatientFollowUpsResponseWire, ResolveCannotRestartBody,
    SettleFollowUpBody, StopFollowUpBody, UpdateFollowUpBody,
};

use crate::{HttpError, PhysicianClient, path_escape};

impl PhysicianClient {
    /// Lists all follow-ups for an authorized patient workspace (`GET /patient-workspaces/{id}/follow-ups`).
    pub async fn patient_follow_ups(
        &self,
        patient_workspace_id: &str,
    ) -> Result<PatientFollowUpsResponseWire, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/follow-ups",
            path_escape(patient_workspace_id)
        ))
        .await
    }

    /// Creates a new scheduled follow-up on an authorized patient workspace.
    pub async fn create_follow_up(
        &self,
        patient_workspace_id: &str,
        summary: &str,
        due_date: &str,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = CreateFollowUpBody {
            summary: summary.to_owned(),
            due_date: due_date.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups",
                path_escape(patient_workspace_id)
            ),
            &body,
        )
        .await
    }

    /// Edits an open follow-up's summary, due date, and optional single-line note.
    pub async fn update_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        body: &UpdateFollowUpBody,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        self.put(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            body,
        )
        .await
    }

    /// Marks an open follow-up `Review now` (`DueNow`) and triggers a patient chart review.
    pub async fn review_follow_up_now(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = FollowUpActionCommandBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/review-now",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }

    /// Holds a follow-up (`Stop`) so it runs no scheduled reviews until `Resume`.
    pub async fn stop_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        reason: Option<String>,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = StopFollowUpBody {
            reason,
            physician_time_zone: None,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/stop",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }

    /// Holds a follow-up (`Stop and discuss`) and seeds a durable patient conversation
    /// with that follow-up's context while leaving the follow-up `Held` until `Resume`.
    pub async fn stop_and_discuss_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        reason: Option<String>,
        physician_time_zone: &str,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = StopFollowUpBody {
            reason,
            physician_time_zone: Some(physician_time_zone.to_owned()),
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/stop-and-discuss",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }

    /// Resumes a `Held` follow-up to `Scheduled`.
    pub async fn resume_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = FollowUpActionCommandBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/resume",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }

    /// Completes an open follow-up with a single-line resolution note.
    pub async fn complete_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        resolution: &str,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = SettleFollowUpBody {
            note: resolution.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/complete",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }

    /// Cancels an open follow-up with a single-line reason.
    pub async fn cancel_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        reason: &str,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = SettleFollowUpBody {
            note: reason.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/cancel",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }

    /// Resolves a `"Can't restart"` follow-up via `Complete`, `New date`, or `Cancel`.
    pub async fn resolve_cannot_restart_follow_up(
        &self,
        patient_workspace_id: &str,
        follow_up_id: &str,
        resolution: CannotRestartResolutionWire,
        idempotency_key: &str,
    ) -> Result<FollowUpMutationResponseWire, HttpError> {
        let body = ResolveCannotRestartBody {
            resolution,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/follow-ups/{}/resolve-cannot-restart",
                path_escape(patient_workspace_id),
                path_escape(follow_up_id)
            ),
            &body,
        )
        .await
    }
}
