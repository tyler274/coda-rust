//! Reply conversations, proposed replies, review completion receipts,
//! physician draft bindings, and copy attribution records.

use super::super::reply::proposals::tables::{
    reply_conversation, reply_copy, reply_draft_binding, reply_proposal, reply_review_completion,
};
use sea_orm_migration::prelude::*;

/// Creates the reply proposal and physician draft coordination tables.
#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_reply_conversations(manager).await?;
        create_reply_proposals(manager).await?;
        create_reply_review_completions(manager).await?;
        create_reply_draft_bindings(manager).await?;
        create_reply_copies(manager).await?;
        Ok(())
    }
}

async fn create_reply_conversations(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use reply_conversation::Column as C;

    let non_empty_text = |column: C| {
        ColumnDef::new(column)
            .text()
            .not_null()
            .check(Expr::col(column).ne(""))
            .to_owned()
    };

    let linked_patient_invariant = Condition::all()
        .add(Expr::col(C::LinkKind).eq("linked_patient"))
        .add(Expr::col(C::Workspace).is_not_null())
        .add(Expr::col(C::Workspace).ne(""));
    let not_patient_invariant = Condition::all()
        .add(Expr::col(C::LinkKind).eq("not_a_patient"))
        .add(Expr::col(C::Workspace).is_null())
        .add(Expr::col(C::ReplyState).eq("none"));

    manager
        .create_table(
            Table::create()
                .table(reply_conversation::Entity)
                .col(non_empty_text(C::Practice))
                .col(non_empty_text(C::ConversationId))
                .col(
                    ColumnDef::new(C::LinkKind)
                        .text()
                        .not_null()
                        .check(Expr::col(C::LinkKind).is_in(["linked_patient", "not_a_patient"])),
                )
                .col(ColumnDef::new(C::Workspace).text())
                .col(
                    ColumnDef::new(C::ChannelState)
                        .text()
                        .not_null()
                        .check(Expr::col(C::ChannelState).is_in(["connected", "disconnected"])),
                )
                .col(ColumnDef::new(C::ReplyState).text().not_null().check(
                    Expr::col(C::ReplyState).is_in([
                        "none",
                        "ready",
                        "withheld",
                        "closed_with_undo",
                    ]),
                ))
                .col(
                    ColumnDef::new(C::ConversationVersion)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::ConversationVersion).gte(1)),
                )
                .col(
                    ColumnDef::new(C::MoveEpoch)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::MoveEpoch).gte(0)),
                )
                .col(
                    ColumnDef::new(C::LastReplyId)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::LastReplyId).gte(0)),
                )
                .col(
                    ColumnDef::new(C::LastProposalVersion)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::LastProposalVersion).gte(0)),
                )
                .col(
                    ColumnDef::new(C::LastPracticeObservationSeq)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::LastPracticeObservationSeq).gte(0)),
                )
                .col(ColumnDef::new(C::State).json_binary().not_null())
                .col(ColumnDef::new(C::CreatedAt).big_integer().not_null())
                .col(ColumnDef::new(C::UpdatedAt).big_integer().not_null())
                .check(
                    Condition::any()
                        .add(linked_patient_invariant)
                        .add(not_patient_invariant),
                )
                .primary_key(
                    Index::create()
                        .name("pk-physician_runtime_reply_conversations")
                        .col(C::Practice)
                        .col(C::ConversationId),
                )
                .to_owned(),
        )
        .await?;

    manager
        .create_index(
            Index::create()
                .name("idx-physician_runtime_reply_conversations-workspace")
                .table(reply_conversation::Entity)
                .col(C::Practice)
                .col(C::Workspace)
                .col(C::UpdatedAt)
                .to_owned(),
        )
        .await
}

