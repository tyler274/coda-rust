//! Explicit operator-only front-door mapping; no email matching and no public linking endpoint.
use sqlx::{Connection, PgConnection, Row};
#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(
        args.len(),
        3,
        "usage: duet-map-front-door USERS_UUID FRONT_DOOR_UUID EXPECTED_PROVIDER_SUBJECT"
    );
    let account: uuid::Uuid = args[0].parse().expect("existing users UUID");
    let door: uuid::Uuid = args[1].parse().expect("verified front-door UUID");
    let mut conn = PgConnection::connect(
        &std::env::var("DUET_MIGRATION_DATABASE_URL").expect("operator migration credentials"),
    )
    .await
    .expect("operator database");
    let mut tx = conn.begin().await.expect("transaction");
    let row = sqlx::query("select auth_user_id from users where id=$1 for update")
        .bind(account)
        .fetch_one(&mut *tx)
        .await
        .expect("existing account");
    assert_eq!(
        row.get::<String, _>("auth_user_id"),
        args[2],
        "provider identity proof differs"
    );
    // Operator supplies an audited front-door UUID; no lookup by email is permitted.
    sqlx::query("insert into duet_front_door_mapping(front_door_account_id,user_id) values($1,$2) on conflict(user_id) do nothing").bind(door).bind(account).execute(&mut *tx).await.expect("mapping must be unique");
    let actual: uuid::Uuid = sqlx::query_scalar(
        "select front_door_account_id from duet_front_door_mapping where user_id=$1",
    )
    .bind(account)
    .fetch_one(&mut *tx)
    .await
    .expect("mapping readback");
    assert_eq!(actual, door, "existing mapping differs");
    tx.commit().await.expect("commit mapping");
}
