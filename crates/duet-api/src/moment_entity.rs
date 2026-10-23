//! Duet-owned mapping of the externally managed account-moments relation.
//!
//! This adapter does not create a schema or replace the legacy migration ledger.
//! Keep kind as text: unknown stored kinds must fail wire conversion explicitly.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "account_moments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: String,
    pub occurred_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
