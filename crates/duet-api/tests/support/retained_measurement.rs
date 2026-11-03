//! Seed an older retained context that predates producer fingerprints.

#[path = "../../src/health_context_entity.rs"]
mod context;

use sea_orm::{ActiveValue::Set, Database, EntityTrait};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn insert(owner_url: &str, account: Uuid, snapshot: Uuid, boundary: Uuid, fact: &Value) {
    let db = Database::connect(owner_url).await.unwrap();
    context::Entity::insert(context::ActiveModel {
        user_id: Set(account),
        snapshot_id: Set(snapshot),
        source_boundary_id: Set(boundary),
        generated_at: Set(chrono::Utc::now()),
        timezone: Set("UTC".into()),
        projections: Set(json!([{"kind":"daily"}])),
        facts: Set(json!([fact])),
        ..Default::default()
    })
    .exec_without_returning(&db)
    .await
    .unwrap();
}
