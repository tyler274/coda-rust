//! Client methods for reading saved patient reviews and sending control commands.

use physician_wire::{
    PatientReviewWire, PatientReviewsResponse, ReviewControlRequest, ReviewControlResponse,
};

use crate::{HttpError, PhysicianClient, query_escape};

impl PhysicianClient {
    /// Lists saved patient reviews for `workspace_id`, newest update first.
    pub async fn patient_reviews(
        &self,
        workspace_id: &str,
    ) -> Result<PatientReviewsResponse, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/reviews",
            query_escape(workspace_id)
        ))
        .await
    }

    /// Reads one saved patient review in `workspace_id`.
    pub async fn patient_review(
        &self,
        workspace_id: &str,
        review_id: &str,
    ) -> Result<PatientReviewWire, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/reviews/{}",
            query_escape(workspace_id),
            query_escape(review_id)
        ))
        .await
    }

    /// Sends a versioned `Stop`, `Resume`, or `Try again` command for one patient review.
    ///
    /// Validated `400`, `409`, and `422` refusal bodies decode into
    /// [`ReviewControlResponse::Refused`] so the caller receives both the server's
    /// refusal reason and any current review state; `401`, `403`, `404`, `5xx`, and
    /// transport failures remain [`HttpError`] values.
    pub async fn control_patient_review(
        &self,
        workspace_id: &str,
        request: &ReviewControlRequest,
    ) -> Result<ReviewControlResponse, HttpError> {
        let path = format!(
            "/patient-workspaces/{}/reviews/{}/control",
            query_escape(workspace_id),
            query_escape(&request.review_id)
        );
        match self.post::<ReviewControlResponse>(&path, request).await {
            Ok(answer) => Ok(answer),
            Err(HttpError::Status {
                status: status @ (400 | 409 | 422),
                body,
            }) => match serde_json::from_str::<ReviewControlResponse>(&body) {
                Ok(refused @ ReviewControlResponse::Refused { .. }) => Ok(refused),
                _ => Err(HttpError::Status { status, body }),
            },
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::Json;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::routing::{get, post};
    use physician_wire::{
        PatientReviewStateWire, ReviewControlActionWire, ReviewControlRefusalReason, ReviewVersion,
    };

    use super::*;

    #[tokio::test]
    async fn control_patient_review_sends_id_and_version_and_decodes_409_refusal() {
        let app = axum::Router::new()
            .route(
                "/patient-workspaces/{ws}/reviews",
                get(|| async {
                    Json(PatientReviewsResponse {
                        reviews: Vec::new(),
                    })
                }),
            )
            .route(
                "/patient-workspaces/{ws}/reviews/{rev}/control",
                post(
                    |Path((ws, rev)): Path<(String, String)>,
                     Json(body): Json<ReviewControlRequest>| async move {
                        assert_eq!(ws, "pw-1");
                        assert_eq!(rev, "rev-1");
                        assert_eq!(body.review_id, "rev-1");
                        if body.version == ReviewVersion("rv-stale".into()) {
                            (
                                StatusCode::CONFLICT,
                                Json(ReviewControlResponse::Refused {
                                    reason: ReviewControlRefusalReason::StaleVersion,
                                    message: "Review version changed.".into(),
                                    review: None,
                                }),
                            )
                        } else {
                            (
                                StatusCode::OK,
                                Json(ReviewControlResponse::Accepted {
                                    review: Box::new(PatientReviewWire {
                                        review_id: "rev-1".into(),
                                        version: ReviewVersion("rv-2".into()),
                                        work_id: "work-1".into(),
                                        triggers: Vec::new(),
                                        accepted_inputs: Vec::new(),
                                        state: PatientReviewStateWire::Pending,
                                        steps: Vec::new(),
                                        evidence: Vec::new(),
                                        sources: Default::default(),
                                        actions: Vec::new(),
                                        unfinished_work: Vec::new(),
                                        queued_at: "2026-08-27T12:00:00Z".into(),
                                        updated_at: "2026-08-27T12:00:01Z".into(),
                                    }),
                                }),
                            )
                        }
                    },
                ),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let client = PhysicianClient::new(format!("http://{addr}"));
        let list = client.patient_reviews("pw-1").await.unwrap();
        assert!(list.reviews.is_empty());

        let stale = client
            .control_patient_review(
                "pw-1",
                &ReviewControlRequest {
                    review_id: "rev-1".into(),
                    version: ReviewVersion("rv-stale".into()),
                    action: ReviewControlActionWire::Stop,
                    idempotency_key: "k-1".into(),
                },
            )
            .await
            .unwrap();
        assert!(matches!(
            stale,
            ReviewControlResponse::Refused {
                reason: ReviewControlRefusalReason::StaleVersion,
                ..
            }
        ));

        let ok = client
            .control_patient_review(
                "pw-1",
                &ReviewControlRequest {
                    review_id: "rev-1".into(),
                    version: ReviewVersion("rv-1".into()),
                    action: ReviewControlActionWire::Stop,
                    idempotency_key: "k-2".into(),
                },
            )
            .await
            .unwrap();
        assert!(matches!(ok, ReviewControlResponse::Accepted { .. }));
    }
}
