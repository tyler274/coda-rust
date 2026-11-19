//! The SeaORM entities and the clinical library's migration chain.

use sea_orm::sea_query::{Func, Query};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, TransactionTrait};
use sea_orm_migration::prelude::*;

mod m20261109_init;
mod m20261110_pubmed_status;
mod m20261111_night_runner;
mod m20261119_corpus_and_freshness;

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
        vec![
            Box::new(m20261109_init::Migration),
            Box::new(m20261110_pubmed_status::Migration),
            Box::new(m20261111_night_runner::Migration),
            Box::new(m20261119_corpus_and_freshness::Migration),
        ]
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
        /// What a failed night's failure was, or the failure that stopped
        /// the nightly on the same failure twice.
        pub failure_code: Option<String>,
        /// The stop rule that kept a stopped night from running.
        pub stop_reason: Option<String>,
        /// The head of a failed night's failure, in words a person reads.
        pub failure_detail: Option<String>,
        /// When this outcome was recorded.
        pub recorded_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod night_item {
    use sea_orm::entity::prelude::*;

    /// One item a run was due to do, and what it did with it.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_night_items")]
    pub struct Model {
        /// The run.
        #[sea_orm(primary_key, auto_increment = false)]
        pub run_id: Uuid,
        /// The item's source.
        #[sea_orm(primary_key, auto_increment = false)]
        pub source: String,
        /// The item's key.
        #[sea_orm(primary_key, auto_increment = false)]
        pub item_key: String,
        /// `due`, `done` or `set_aside`.
        pub result: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod unpublished {
    use sea_orm::entity::prelude::*;

    /// Admitted content no version has published yet.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_unpublished")]
    pub struct Model {
        /// The item's source.
        #[sea_orm(primary_key, auto_increment = false)]
        pub source: String,
        /// The item's key.
        #[sea_orm(primary_key, auto_increment = false)]
        pub item_key: String,
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

#[allow(missing_docs)]
pub(crate) mod pubmed_status {
    use sea_orm::entity::prelude::*;

    /// NLM's status for one article.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_pubmed_status")]
    pub struct Model {
        /// The article's PMID.
        #[sea_orm(primary_key, auto_increment = false)]
        pub pmid: i64,
        /// NLM's publication types, in NLM's order.
        pub publication_types: Vec<String>,
        /// When NLM last revised the record.
        pub revised: Option<Date>,
        /// The hash of the record the status came from.
        pub record_sha256: Vec<u8>,
        /// The record exactly as NLM sent it.
        pub record_xml: String,
        /// When the library first held this record.
        pub observed_at: DateTimeWithTimeZone,
        /// When NLM last sent it unchanged.
        pub confirmed_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod pubmed_relation {
    use sea_orm::entity::prelude::*;

    /// One comments-and-corrections link NLM records on an article.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_pubmed_relations")]
    pub struct Model {
        /// The article's PMID.
        #[sea_orm(primary_key, auto_increment = false)]
        pub pmid: i64,
        /// The link's place in NLM's order.
        #[sea_orm(primary_key, auto_increment = false)]
        pub position: i16,
        /// NLM's RefType.
        pub ref_type: String,
        /// The linked article's PMID.
        pub target_pmid: Option<i64>,
        /// NLM's citation of the linked item.
        pub source_reference: Option<String>,
        /// NLM's note.
        pub note: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod document {
    use sea_orm::entity::prelude::*;

    /// One admitted evidence document in the corpus.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_documents")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub source: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub item_key: String,
        pub source_code: String,
        pub source_display_name: String,
        pub native_document_id: String,
        pub native_version_id: Option<String>,
        pub source_version_label: Option<String>,
        pub title: String,
        pub document_type: String,
        pub publication_date: Option<String>,
        pub version_date: Option<String>,
        pub acquired_at: DateTimeWithTimeZone,
        pub lifecycle_state: String,
        pub source_status_code: Option<String>,
        pub native_destination_uri: String,
        pub version_destination_uri: String,
        pub rights_decision_id: String,
        pub license_identifier: Option<String>,
        pub attribution_text: Option<String>,
        pub may_display_excerpt: bool,
        pub identifiers_json: String,
        pub relationships_json: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod unit {
    use sea_orm::entity::prelude::*;

    /// One searchable passage unit belonging to an admitted document.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_units")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub unit_id: Uuid,
        pub source: String,
        pub item_key: String,
        pub native_unit_id: String,
        pub native_locator: String,
        pub unit_kind: String,
        pub ordinal: i32,
        pub label: Option<String>,
        pub content: String,
        pub content_sha256: String,
        pub native_destination_uri: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod version_unit {
    use sea_orm::entity::prelude::*;

    /// Membership of a unit inside one built corpus version snapshot.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_version_units")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub version_id: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub unit_id: Uuid,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod source_status {
    use sea_orm::entity::prelude::*;

    /// Per-source refresh confirmation watermark.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_source_status")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub source: String,
        pub last_checked_at: Option<DateTimeWithTimeZone>,
        pub last_succeeded_at: Option<DateTimeWithTimeZone>,
        pub last_run_id: Option<Uuid>,
        pub last_error_code: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(missing_docs)]
pub(crate) mod search_latency {
    use sea_orm::entity::prelude::*;

    /// Recorded latency observation for search/read endpoint requests.
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "clinical_library_search_latencies")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub recorded_at: DateTimeWithTimeZone,
        pub endpoint: String,
        pub duration_ms: f64,
        pub candidate_count: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
