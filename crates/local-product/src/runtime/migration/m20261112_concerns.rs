//! Patient concerns raised by Coda for a workspace, and durable physician
//! feedback saved when a physician marks a concern `Done`.

use super::super::entities::{concern, concern_feedback};
use sea_orm_migration::prelude::*;

/// Creates `physician_runtime_concerns` and `physician_runtime_concern_feedback`.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let concern_key =
            |column: concern::Column| ColumnDef::new(column).text().not_null().to_owned();
        let open_standing = Condition::all()
            .add(Expr::col(concern::Column::Status).eq("open"))
            .add(Expr::col(concern::Column::ClearedByPhysician).is_null())
            .add(Expr::col(concern::Column::ClearedAt).is_null())
            .add(Expr::col(concern::Column::FeedbackRevision).is_null())
            .add(Expr::col(concern::Column::RetirementReason).is_null())
            .add(Expr::col(concern::Column::RetiredByWork).is_null())
            .add(Expr::col(concern::Column::RetiredAt).is_null());
        let done_standing = Condition::all()
            .add(Expr::col(concern::Column::Status).eq("done"))
            .add(Expr::col(concern::Column::ClearedByPhysician).is_not_null())
            .add(Expr::col(concern::Column::ClearedAt).is_not_null())
            .add(Expr::col(concern::Column::FeedbackRevision).is_not_null())
            .add(Expr::col(concern::Column::FeedbackRevision).gt(0))
            .add(Expr::col(concern::Column::RetirementReason).is_null())
            .add(Expr::col(concern::Column::RetiredByWork).is_null())
            .add(Expr::col(concern::Column::RetiredAt).is_null());
        let retired_standing = Condition::all()
            .add(Expr::col(concern::Column::Status).eq("retired"))
            .add(Expr::col(concern::Column::RetirementReason).is_not_null())
            .add(Expr::col(concern::Column::RetiredByWork).is_not_null())
            .add(Expr::col(concern::Column::RetiredAt).is_not_null())
            .add(Expr::col(concern::Column::ClearedByPhysician).is_null())
            .add(Expr::col(concern::Column::ClearedAt).is_null())
            .add(Expr::col(concern::Column::FeedbackRevision).is_null());

        manager
            .create_table(
                Table::create()
                    .table(concern::Entity)
                    .col(concern_key(concern::Column::Practice))
                    .col(concern_key(concern::Column::Workspace))
                    .col(concern_key(concern::Column::Id))
                    .col(
                        ColumnDef::new(concern::Column::Urgency)
                            .text()
                            .not_null()
                            .check(Expr::col(concern::Column::Urgency).is_in(["now", "today"])),
                    )
                    .col(
                        ColumnDef::new(concern::Column::Status)
                            .text()
                            .not_null()
                            .check(
                                Expr::col(concern::Column::Status)
                                    .is_in(["open", "done", "retired"]),
                            ),
                    )
                    .col(ColumnDef::new(concern::Column::Reason).text().not_null())
                    .col(
                        ColumnDef::new(concern::Column::Records)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern::Column::SourceWork)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(concern::Column::ClearedByPhysician).text())
                    .col(ColumnDef::new(concern::Column::ClearedAt).big_integer())
                    .col(ColumnDef::new(concern::Column::FeedbackRevision).big_integer())
                    .col(ColumnDef::new(concern::Column::RetirementReason).text())
                    .col(ColumnDef::new(concern::Column::RetiredByWork).json_binary())
                    .col(ColumnDef::new(concern::Column::RetiredAt).big_integer())
                    .col(
                        ColumnDef::new(concern::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern::Column::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(concern::Column::Revision).gt(0)),
                    )
                    .check(
                        Condition::any()
                            .add(open_standing)
                            .add(done_standing)
                            .add(retired_standing),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_concerns")
                            .col(concern::Column::Practice)
                            .col(concern::Column::Workspace)
                            .col(concern::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_concerns-workspace_status")
                    .table(concern::Entity)
                    .col(concern::Column::Practice)
                    .col(concern::Column::Workspace)
                    .col(concern::Column::Status)
                    .to_owned(),
            )
            .await?;

        let feedback_key =
            |column: concern_feedback::Column| ColumnDef::new(column).text().not_null().to_owned();
        manager
            .create_table(
                Table::create()
                    .table(concern_feedback::Entity)
                    .col(feedback_key(concern_feedback::Column::Practice))
                    .col(feedback_key(concern_feedback::Column::Workspace))
                    .col(feedback_key(concern_feedback::Column::Concern))
                    .col(feedback_key(concern_feedback::Column::Physician))
                    .col(
                        ColumnDef::new(concern_feedback::Column::Action)
                            .text()
                            .not_null()
                            .check(Expr::col(concern_feedback::Column::Action).is_in(["done"])),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::ConcernRevision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(concern_feedback::Column::ConcernRevision).gt(0)),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::Urgency)
                            .text()
                            .not_null()
                            .check(
                                Expr::col(concern_feedback::Column::Urgency)
                                    .is_in(["now", "today"]),
                            ),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::ConcernReason)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::Records)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::SourceWork)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::SavedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(concern_feedback::Column::Revision)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(concern_feedback::Column::Revision).gt(0)),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_concern_feedback")
                            .col(concern_feedback::Column::Practice)
                            .col(concern_feedback::Column::Workspace)
                            .col(concern_feedback::Column::Concern),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Custom(
            "patient concerns and physician feedback are clinical working records and can't be rolled back"
                .into(),
        ))
    }
}