async fn create_reply_proposals(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use reply_proposal::Column as C;

    let non_empty_text = |column: C| {
        ColumnDef::new(column)
            .text()
            .not_null()
            .check(Expr::col(column).ne(""))
            .to_owned()
    };

    manager
        .create_table(
            Table::create()
                .table(reply_proposal::Entity)
                .col(non_empty_text(C::Practice))
                .col(non_empty_text(C::ConversationId))
                .col(
                    ColumnDef::new(C::Version)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::Version).gte(1)),
                )
                .col(
                    ColumnDef::new(C::ReplyId)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::ReplyId).gte(1)),
                )
                .col(non_empty_text(C::Workspace))
                .col(non_empty_text(C::ReviewId))
                .col(non_empty_text(C::RunId))
                .col(non_empty_text(C::CompletionKey))
                .col(
                    ColumnDef::new(C::ConversationVersion)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::ConversationVersion).gte(1)),
                )
                .col(
                    ColumnDef::new(C::SourceVersion)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::SourceVersion).gte(0)),
                )
                .col(
                    ColumnDef::new(C::MoveEpoch)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::MoveEpoch).gte(0)),
                )
                .col(
                    ColumnDef::new(C::ReplacedProposalVersion)
                        .big_integer()
                        .check(
                            Expr::col(C::ReplacedProposalVersion)
                                .is_null()
                                .or(Expr::col(C::ReplacedProposalVersion).gte(1)),
                        ),
                )
                .col(non_empty_text(C::Text))
                .col(
                    ColumnDef::new(C::CoveredMessageIds)
                        .json_binary()
                        .not_null(),
                )
                .col(ColumnDef::new(C::Basis).json_binary().not_null())
                .col(ColumnDef::new(C::Citations).json_binary().not_null())
                .col(ColumnDef::new(C::ProposedAt).big_integer().not_null())
                .primary_key(
                    Index::create()
                        .name("pk-physician_runtime_reply_proposals")
                        .col(C::Practice)
                        .col(C::ConversationId)
                        .col(C::Version),
                )
                .to_owned(),
        )
        .await?;

    manager
        .create_index(
            Index::create()
                .name("uq-physician_runtime_reply_proposals-completion_key")
                .table(reply_proposal::Entity)
                .col(C::Practice)
                .col(C::ConversationId)
                .col(C::CompletionKey)
                .unique()
                .to_owned(),
        )
        .await?;

    manager
        .create_index(
            Index::create()
                .name("idx-physician_runtime_reply_proposals-workspace")
                .table(reply_proposal::Entity)
                .col(C::Practice)
                .col(C::Workspace)
                .col(C::ProposedAt)
                .to_owned(),
        )
        .await
}

async fn create_reply_review_completions(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use reply_review_completion::Column as C;

    let non_empty_text = |column: C| {
        ColumnDef::new(column)
            .text()
            .not_null()
            .check(Expr::col(column).ne(""))
            .to_owned()
    };

    manager
        .create_table(
            Table::create()
                .table(reply_review_completion::Entity)
                .col(non_empty_text(C::Practice))
                .col(non_empty_text(C::ConversationId))
                .col(non_empty_text(C::CompletionKey))
                .col(non_empty_text(C::RequestDigest))
                .col(non_empty_text(C::ReviewId))
                .col(non_empty_text(C::RunId))
                .col(non_empty_text(C::Workspace))
                .col(ColumnDef::new(C::Outcome).json_binary().not_null())
                .col(ColumnDef::new(C::CompletedAt).big_integer().not_null())
                .primary_key(
                    Index::create()
                        .name("pk-physician_runtime_reply_review_completions")
                        .col(C::Practice)
                        .col(C::ConversationId)
                        .col(C::CompletionKey),
                )
                .to_owned(),
        )
        .await
}

