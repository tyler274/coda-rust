//! Extraction is layout-only: preserve every non-whitespace character of the
//! original static queries, including bind positions, ordering and predicates.
//! These three queries contain no quoted strings or comments, so removing
//! whitespace for this regression assertion cannot hide a literal change.

#[test]
fn extracted_queries_preserve_the_original_sql() {
    for (query, original) in [
        (
            include_str!("../src/sql/list_moments.sql"),
            "select id, kind, occurred_at from public.account_moments where user_id = $1 order by occurred_at desc, id desc",
        ),
        (
            include_str!("../src/sql/delete_moment.sql"),
            "delete from public.account_moments where id = $1 and user_id = $2",
        ),
        (
            include_str!("../src/sql/has_migration.sql"),
            "select exists (select 1 from public.schema_migrations where version = $1)",
        ),
    ] {
        let compact = |sql: &str| {
            sql.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        };
        assert_eq!(compact(query), compact(original));
    }
}
