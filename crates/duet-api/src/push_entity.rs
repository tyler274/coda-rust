//! Typed SeaORM entities for multi-device push registrations and generic deliveries.

use sea_orm::entity::prelude::*;

pub(crate) mod device {
    use super::*;
    use std::fmt;

    #[derive(Clone, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "duet_push_devices")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub device_id: String,
        pub session_authenticated_at: DateTimeUtc,
        pub platform: String,
        pub push_token: String,
        pub registered_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
        pub revoked_at: Option<DateTimeUtc>,
    }

    impl fmt::Debug for Model {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PushDeviceModel")
                .field("user_id", &self.user_id)
                .field("device_id", &self.device_id)
                .field("session_authenticated_at", &self.session_authenticated_at)
                .field("platform", &self.platform)
                .field("push_token", &"<redacted>")
                .field("registered_at", &self.registered_at)
                .field("updated_at", &self.updated_at)
                .field("revoked_at", &self.revoked_at)
                .finish()
        }
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod delivery {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "duet_push_deliveries")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub device_id: String,
        pub alert_kind: String,
        pub status: String,
        pub created_at: DateTimeUtc,
        pub dispatched_at: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod account_fence {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub sessions_revoked_before: Option<DateTimeUtc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
