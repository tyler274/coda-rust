use sqlx::postgres::PgConnectOptions;

pub fn options(url: &str) -> PgConnectOptions {
    let options: PgConnectOptions = url.parse().expect("valid disposable PostgreSQL URL");
    assert!(
        allowed(
            &options,
            std::env::var("DUET_TEST_ALLOW_POSTGRES_SERVICE")
                .ok()
                .as_deref()
        ),
        "Duet tests require loopback PostgreSQL or the explicitly opted-in disposable postgres service"
    );
    options
}

fn allowed(options: &PgConnectOptions, service_opt_in: Option<&str>) -> bool {
    // SQLx preserves brackets for an IPv6 URL authority, but not a host option.
    matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1" | "[::1]"
    ) || (options.get_host() == "postgres" && service_opt_in == Some("1"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_remains_allowed_without_opt_in() {
        for host in ["localhost", "127.0.0.1", "[::1]"] {
            let options = format!("postgres://test@{host}/test").parse().unwrap();
            assert!(allowed(&options, None), "{host}");
        }
    }

    #[test]
    fn service_requires_exact_explicit_opt_in() {
        let options = "postgres://test@postgres:5432/test".parse().unwrap();
        for opt_in in [None, Some(""), Some("0"), Some("true")] {
            assert!(!allowed(&options, opt_in));
        }
        assert!(allowed(&options, Some("1")));
    }

    #[test]
    fn remote_hosts_and_misleading_userinfo_remain_denied() {
        for url in [
            "postgres://test@remote.example/test",
            "postgres://postgres@remote.example/test",
            "postgres://localhost@remote.example/test",
            "postgres://test:postgres@remote.example/test",
            "postgres://test@postgres.remote.example/test",
            "postgres://test@127.0.0.1.remote.example/test",
            "postgres://test@postgres/test?host=remote.example",
            "postgres://test@localhost/test?host=remote.example",
            "postgres://test@postgres/test?hostaddr=192.0.2.1",
            "postgres://test@localhost/test?hostaddr=192.0.2.1",
        ] {
            let options = url.parse().unwrap();
            for opt_in in [None, Some("1")] {
                assert!(!allowed(&options, opt_in), "{url}");
            }
        }
    }
}
