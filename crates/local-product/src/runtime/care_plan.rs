//! Durable care-plan recommendations and physician approval standing.
//!
//! Medication recommendations start strictly `Undecided` and stay outside the
//! patient's `active_plan_items` until an authorized panel physician explicitly
//! approves or declines them (`Rule.Authority.SilenceNeverApproves`, `L1.3`,
//! `L1.5`). Time passing, rounds finishing, and silence never promote an
//! `Undecided` recommendation into the active plan.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::concerns::WorkspaceAccess;
use super::{Authority, Store};

/// SeaORM table definition for `physician_runtime_care_plan_recommendations`.
pub(in crate::runtime) mod table {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_care_plan_recommendations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub recommendation_id: String,
        pub medication: String,
        pub rationale: String,
        pub detail: String,
        pub source_label: String,
        pub standing: String,
        pub proposed_at: i64,
        pub decided_at: Option<i64>,
        pub decided_by_physician: Option<String>,
        pub decision_note: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Wire / storage standing code for an undecided recommendation.
pub(in crate::runtime) const STANDING_UNDECIDED: &str = "undecided";
/// Wire / storage standing code for a physician-approved plan item.
pub(in crate::runtime) const STANDING_APPROVED_INTO_PLAN: &str = "approved_into_plan";
/// Wire / storage standing code for a physician-declined recommendation.
pub(in crate::runtime) const STANDING_DECLINED: &str = "declined";

/// Current standing of a care-plan medication recommendation.
///
/// Only explicit physician action moves a recommendation out of [`Self::Undecided`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum RecommendationStanding {
    /// Proposed and waiting for explicit physician decision; never part of the active plan.
    Undecided,
    /// Explicitly approved by an authorized panel physician into the patient's active plan.
    ApprovedIntoPlan {
        /// Physician who explicitly approved the recommendation.
        physician_id: String,
        /// Practice-clock Unix second when the physician approved it.
        approved_at: i64,
        /// Optional physician note captured with the approval.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
    /// Explicitly declined by an authorized panel physician; never enters the active plan.
    Declined {
        /// Physician who explicitly declined the recommendation.
        physician_id: String,
        /// Practice-clock Unix second when the physician declined it.
        declined_at: i64,
        /// Optional physician note captured with the decline.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
}

/// Explicit physician decision applied to an undecided recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum RecommendationDecision {
    /// Promote the recommendation into the patient's active care plan.
    ApproveIntoPlan {
        /// Optional physician note captured with the approval.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
    /// Decline the recommendation so it stays out of the active care plan.
    Decline {
        /// Optional physician note captured with the decline.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
}

/// Input required to propose a new `Undecided` medication recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewMedicationRecommendation {
    /// Stable recommendation identifier within `(practice, workspace)`.
    pub recommendation_id: String,
    /// Medication name and dose (for example `"Lisinopril 20 mg daily"`).
    pub medication: String,
    /// Clinical rationale or supporting context.
    pub rationale: String,
    /// Clinical plan detail shown once approved into the active plan.
    pub detail: String,
    /// Label describing the originating note or review.
    pub source_label: String,
    /// Practice-clock Unix second when the recommendation was proposed.
    pub proposed_at: i64,
}

/// Persisted care-plan recommendation record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarePlanRecommendation {
    /// Practice owning the patient workspace.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Recommendation identifier.
    pub recommendation_id: String,
    /// Medication name and dose.
    pub medication: String,
    /// Clinical rationale or context.
    pub rationale: String,
    /// Clinical detail used when promoted into the active plan.
    pub detail: String,
    /// Source note or review label.
    pub source_label: String,
    /// Practice-clock Unix second when the recommendation was proposed.
    pub proposed_at: i64,
    /// Typed standing (`Undecided`, `ApprovedIntoPlan`, or `Declined`).
    pub standing: RecommendationStanding,
}

/// Partitioned workspace care-plan projection.
///
/// `undecided_recommendations` contains ONLY [`RecommendationStanding::Undecided`] items,
/// and `active_plan_items` contains ONLY [`RecommendationStanding::ApprovedIntoPlan`] items.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCarePlan {
    /// Items explicitly approved by a physician into the active care plan.
    pub active_plan_items: Vec<CarePlanRecommendation>,
    /// Recommendations waiting for explicit physician decision.
    pub undecided_recommendations: Vec<CarePlanRecommendation>,
    /// Recommendations explicitly declined by a physician.
    pub declined_recommendations: Vec<CarePlanRecommendation>,
}

