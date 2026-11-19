//! Server-persisted per-physician panel collapse preferences (`P4.Rule.PanelChoiceRemembered`).
//!
//! Stored in Postgres keyed by `(practice, physician)` so sidebar and schedule
//! rail collapse choices survive sign-out and follow the physician across browsers
//! and devices while browser `localStorage` stays cleared on sign-out.

use sea_orm::{DbErr, EntityTrait, Set, sea_query::OnConflict};
use serde::{Deserialize, Serialize};

use super::{Store, error, now};

pub(in crate::runtime) mod table {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_panel_preferences")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub physician: String,
        pub sidebar_collapsed: bool,
        pub schedule_collapsed: bool,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Durable sidebar and schedule rail collapse preferences for one physician.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelPreferences {
    /// Whether the navigation sidebar is collapsed.
    pub sidebar_collapsed: bool,
    /// Whether the schedule rail is collapsed.
    pub schedule_collapsed: bool,
}

fn validate_scope<'a>(practice: &'a str, physician: &'a str) -> Result<(&'a str, &'a str), DbErr> {
    let practice = practice.trim();
    let physician = physician.trim();
    if practice.is_empty() || physician.is_empty() {
        return Err(error(
            "panel preferences practice and physician must not be empty",
        ));
    }
    Ok((practice, physician))
}

impl Store {
    /// Reads the saved [`PanelPreferences`] for `(practice, physician)`.
    /// Defaults to both panels expanded when the physician has not saved a preference yet.
    pub async fn panel_preferences(
        &self,
        practice: &str,
        physician: &str,
    ) -> Result<PanelPreferences, DbErr> {
        let (practice, physician) = validate_scope(practice, physician)?;
        let row = table::Entity::find_by_id((practice.to_owned(), physician.to_owned()))
            .one(&self.db)
            .await?;
        Ok(match row {
            Some(saved) => PanelPreferences {
                sidebar_collapsed: saved.sidebar_collapsed,
                schedule_collapsed: saved.schedule_collapsed,
            },
            None => PanelPreferences::default(),
        })
    }

    /// Saves [`PanelPreferences`] for `(practice, physician)` durably in Postgres.
    pub async fn save_panel_preferences(
        &self,
        practice: &str,
        physician: &str,
        preferences: PanelPreferences,
    ) -> Result<PanelPreferences, DbErr> {
        let (practice, physician) = validate_scope(practice, physician)?;
        let updated_at = now(&self.db).await?;
        table::Entity::insert(table::ActiveModel {
            practice: Set(practice.to_owned()),
            physician: Set(physician.to_owned()),
            sidebar_collapsed: Set(preferences.sidebar_collapsed),
            schedule_collapsed: Set(preferences.schedule_collapsed),
            updated_at: Set(updated_at),
        })
        .on_conflict(
            OnConflict::columns([table::Column::Practice, table::Column::Physician])
                .update_columns([
                    table::Column::SidebarCollapsed,
                    table::Column::ScheduleCollapsed,
                    table::Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        Ok(preferences)
    }
}
