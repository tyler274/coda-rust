//! The layout each source's records of one type read through, kept per
//! `(source, record type)`. A layout is held exactly as it was received: this
//! store checks only that it is JSON, and the reader that renders it owns its
//! shape and checks it against each record as it reads.
//!
//! A key's layout is immutable once stored. Saving the same bytes again
//! changes nothing; saving different bytes for a stored key is refused, so a
//! record type reads the same way every time it is cited, and two writers
//! racing on a first citation settle on whichever stored first.
use std::fmt;

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,
    sea_query::OnConflict,
};

use crate::runtime::Store;

/// One stored layout for one source's records of one type.
pub(crate) mod entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_source_layouts")]
    pub struct Model {
        /// The source's stable identifier.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        /// The source's own name for this kind of record.
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub record_type: String,
        /// The layout JSON, byte for byte as it was saved. Text rather than
        /// JSONB, which would reorder and reformat it.
        #[sea_orm(column_type = "Text")]
        pub layout: String,
        /// SHA-256 of `layout`'s bytes, checked on every read.
        #[sea_orm(column_type = "Text")]
        pub sha256: String,
        /// Database-clock seconds when the layout was stored.
        pub created_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Creates the layout table inside the store's migration transaction.
pub(crate) async fn migrate(tx: &impl ConnectionTrait) -> Result<(), DbErr> {
    let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
    let mut table = schema.create_table_from_entity(entity::Entity);
    table.if_not_exists();
    tx.execute(&table).await?;
    Ok(())
}

/// Which source's records of which type a layout reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LayoutKey<'a> {
    /// The source's stable identifier.
    pub source_id: &'a str,
    /// The source's own name for this kind of record.
    pub record_type: &'a str,
}

/// A layout as the store holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredLayout {
    /// The layout JSON exactly as it was saved.
    pub layout: String,
    /// SHA-256 of `layout`'s bytes.
    pub sha256: String,
    /// Database-clock seconds when it was stored.
    pub created_at: i64,
}

/// What a save did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Saved {
    /// The key had no layout; this one is stored now.
    Stored(StoredLayout),
    /// The key already held these exact bytes; nothing changed.
    Unchanged(StoredLayout),
}

/// Why a layout could not be read or saved.
#[derive(Debug)]
pub enum LayoutStoreError {
    /// The source or record type is empty.
    BlankKey,
    /// The layout to save is not JSON.
    NotJson(serde_json::Error),
    /// The key already holds a different layout, which stays.
    Conflict {
        /// SHA-256 of the layout the key holds.
        stored_sha256: String,
    },
    /// A stored layout no longer matches its digest.
    Corrupt,
    /// The database could not be reached or refused the statement.
    Database(DbErr),
}

impl fmt::Display for LayoutStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankKey => f.write_str("a layout key needs a source and a record type"),
            Self::NotJson(_) => f.write_str("the layout is not JSON"),
            Self::Conflict { stored_sha256 } => write!(
                f,
                "this source and record type already hold a different layout ({stored_sha256})"
            ),
            Self::Corrupt => f.write_str("a stored layout does not match its digest"),
            Self::Database(_) => f.write_str("the layout store is unavailable"),
        }
    }
}

impl std::error::Error for LayoutStoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NotJson(error) => Some(error),
            Self::Database(error) => Some(error),
            Self::BlankKey | Self::Conflict { .. } | Self::Corrupt => None,
        }
    }
}

impl From<DbErr> for LayoutStoreError {
    fn from(error: DbErr) -> Self {
        Self::Database(error)
    }
}

fn checked(key: LayoutKey<'_>) -> Result<LayoutKey<'_>, LayoutStoreError> {
    if key.source_id.is_empty() || key.record_type.is_empty() {
        return Err(LayoutStoreError::BlankKey);
    }
    Ok(key)
}

fn stored(row: entity::Model) -> Result<StoredLayout, LayoutStoreError> {
    if crate::sha256_hex(row.layout.as_bytes()) != row.sha256 {
        return Err(LayoutStoreError::Corrupt);
    }
    Ok(StoredLayout {
        layout: row.layout,
        sha256: row.sha256,
        created_at: row.created_at,
    })
}

async fn find(
    db: &impl ConnectionTrait,
    key: LayoutKey<'_>,
) -> Result<Option<entity::Model>, DbErr> {
    entity::Entity::find()
        .filter(entity::Column::SourceId.eq(key.source_id))
        .filter(entity::Column::RecordType.eq(key.record_type))
        .one(db)
        .await
}

impl Store {
    /// The layout stored for `key`. `Ok(None)` means only that none is
    /// stored; a failed read is an error, never `None`.
    pub async fn layout_for(
        &self,
        key: LayoutKey<'_>,
    ) -> Result<Option<StoredLayout>, LayoutStoreError> {
        let key = checked(key)?;
        find(self.connection(), key).await?.map(stored).transpose()
    }

    /// Stores `layout` for `key` exactly as given. The same bytes again are a
    /// no-op; different bytes for a stored key are refused and the stored
    /// layout stays.
    pub async fn save_layout(
        &self,
        key: LayoutKey<'_>,
        layout: &str,
    ) -> Result<Saved, LayoutStoreError> {
        let key = checked(key)?;
        serde_json::from_str::<serde::de::IgnoredAny>(layout).map_err(LayoutStoreError::NotJson)?;
        let sha256 = crate::sha256_hex(layout.as_bytes());
        let tx = self.connection().begin().await?;
        let created_at = crate::runtime::now(&tx).await?;
        let inserted = entity::Entity::insert(entity::ActiveModel {
            source_id: Set(key.source_id.into()),
            record_type: Set(key.record_type.into()),
            layout: Set(layout.into()),
            sha256: Set(sha256.clone()),
            created_at: Set(created_at),
        })
        .on_conflict(
            OnConflict::columns([entity::Column::SourceId, entity::Column::RecordType])
                .do_nothing()
                .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        // Read inside the same transaction: after a conflict the row is the
        // committed one that won.
        let row = find(&tx, key)
            .await?
            .ok_or_else(|| DbErr::Custom("stored layout vanished".into()))?;
        tx.commit().await?;
        let held = stored(row)?;
        if inserted == 1 {
            return Ok(Saved::Stored(held));
        }
        if held.layout != layout {
            return Err(LayoutStoreError::Conflict {
                stored_sha256: held.sha256,
            });
        }
        Ok(Saved::Unchanged(held))
    }
}