/// Errors returned by care-plan recommendation operations.
#[derive(Debug, Error)]
pub enum CarePlanError {
    /// Caller supplied an empty or invalid field.
    #[error("invalid care-plan input: {0}")]
    InvalidInput(String),
    /// Caller is not authorized to mutate this patient workspace.
    #[error("care-plan access denied: {reason}")]
    AccessDenied {
        /// Authority check failure reason.
        reason: String,
    },
    /// Target recommendation does not exist in `(practice, workspace)`.
    #[error("recommendation `{recommendation_id}` not found in workspace `{workspace}`")]
    NotFound {
        /// Target workspace.
        workspace: String,
        /// Missing recommendation id.
        recommendation_id: String,
    },
    /// A recommendation with this id was already decided in a conflicting way.
    #[error("recommendation `{recommendation_id}` is already decided ({current_standing})")]
    AlreadyDecided {
        /// Recommendation id.
        recommendation_id: String,
        /// Existing standing label.
        current_standing: &'static str,
    },
    /// Stored row violated the typed standing invariant.
    #[error("corrupt care-plan recommendation row `{recommendation_id}`: {detail}")]
    CorruptRow {
        /// Recommendation id.
        recommendation_id: String,
        /// Invariant violation detail.
        detail: String,
    },
    /// Database error from PostgreSQL.
    #[error("care-plan database error")]
    Database(#[from] sea_orm::DbErr),
}

fn require_non_empty(field: &str, value: &str) -> Result<String, CarePlanError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(CarePlanError::InvalidInput(format!(
            "{field} must not be empty"
        )));
    }
    Ok(trimmed.to_owned())
}

fn from_model(row: table::Model) -> Result<CarePlanRecommendation, CarePlanError> {
    let standing = match row.standing.as_str() {
        STANDING_UNDECIDED => {
            if row.decided_at.is_some() || row.decided_by_physician.is_some() {
                return Err(CarePlanError::CorruptRow {
                    recommendation_id: row.recommendation_id,
                    detail: "undecided recommendation must not carry decision metadata".to_owned(),
                });
            }
            RecommendationStanding::Undecided
        }
        STANDING_APPROVED_INTO_PLAN => {
            let Some(approved_at) = row.decided_at else {
                return Err(CarePlanError::CorruptRow {
                    recommendation_id: row.recommendation_id,
                    detail: "approved recommendation missing decided_at".to_owned(),
                });
            };
            let Some(physician_id) = row.decided_by_physician.filter(|p| !p.trim().is_empty())
            else {
                return Err(CarePlanError::CorruptRow {
                    recommendation_id: row.recommendation_id,
                    detail: "approved recommendation missing decided_by_physician".to_owned(),
                });
            };
            RecommendationStanding::ApprovedIntoPlan {
                physician_id,
                approved_at,
                note: row.decision_note,
            }
        }
        STANDING_DECLINED => {
            let Some(declined_at) = row.decided_at else {
                return Err(CarePlanError::CorruptRow {
                    recommendation_id: row.recommendation_id,
                    detail: "declined recommendation missing decided_at".to_owned(),
                });
            };
            let Some(physician_id) = row.decided_by_physician.filter(|p| !p.trim().is_empty())
            else {
                return Err(CarePlanError::CorruptRow {
                    recommendation_id: row.recommendation_id,
                    detail: "declined recommendation missing decided_by_physician".to_owned(),
                });
            };
            RecommendationStanding::Declined {
                physician_id,
                declined_at,
                note: row.decision_note,
            }
        }
        other => {
            return Err(CarePlanError::CorruptRow {
                recommendation_id: row.recommendation_id,
                detail: format!("unknown standing `{other}`"),
            });
        }
    };
    Ok(CarePlanRecommendation {
        practice: row.practice,
        workspace: row.workspace,
        recommendation_id: row.recommendation_id,
        medication: row.medication,
        rationale: row.rationale,
        detail: row.detail,
        source_label: row.source_label,
        proposed_at: row.proposed_at,
        standing,
    })
}

