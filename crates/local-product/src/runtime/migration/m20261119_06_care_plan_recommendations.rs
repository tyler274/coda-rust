//! Care-plan medication recommendations table (`physician_runtime_care_plan_recommendations`).

use super::super::care_plan::{
    STANDING_APPROVED_INTO_PLAN, STANDING_DECLINED, STANDING_UNDECIDED,
    table::{Column as C, Entity as CarePlanEntity},
};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_care_plan_recommendations` with database-level
/// checks ensuring undecided items carry no decision attribution and approved
/// or declined items always carry physician attribution and timestamp.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let standing_invariant = Cond::any()
            .add(
                Cond::all()
                    .add(Expr::col(C::Standing).eq(STANDING_UNDECIDED))
                    .add(Expr::col(C::DecidedAt).is_null())
                    .add(Expr::col(C::DecidedByPhysician).is_null()),
            )
            .add(
                Cond::all()
                    .add(
                        Expr::col(C::Standing)
                            .is_in([STANDING_APPROVED_INTO_PLAN, STANDING_DECLINED]),
                    )
                    .add(Expr::col(C::DecidedAt).is_not_null())
                    .add(Expr::col(C::DecidedByPhysician).is_not_null())
                    .add(Expr::col(C::DecidedByPhysician).ne("")),
            );

        manager
            .create_table(
                Table::create()
                    .table(CarePlanEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(C::Practice)
                            .text()
                            .not_null()
                            .check(Expr::col(C::Practice).ne("")),
                    )
                    .col(
                        ColumnDef::new(C::Workspace)
                            .text()
                            .not_null()
                            .check(Expr::col(C::Workspace).ne("")),
                    )
                    .col(
                        ColumnDef::new(C::RecommendationId)
                            .text()
                            .not_null()
                            .check(Expr::col(C::RecommendationId).ne("")),
                    )
                    .col(
                        ColumnDef::new(C::Medication)
                            .text()
                            .not_null()
                            .check(Expr::col(C::Medication).ne("")),
                    )
                    .col(ColumnDef::new(C::Rationale).text().not_null())
                    .col(ColumnDef::new(C::Detail).text().not_null())
                    .col(ColumnDef::new(C::SourceLabel).text().not_null())
                    .col(ColumnDef::new(C::Standing).text().not_null().check(
                        Expr::col(C::Standing).is_in([
                            STANDING_UNDECIDED,
                            STANDING_APPROVED_INTO_PLAN,
                            STANDING_DECLINED,
                        ]),
                    ))
                    .col(ColumnDef::new(C::ProposedAt).big_integer().not_null())
                    .col(ColumnDef::new(C::DecidedAt).big_integer().null())
                    .col(ColumnDef::new(C::DecidedByPhysician).text().null())
                    .col(ColumnDef::new(C::DecisionNote).text().null())
                    .check(standing_invariant)
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_care_plan_recommendations")
                            .col(C::Practice)
                            .col(C::Workspace)
                            .col(C::RecommendationId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-physician_runtime_care_plan_recommendations-ws_standing")
                    .table(CarePlanEntity)
                    .col(C::Practice)
                    .col(C::Workspace)
                    .col(C::Standing)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CarePlanEntity).if_exists().to_owned())
            .await
    }
}
