//! Preserve the external schema-readiness query. Moments CRUD is now typed ORM;
//! it has behavioral PostgreSQL proof rather than an authored-SQL quota.

#[test]
fn schema_readiness_query_preserves_the_original_sql() {
    let compact = |sql: &str| {
        sql.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
    assert_eq!(
        compact(include_str!("../src/sql/has_migration.sql")),
        compact("select exists (select 1 from public.schema_migrations where version = $1)"),
    );
}