impl Store {
    /// Records a new medication recommendation in [`RecommendationStanding::Undecided`].
    ///
    /// Proposals NEVER enter the active plan automatically; an identical existing
    /// proposal is returned unchanged, and a proposal never overwrites or promotes
    /// an existing recommendation.
    pub async fn propose_medication_recommendation(
        &self,
        practice: &str,
        workspace: &str,
        proposal: NewMedicationRecommendation,
    ) -> Result<CarePlanRecommendation, CarePlanError> {
        let practice = require_non_empty("practice", practice)?;
        let workspace = require_non_empty("workspace", workspace)?;
        let recommendation_id =
            require_non_empty("recommendation_id", &proposal.recommendation_id)?;
        let medication = require_non_empty("medication", &proposal.medication)?;
        let rationale = require_non_empty("rationale", &proposal.rationale)?;
        let detail = if proposal.detail.trim().is_empty() {
            rationale.clone()
        } else {
            proposal.detail.trim().to_owned()
        };
        let source_label = if proposal.source_label.trim().is_empty() {
            "Patient review".to_owned()
        } else {
            proposal.source_label.trim().to_owned()
        };

        let tx = self.db.begin().await?;
        if let Some(existing) = table::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            recommendation_id.clone(),
        ))
        .one(&tx)
        .await?
        {
            tx.commit().await?;
            return from_model(existing);
        }

        let active = table::ActiveModel {
            practice: Set(practice),
            workspace: Set(workspace),
            recommendation_id: Set(recommendation_id),
            medication: Set(medication),
            rationale: Set(rationale),
            detail: Set(detail),
            source_label: Set(source_label),
            standing: Set(STANDING_UNDECIDED.to_owned()),
            proposed_at: Set(proposal.proposed_at),
            decided_at: Set(None),
            decided_by_physician: Set(None),
            decision_note: Set(None),
        };
        let inserted = active.insert(&tx).await?;
        tx.commit().await?;
        from_model(inserted)
    }

    /// Applies an explicit physician decision to an undecided medication
    /// recommendation after verifying and locking publication authority.
    pub async fn decide_medication_recommendation_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        recommendation_id: &str,
        decision: RecommendationDecision,
        decided_at: i64,
        authority: &mut dyn Authority,
    ) -> Result<CarePlanRecommendation, CarePlanError> {
        let scope = access
            .authority_scope()
            .map_err(|err| CarePlanError::InvalidInput(err.to_string()))?;
        let recommendation_id = require_non_empty("recommendation_id", recommendation_id)?;
        let physician_id = require_non_empty("physician", &scope.physician)?;

        let tx = self.db.begin().await?;
        if let Err(reason) = authority
            .lock_publication(&tx, &scope, Some(access.session_hash))
            .await
        {
            tx.rollback().await?;
            return Err(CarePlanError::AccessDenied { reason });
        }

        let Some(existing) = table::Entity::find_by_id((
            scope.practice.clone(),
            scope.workspace.clone(),
            recommendation_id.clone(),
        ))
        .one(&tx)
        .await?
        else {
            tx.rollback().await?;
            return Err(CarePlanError::NotFound {
                workspace: scope.workspace,
                recommendation_id,
            });
        };

        let (target_standing, target_note) = match decision {
            RecommendationDecision::ApproveIntoPlan { note } => (
                STANDING_APPROVED_INTO_PLAN,
                note.map(|n| n.trim().to_owned()).filter(|n| !n.is_empty()),
            ),
            RecommendationDecision::Decline { note } => (
                STANDING_DECLINED,
                note.map(|n| n.trim().to_owned()).filter(|n| !n.is_empty()),
            ),
        };

        if existing.standing == target_standing {
            tx.commit().await?;
            return from_model(existing);
        }
        if existing.standing != STANDING_UNDECIDED {
            let current_standing = match existing.standing.as_str() {
                STANDING_APPROVED_INTO_PLAN => STANDING_APPROVED_INTO_PLAN,
                STANDING_DECLINED => STANDING_DECLINED,
                _ => "decided",
            };
            tx.rollback().await?;
            return Err(CarePlanError::AlreadyDecided {
                recommendation_id,
                current_standing,
            });
        }

        let mut active: table::ActiveModel = existing.into();
        active.standing = Set(target_standing.to_owned());
        active.decided_at = Set(Some(decided_at));
        active.decided_by_physician = Set(Some(physician_id));
        active.decision_note = Set(target_note);
        let updated = active.update(&tx).await?;
        tx.commit().await?;
        from_model(updated)
    }

    /// Reads the patient workspace's care plan partitioned into physician-approved
    /// active plan items, undecided recommendations, and declined recommendations.
    pub async fn workspace_care_plan(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<WorkspaceCarePlan, CarePlanError> {
        let practice = require_non_empty("practice", practice)?;
        let workspace = require_non_empty("workspace", workspace)?;
        let rows = table::Entity::find()
            .filter(
                Condition::all()
                    .add(table::Column::Practice.eq(practice))
                    .add(table::Column::Workspace.eq(workspace)),
            )
            .order_by_asc(table::Column::ProposedAt)
            .order_by_asc(table::Column::RecommendationId)
            .all(&self.db)
            .await?;

        let mut plan = WorkspaceCarePlan::default();
        for row in rows {
            let rec = from_model(row)?;
            match &rec.standing {
                RecommendationStanding::Undecided => plan.undecided_recommendations.push(rec),
                RecommendationStanding::ApprovedIntoPlan { .. } => plan.active_plan_items.push(rec),
                RecommendationStanding::Declined { .. } => plan.declined_recommendations.push(rec),
            }
        }
        Ok(plan)
    }

    /// Lists all care-plan recommendations across `practice` for L1 measure accounting.
    pub async fn practice_care_plan_recommendations(
        &self,
        practice: &str,
    ) -> Result<Vec<CarePlanRecommendation>, CarePlanError> {
        let practice = require_non_empty("practice", practice)?;
        let rows = table::Entity::find()
            .filter(table::Column::Practice.eq(practice))
            .order_by_asc(table::Column::ProposedAt)
            .order_by_asc(table::Column::RecommendationId)
            .all(&self.db)
            .await?;
        rows.into_iter().map(from_model).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{PublicationTransaction, Scope};

    struct AllowPanelAuthority {
        allowed_physician: String,
    }

    #[async_trait::async_trait]
    impl Authority for AllowPanelAuthority {
        async fn check(
            &mut self,
            _tx: &PublicationTransaction,
            scope: &Scope,
            _session_hash: Option<&[u8]>,
        ) -> Result<(), String> {
            if scope.physician == self.allowed_physician {
                Ok(())
            } else {
                Err("physician not on patient panel".to_owned())
            }
        }

        async fn lock_publication(
            &mut self,
            tx: &PublicationTransaction,
            scope: &Scope,
            session_hash: Option<&[u8]>,
        ) -> Result<(), String> {
            self.check(tx, scope, session_hash).await
        }
    }

    #[tokio::test]
    #[ignore = "requires explicit local Postgres"]
    async fn silence_never_approves_undecided_medication_recommendations() {
        let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .expect("PHYSICIAN_RUNTIME_TEST_DATABASE_URL");
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();

        let suffix = uuid::Uuid::new_v4().simple().to_string();
        let practice = format!("care-plan-practice-{suffix}");
        let workspace = format!("pw-care-plan-{suffix}");
        let rec_id = format!("rec-lisinopril-{suffix}");

        let proposed = store
            .propose_medication_recommendation(
                &practice,
                &workspace,
                NewMedicationRecommendation {
                    recommendation_id: rec_id.clone(),
                    medication: "Lisinopril 20 mg PO daily".to_owned(),
                    rationale: "Home BP 148/92 over two weeks".to_owned(),
                    detail:
                        "Increase lisinopril from 10 mg to 20 mg daily; recheck BMP in 2 weeks."
                            .to_owned(),
                    source_label: "Hypertension review".to_owned(),
                    proposed_at: 1_787_846_400,
                },
            )
            .await
            .unwrap();
        assert_eq!(proposed.standing, RecommendationStanding::Undecided);

        // Before physician decision, it stays strictly in undecided_recommendations
        // and NEVER appears in active_plan_items.
        let before = store
            .workspace_care_plan(&practice, &workspace)
            .await
            .unwrap();
        assert!(before.active_plan_items.is_empty());
        assert_eq!(before.undecided_recommendations.len(), 1);
        assert_eq!(
            before.undecided_recommendations[0].recommendation_id,
            rec_id
        );

        // Unauthorized physician is refused and recommendation remains Undecided.
        let mut authority = AllowPanelAuthority {
            allowed_physician: "dr-panel".to_owned(),
        };
        let session = [7_u8; 32];
        let intruder = WorkspaceAccess {
            practice: &practice,
            physician: "dr-intruder",
            workspace: &workspace,
            session_hash: &session,
        };
        let denied = store
            .decide_medication_recommendation_for_physician(
                &intruder,
                &rec_id,
                RecommendationDecision::ApproveIntoPlan { note: None },
                1_788_451_200,
                &mut authority,
            )
            .await;
        assert!(matches!(denied, Err(CarePlanError::AccessDenied { .. })));

        // Re-proposing the same recommendation never promotes it.
        let still_undecided = store
            .propose_medication_recommendation(
                &practice,
                &workspace,
                NewMedicationRecommendation {
                    recommendation_id: rec_id.clone(),
                    medication: "Lisinopril 20 mg PO daily".to_owned(),
                    rationale: "Home BP 148/92 over two weeks".to_owned(),
                    detail: "Increase lisinopril".to_owned(),
                    source_label: "Hypertension review".to_owned(),
                    proposed_at: 1_788_451_200,
                },
            )
            .await
            .unwrap();
        assert_eq!(still_undecided.standing, RecommendationStanding::Undecided);

        // Authorized panel physician explicitly approves -> moves into active_plan_items.
        let doctor = WorkspaceAccess {
            practice: &practice,
            physician: "dr-panel",
            workspace: &workspace,
            session_hash: &session,
        };
        let approved = store
            .decide_medication_recommendation_for_physician(
                &doctor,
                &rec_id,
                RecommendationDecision::ApproveIntoPlan {
                    note: Some("Discussed with patient".to_owned()),
                },
                1_788_451_200,
                &mut authority,
            )
            .await
            .unwrap();
        assert_eq!(
            approved.standing,
            RecommendationStanding::ApprovedIntoPlan {
                physician_id: "dr-panel".to_owned(),
                approved_at: 1_788_451_200,
                note: Some("Discussed with patient".to_owned()),
            }
        );

        let after = store
            .workspace_care_plan(&practice, &workspace)
            .await
            .unwrap();
        assert!(after.undecided_recommendations.is_empty());
        assert_eq!(after.active_plan_items.len(), 1);
        assert_eq!(after.active_plan_items[0].recommendation_id, rec_id);

        // Conflicting decline after approval is refused.
        let conflict = store
            .decide_medication_recommendation_for_physician(
                &doctor,
                &rec_id,
                RecommendationDecision::Decline { note: None },
                1_788_451_300,
                &mut authority,
            )
            .await;
        assert!(matches!(
            conflict,
            Err(CarePlanError::AlreadyDecided { .. })
        ));
    }
}
