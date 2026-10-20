select
    exists (
        select
            1
        from
            public.schema_migrations
        where
            version = $1)