async fn create_reply_draft_bindings(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use reply_draft_binding::Column as C;

    let non_empty_text = |column: C| {
        ColumnDef::new(column)
            .text()
            .not_null()
            .check(Expr::col(column).ne(""))
            .to_owned()
    };

    let revision_has_restore_text = Condition::any()
        .add(Expr::col(C::Standing).ne("coda_revision"))
        .add(
            Condition::all()
                .add(Expr::col(C::PreRevisionYourText).is_not_null())
                .add(Expr::col(C::PreRevisionYourText).ne("")),
        );

    manager
        .create_table(
            Table::create()
                .table(reply_draft_binding::Entity)
                .col(non_empty_text(C::Practice))
                .col(non_empty_text(C::ConversationId))
                .col(non_empty_text(C::Physician))
                .col(
                    ColumnDef::new(C::ReplyId)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::ReplyId).gte(1)),
                )
                .col(non_empty_text(C::Workspace))
                .col(non_empty_text(C::DraftScopeChat))
                .col(ColumnDef::new(C::Standing).text().not_null().check(
                    Expr::col(C::Standing).is_in([
                        "your_text",
                        "coda_revision",
                        "detached_chose_update",
                    ]),
                ))
                .col(
                    ColumnDef::new(C::DraftRevision)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::DraftRevision).gte(1)),
                )
                .col(
                    ColumnDef::new(C::BasedOnProposalVersion)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::BasedOnProposalVersion).gte(1)),
                )
                .col(ColumnDef::new(C::PreRevisionYourText).text())
                .col(
                    ColumnDef::new(C::UpdateOfferedProposalVersion)
                        .big_integer()
                        .check(
                            Expr::col(C::UpdateOfferedProposalVersion)
                                .is_null()
                                .or(Expr::col(C::UpdateOfferedProposalVersion).gte(1)),
                        ),
                )
                .col(ColumnDef::new(C::UpdatedAt).big_integer().not_null())
                .check(revision_has_restore_text)
                .primary_key(
                    Index::create()
                        .name("pk-physician_runtime_reply_draft_bindings")
                        .col(C::Practice)
                        .col(C::ConversationId)
                        .col(C::Physician)
                        .col(C::ReplyId),
                )
                .to_owned(),
        )
        .await
}

async fn create_reply_copies(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use reply_copy::Column as C;

    let non_empty_text = |column: C| {
        ColumnDef::new(column)
            .text()
            .not_null()
            .check(Expr::col(column).ne(""))
            .to_owned()
    };

    manager
        .create_table(
            Table::create()
                .table(reply_copy::Entity)
                .col(non_empty_text(C::Practice))
                .col(non_empty_text(C::ConversationId))
                .col(non_empty_text(C::CopyId))
                .col(non_empty_text(C::CommandKey))
                .col(non_empty_text(C::Physician))
                .col(non_empty_text(C::Workspace))
                .col(
                    ColumnDef::new(C::ReplyId)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::ReplyId).gte(1)),
                )
                .col(
                    ColumnDef::new(C::PracticeObservationSeq)
                        .big_integer()
                        .not_null()
                        .check(Expr::col(C::PracticeObservationSeq).gte(0)),
                )
                .col(ColumnDef::new(C::CopiedSource).json_binary().not_null())
                .col(non_empty_text(C::CopiedText))
                .col(ColumnDef::new(C::CopiedAt).big_integer().not_null())
                .primary_key(
                    Index::create()
                        .name("pk-physician_runtime_reply_copies")
                        .col(C::Practice)
                        .col(C::ConversationId)
                        .col(C::CopyId),
                )
                .to_owned(),
        )
        .await?;

    manager
        .create_index(
            Index::create()
                .name("uq-physician_runtime_reply_copies-command_key")
                .table(reply_copy::Entity)
                .col(C::Practice)
                .col(C::ConversationId)
                .col(C::Physician)
                .col(C::CommandKey)
                .unique()
                .to_owned(),
        )
        .await?;

    manager
        .create_index(
            Index::create()
                .name("idx-physician_runtime_reply_copies-observation_seq")
                .table(reply_copy::Entity)
                .col(C::Practice)
                .col(C::ConversationId)
                .col(C::PracticeObservationSeq)
                .col(C::CopiedAt)
                .to_owned(),
        )
        .await
}
