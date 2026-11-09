//! The SeaORM entities and the clinical library's migration chain.

use sea_orm::sea_query::{Func, Query};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, TransactionTrait};
use sea_orm_migration::prelude::*;

mod m20261109_init;

/// The table this chain records applied migrations in. The library may share
/// a database with other chains, and each chain owns its own ledger.
pub const MIGRATION_TABLE: &str = "clinical_library_migrations";

/// Every migration, oldest first.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> DynIden {
        Alias::new(MIGRATION_TABLE).into_iden()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20261109_init::Migration)]
    }
}

/// Applies every pending migration in one transaction. Concurrent callers
/// wait for each other rather than racing on the ledger.
pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let tx = db.begin().await?;
    // A transaction advisory lock, because a row lock can't protect a
    // ledger that may not exist yet.
    tx.query_one(
        &Query::select()
            .expr(Func::cust("pg_advisory_xact_lock").arg(740146403_i64))
            .to_owned(),
    )
    .await?;
    Migrator::up(&tx, None).await?;
    tx.commit().await
}

// The entity models carry field docs; the generated struct and relation
// types are mechanical SeaORM shapes.
#[allow(missing_docs)]
pub(crate) mod slot {
    use sea_orm::entity::prelude::*;

    /// One slot and the run that holds it.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_slots")]
    pub struct Model {
        /// The slot's name.
        #[sea_orm(primary_key, auto_increment = false)]
        pub slot: String,
        /// The run holding it, if any.
        pub holder: Option<Uuid>,
        /// Goes up with each new holder.
        pub token: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod item {
    use sea_orm::entity::prelude::*;

    /// One source item and the state it is in.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_items")]
    pub struct Model {
        /// The source it comes from.
        #[sea_orm(primary_key, auto_increment = false)]
        pub source: String,
        /// The source's own key for it, such as a PMID.
        #[sea_orm(primary_key, auto_increment = false)]
        pub item_key: String,
        /// The state's stored name.
        pub state: String,
        /// Outside failures in a row, while waiting to retry.
        pub failures: i16,
        /// Why it waits for a change.
        pub wait_reason: Option<String>,
        /// The latest outside failure.
        pub last_failure: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod night {
    use sea_orm::entity::prelude::*;

    /// How one run's night ended.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_nights")]
    pub struct Model {
        /// Its place in the history.
        #[sea_orm(primary_key)]
        pub id: i64,
        /// The slot the run ran under.
        pub slot: String,
        /// The run.
        pub run_id: Uuid,
        /// The outcome's stored name.
        pub outcome: String,
        /// The version a successful night activated.
        pub activated_version: Option<String>,
        /// Why a deferred night deferred.
        pub deferral_reason: Option<String>,
        /// What a failed night's failure was.
        pub failure_code: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod resolution {
    use sea_orm::entity::prelude::*;

    /// A person recording that they resolved what stopped the nightly.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_resolutions")]
    pub struct Model {
        /// Its place in the history.
        #[sea_orm(primary_key)]
        pub id: i64,
        /// The slot it resolves.
        pub slot: String,
        /// Who resolved it.
        pub recorded_by: String,
        /// What they did.
        pub reason: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod version {
    use sea_orm::entity::prelude::*;

    /// One built version of the search index.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_versions")]
    pub struct Model {
        /// The version's name.
        #[sea_orm(primary_key, auto_increment = false)]
        pub version: String,
        /// Whether it passed its checks.
        pub checked: bool,
        /// Whether search serves it.
        pub active: bool,
        /// The run that activated it.
        pub activated_by_run: Option<Uuid>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
