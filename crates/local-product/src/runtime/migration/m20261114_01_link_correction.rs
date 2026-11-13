//! Wrong-patient conversation link correction state, multi-step settlement
//! progress, and per-workspace withholding index tables.

use sea_orm_migration::prelude::*;

/// Creates wrong-patient link correction and workspace withholding tables.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[derive(DeriveIden)]
enum LinkCorrectionTable {
    #[sea_orm(iden = "physician_runtime_link_corrections")]
    Table,
    Practice,
    #[sea_orm(iden = "correction_id")]
    CorrectionId,
    #[sea_orm(iden = "conversation_id")]
    ConversationId,
    #[sea_orm(iden = "correction_epoch")]
    CorrectionEpoch,
    #[sea_orm(iden = "move_ordinal")]
    MoveOrdinal,
    #[sea_orm(iden = "original_workspace")]
    OriginalWorkspace,
    #[sea_orm(iden = "intermediate_workspaces")]
    IntermediateWorkspaces,
    Destination,
    #[sea_orm(iden = "standing_kind")]
    StandingKind,
    #[sea_orm(iden = "standing_detail")]
    StandingDetail,
    #[sea_orm(iden = "source_candidate_ids")]
    SourceCandidateIds,
    #[sea_orm(iden = "withheld_work_ids")]
    WithheldWorkIds,
    #[sea_orm(iden = "withheld_reply_ids")]
    WithheldReplyIds,
    #[sea_orm(iden = "withheld_draft_keys")]
    WithheldDraftKeys,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum LinkCorrectionWorkspaceTable {
    #[sea_orm(iden = "physician_runtime_link_correction_workspaces")]
    Table,
    Workspace,
    Practice,
    #[sea_orm(iden = "correction_id")]
    CorrectionId,
    #[sea_orm(iden = "conversation_id")]
    ConversationId,
    #[sea_orm(iden = "active_withholding")]
    ActiveWithholding,
    #[sea_orm(iden = "withheld_standing")]
    WithheldStanding,
    #[sea_orm(iden = "message_ids")]
    MessageIds,
    #[sea_orm(iden = "candidate_ids")]
    CandidateIds,
    #[sea_orm(iden = "work_ids")]
    WorkIds,
    #[sea_orm(iden = "reply_ids")]
    ReplyIds,
    #[sea_orm(iden = "draft_keys")]
    DraftKeys,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LinkCorrectionTable::Table)
                    .col(
                        ColumnDef::new(LinkCorrectionTable::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::CorrectionId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::ConversationId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::CorrectionEpoch)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(LinkCorrectionTable::CorrectionEpoch).gte(1)),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::MoveOrdinal)
                            .big_integer()
                            .not_null()
                            .check(Expr::col(LinkCorrectionTable::MoveOrdinal).gte(1)),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::OriginalWorkspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::IntermediateWorkspaces)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::Destination)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::StandingKind)
                            .text()
                            .not_null()
                            .check(Expr::col(LinkCorrectionTable::StandingKind).is_in([
                                "undo_window",
                                "undone",
                                "cancelled_returned_to_origin",
                                "settling_failed",
                                "settled",
                            ])),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::StandingDetail)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::SourceCandidateIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::WithheldWorkIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::WithheldReplyIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::WithheldDraftKeys)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionTable::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_link_corrections")
                            .col(LinkCorrectionTable::Practice)
                            .col(LinkCorrectionTable::CorrectionId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_link_corrections-conv_epoch")
                    .table(LinkCorrectionTable::Table)
                    .col(LinkCorrectionTable::Practice)
                    .col(LinkCorrectionTable::ConversationId)
                    .col(LinkCorrectionTable::CorrectionEpoch)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_link_corrections-standing")
                    .table(LinkCorrectionTable::Table)
                    .col(LinkCorrectionTable::Practice)
                    .col(LinkCorrectionTable::StandingKind)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LinkCorrectionWorkspaceTable::Table)
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::Workspace)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::Practice)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::CorrectionId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::ConversationId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::ActiveWithholding)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::WithheldStanding)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::MessageIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::CandidateIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::WorkIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::ReplyIds)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::DraftKeys)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LinkCorrectionWorkspaceTable::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-physician_runtime_link_correction_workspaces")
                            .col(LinkCorrectionWorkspaceTable::Workspace)
                            .col(LinkCorrectionWorkspaceTable::Practice)
                            .col(LinkCorrectionWorkspaceTable::CorrectionId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_link_correction_ws-active")
                    .table(LinkCorrectionWorkspaceTable::Table)
                    .col(LinkCorrectionWorkspaceTable::Workspace)
                    .col(LinkCorrectionWorkspaceTable::ActiveWithholding)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-physician_runtime_link_correction_ws-conv")
                    .table(LinkCorrectionWorkspaceTable::Table)
                    .col(LinkCorrectionWorkspaceTable::Practice)
                    .col(LinkCorrectionWorkspaceTable::Workspace)
                    .col(LinkCorrectionWorkspaceTable::ConversationId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(LinkCorrectionWorkspaceTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(LinkCorrectionTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
