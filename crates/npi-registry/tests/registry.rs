//! Recorded-fixture, validation, client, and (opt-in) live NPPES tests.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use npi_registry::{
    Client, ClientConfig, Clinician, ClinicianKind, Directory, FixtureDirectory, MAX_LIMIT, Npi,
    NpiError, RegistryError, SearchQuery, parse_record, parse_response, samples,
};

fn one(body: &str) -> Clinician {
    let mut clinicians = parse_response(body).unwrap();
    assert_eq!(clinicians.len(), 1);
    clinicians.remove(0)
}

fn npi(value: &str) -> Npi {
    Npi::parse(value).unwrap()
}

#[test]
fn luhn_accepts_valid_npis_and_rejects_bad_ones() {
    for valid in [
        "1234567893",
        "1987654328",
        "1555012347",
        "1677000014",
        " 1000000004 ",
    ] {
        assert!(Npi::parse(valid).is_ok(), "{valid}");
    }
    assert_eq!(Npi::parse("1234567890"), Err(NpiError::CheckDigit));
    assert_eq!(Npi::parse("1234567894"), Err(NpiError::CheckDigit));
    assert_eq!(Npi::parse("123456789"), Err(NpiError::Format));
    assert_eq!(Npi::parse("12345678930"), Err(NpiError::Format));
    assert_eq!(Npi::parse("12345a7893"), Err(NpiError::Format));
    assert_eq!(Npi::parse(""), Err(NpiError::Format));
    assert_eq!(Npi::parse("１２３４５６７８９３"), Err(NpiError::Format));
    assert_eq!(npi(" 1234567893").as_str(), "1234567893");
    assert_eq!(
        "1234567893".parse::<Npi>().unwrap().to_string(),
        "1234567893"
    );
}

#[test]
fn npi_serializes_as_a_string_and_refuses_bad_digits() {
    assert_eq!(
        serde_json::to_string(&npi("1234567893")).unwrap(),
        "\"1234567893\""
    );
    assert_eq!(
        serde_json::from_str::<Npi>("\"1987654328\"").unwrap(),
        npi("1987654328")
    );
    assert!(serde_json::from_str::<Npi>("\"1234567890\"").is_err());
}

#[test]
fn parses_an_individual_record() {
    let clinician = one(samples::INDIVIDUAL);
    assert_eq!(clinician.npi, npi("1234567893"));
    assert_eq!(clinician.kind, ClinicianKind::Individual);
    assert_eq!(clinician.name, "AVERY J QUINLAN");
    let person = clinician.person.as_ref().unwrap();
    assert_eq!(person.prefix.as_deref(), Some("Dr."));
    assert_eq!(person.first, "AVERY");
    assert_eq!(person.middle.as_deref(), Some("J"));
    assert_eq!(person.last, "QUINLAN");
    assert_eq!(person.suffix, None, "`--` means absent");
    assert_eq!(clinician.credential.as_deref(), Some("MD"));

    let primary = clinician.primary_taxonomy.as_ref().unwrap();
    assert_eq!(primary.code, "207R00000X");
    assert_eq!(primary.description, "Internal Medicine");
    assert!(primary.primary);
    assert_eq!(primary.state.as_deref(), Some("MA"));
    assert_eq!(clinician.taxonomies.len(), 2);
    assert_eq!(clinician.taxonomies[0].code, "207RC0000X");

    let practice = clinician.practice_address.as_ref().unwrap();
    assert_eq!(practice.line1, "100 EXAMPLE MEDICAL PLZ");
    assert_eq!(practice.line2.as_deref(), Some("SUITE 210"));
    assert_eq!(practice.city, "SPRINGFIELD");
    assert_eq!(practice.state, "MA");
    assert_eq!(practice.postal_code, "01103-0001");
    assert_eq!(practice.country_code.as_deref(), Some("US"));
    let mailing = clinician.mailing_address.as_ref().unwrap();
    assert_eq!(mailing.line1, "PO BOX 4400");
    assert_eq!(mailing.fax, None);

    assert_eq!(clinician.phone.as_deref(), Some("413-555-0142"));
    assert_eq!(clinician.fax.as_deref(), Some("413-555-0199"));
    assert_eq!(clinician.endpoints.len(), 2);
    assert_eq!(
        clinician.direct_addresses().collect::<Vec<_>>(),
        ["aquinlan@direct.example-health.org"]
    );
    assert_eq!(clinician.endpoints[1].kind, "FHIR");
    assert_eq!(clinician.last_updated.as_deref(), Some("2024-03-18"));
}

#[test]
fn parses_an_organization_record() {
    let clinician = one(samples::ORGANIZATION);
    assert_eq!(clinician.npi, npi("1987654328"));
    assert_eq!(clinician.kind, ClinicianKind::Organization);
    assert_eq!(clinician.name, "BACK BAY EXAMPLE FAMILY MEDICINE LLC");
    assert_eq!(clinician.person, None);
    assert_eq!(clinician.credential, None);
    assert_eq!(
        clinician.primary_taxonomy.as_ref().unwrap().description,
        "Family Medicine"
    );
    let taxonomy = &clinician.taxonomies[1];
    assert_eq!(
        (taxonomy.state.as_ref(), taxonomy.license.as_ref()),
        (None, None)
    );
    assert_eq!(clinician.phone.as_deref(), Some("617-555-0123"));
    assert_eq!(clinician.fax.as_deref(), Some("617-555-0177"));
    let mailing = clinician.mailing_address.as_ref().unwrap();
    assert_eq!(mailing.postal_code, "02116");
    assert_eq!(mailing.line2.as_deref(), Some("ATTN MEDICAL RECORDS"));
    assert!(clinician.endpoints.is_empty());
}

#[test]
fn parses_search_results_and_empty_and_error_bodies() {
    let results = parse_response(samples::SEARCH).unwrap();
    assert_eq!(results.len(), 2);
    // The practice address is the LOCATION entry, even when MAILING is first.
    assert_eq!(
        results[0].practice_address.as_ref().unwrap().city,
        "AMHERST"
    );
    assert_eq!(results[0].phone.as_deref(), Some("413-555-0137"));
    assert_eq!(results[1].name, "ELLIS SAMPLEWOOD JR.");
    assert_eq!(results[1].credential.as_deref(), Some("M.D."));

    assert!(parse_response(samples::EMPTY).unwrap().is_empty());

    let Err(RegistryError::Rejected(messages)) = parse_response(samples::ERROR) else {
        panic!("Errors body must be a rejection");
    };
    assert_eq!(messages.len(), 1);
    assert_eq!(
        messages[0].description,
        "Field state requires additional search criteria"
    );
    assert_eq!(messages[0].field.as_deref(), Some("state"));
    assert_eq!(messages[0].code.as_deref(), Some("07"));

    assert!(matches!(
        parse_response("<html>"),
        Err(RegistryError::Decode(_))
    ));
}

#[test]
fn parse_record_refuses_bad_check_digits_and_unknown_types() {
    let mut record: serde_json::Value = serde_json::from_str(samples::INDIVIDUAL).unwrap();
    let mut record = record["results"][0].take();
    assert!(parse_record(&record).is_ok());
    record["number"] = "1234567890".into();
    assert!(matches!(
        parse_record(&record),
        Err(RegistryError::Decode(_))
    ));
    record["number"] = serde_json::json!(1234567893_u64);
    assert!(parse_record(&record).is_ok(), "numeric NPI is tolerated");
    record["enumeration_type"] = "NPI-3".into();
    assert!(matches!(
        parse_record(&record),
        Err(RegistryError::Decode(_))
    ));
}

#[test]
fn clinician_snapshots_round_trip_as_json() {
    for body in [samples::INDIVIDUAL, samples::ORGANIZATION] {
        let clinician = one(body);
        let json = serde_json::to_value(&clinician).unwrap();
        assert_eq!(json["npi"], clinician.npi.as_str());
        let back: Clinician = serde_json::from_value(json).unwrap();
        assert_eq!(back, clinician);
    }
    let kind = serde_json::to_string(&ClinicianKind::Organization).unwrap();
    assert_eq!(kind, "\"organization\"");
}

fn query() -> SearchQuery {
    SearchQuery::default()
}

#[test]
fn query_validation_matches_nppes_rules() {
    assert!(matches!(
        query().validate(),
        Err(RegistryError::InvalidQuery(_))
    ));
    let state_only = SearchQuery {
        state: Some("MA".into()),
        ..query()
    };
    assert!(state_only.validate().is_err(), "state alone is not enough");
    let kind_and_state = SearchQuery {
        kind: Some(ClinicianKind::Individual),
        ..state_only.clone()
    };
    assert!(kind_and_state.validate().is_err());
    let blank = SearchQuery {
        last_name: Some("   ".into()),
        ..state_only.clone()
    };
    assert!(blank.validate().is_err(), "blank values do not count");

    let ok = SearchQuery {
        last_name: Some(" samplewood ".into()),
        state: Some("ma".into()),
        ..query()
    };
    assert_eq!(
        ok.criteria().unwrap(),
        [
            ("last_name", "samplewood".to_owned()),
            ("state", "MA".to_owned())
        ]
    );

    for (field, value, valid) in [
        ("last", "sa*", true),
        ("last", "s*", false),
        ("last", "*", false),
        ("last", "sa*m", false),
        ("last", "sa**", false),
        ("postal", "02*", true),
        ("postal", "0*", false),
        ("postal", "02116", true),
        ("postal", "021160002", true),
        ("postal", "0211600021", false),
        ("postal", "ABCDE", false),
        ("state", "Massachusetts", false),
        ("state", "M1", false),
    ] {
        let mut candidate = SearchQuery {
            city: Some("Boston".into()),
            ..query()
        };
        match field {
            "last" => candidate.last_name = Some(value.into()),
            "postal" => candidate.postal_code = Some(value.into()),
            _ => candidate.state = Some(value.into()),
        }
        assert_eq!(candidate.validate().is_ok(), valid, "{field}={value}");
    }

    let with_kind = SearchQuery {
        organization_name: Some("back bay*".into()),
        kind: Some(ClinicianKind::Organization),
        ..query()
    };
    assert!(
        with_kind
            .criteria()
            .unwrap()
            .contains(&("enumeration_type", "NPI-2".to_owned()))
    );
}

#[test]
fn limit_is_defaulted_and_capped() {
    assert_eq!(query().effective_limit(), 10);
    let big = SearchQuery {
        limit: Some(500),
        skip: Some(5000),
        ..query()
    };
    assert_eq!(big.effective_limit(), MAX_LIMIT);
    assert_eq!(big.effective_skip(), 1000);
    let zero = SearchQuery {
        limit: Some(0),
        ..query()
    };
    assert_eq!(zero.effective_limit(), 1);
}

#[test]
fn search_terms_stay_out_of_debug_and_errors() {
    let secret = SearchQuery {
        first_name: Some("Zebulonsecret".into()),
        last_name: Some("Q*".into()),
        state: Some("MA".into()),
        ..query()
    };
    let debug = format!("{secret:?}");
    assert!(!debug.contains("Zebulonsecret"), "{debug}");
    assert!(debug.contains("first_name"));
    let error = secret.validate().unwrap_err().to_string();
    assert!(!error.contains("Q*"), "{error}");
    assert!(error.contains("last_name"));
}

#[tokio::test]
async fn fixture_directory_answers_without_network() {
    let directory = FixtureDirectory::sample();
    assert_eq!(directory.clinicians().len(), 4);
    let found = directory.by_number(&npi("1234567893")).await.unwrap();
    assert_eq!(found.unwrap().name, "AVERY J QUINLAN");
    assert_eq!(directory.by_number(&npi("1000000004")).await.unwrap(), None);

    let by_last = directory
        .search(&SearchQuery {
            last_name: Some("samplewood".into()),
            state: Some("ma".into()),
            ..query()
        })
        .await
        .unwrap();
    assert_eq!(by_last.len(), 2);
    let wildcard = directory
        .search(&SearchQuery {
            last_name: Some("QUIN*".into()),
            ..query()
        })
        .await
        .unwrap();
    assert_eq!(wildcard.len(), 1);
    let specialty = directory
        .search(&SearchQuery {
            taxonomy_description: Some("cardiovascular".into()),
            kind: Some(ClinicianKind::Individual),
            limit: Some(1),
            ..query()
        })
        .await
        .unwrap();
    assert_eq!(specialty.len(), 1);
    let organization = directory
        .search(&SearchQuery {
            organization_name: Some("back bay*".into()),
            postal_code: Some("02116".into()),
            ..query()
        })
        .await
        .unwrap();
    assert_eq!(organization[0].npi, npi("1987654328"));
    assert!(
        directory
            .search(&SearchQuery {
                state: Some("MA".into()),
                ..query()
            })
            .await
            .is_err()
    );

    directory.remove(&npi("1234567893"));
    assert_eq!(directory.by_number(&npi("1234567893")).await.unwrap(), None);
    directory.insert(one(samples::INDIVIDUAL));
    assert!(
        directory
            .by_number(&npi("1234567893"))
            .await
            .unwrap()
            .is_some()
    );

    let shared: Arc<dyn Directory> = Arc::new(FixtureDirectory::failing(
        RegistryError::Unavailable("down".into()),
    ));
    assert!(matches!(
        shared.by_number(&npi("1234567893")).await,
        Err(RegistryError::Unavailable(_))
    ));
}

#[derive(Clone, Default)]
struct Recorded {
    queries: Arc<Mutex<Vec<HashMap<String, String>>>>,
}

async fn nppes(
    State(recorded): State<Recorded>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    recorded.queries.lock().unwrap().push(params.clone());
    let body = match params.get("number").map(String::as_str) {
        Some("1234567893") => samples::INDIVIDUAL,
        Some("1987654328") => samples::ORGANIZATION,
        Some(_) => samples::EMPTY,
        None if params.contains_key("last_name") => samples::SEARCH,
        None => samples::ERROR,
    };
    ([("content-type", "application/json")], body).into_response()
}

async fn serve(recorded: Recorded) -> String {
    let app = axum::Router::new()
        .route("/api/", get(nppes))
        .route("/moved/", get(|| async { Redirect::temporary("/api/") }))
        .route(
            "/down/",
            get(|| async { (StatusCode::SERVICE_UNAVAILABLE, "maintenance") }),
        )
        .route(
            "/slow/",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(5)).await;
                samples::EMPTY
            }),
        )
        .with_state(recorded);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    format!("http://{address}")
}

#[tokio::test]
async fn client_calls_nppes_shape_and_caches_by_npi() {
    let recorded = Recorded::default();
    let base = serve(recorded.clone()).await;
    let client = Client::with_base_url(format!("{base}/api/")).unwrap();

    let found = client.by_number(&npi("1234567893")).await.unwrap().unwrap();
    assert_eq!(found.kind, ClinicianKind::Individual);
    let again = client.by_number(&npi("1234567893")).await.unwrap().unwrap();
    assert_eq!(again, found);
    assert_eq!(
        recorded.queries.lock().unwrap().len(),
        1,
        "second hit cached"
    );
    {
        let queries = recorded.queries.lock().unwrap();
        assert_eq!(queries[0]["version"], "2.1");
        assert_eq!(queries[0]["number"], "1234567893");
    }

    assert_eq!(client.by_number(&npi("1000000004")).await.unwrap(), None);

    let results = client
        .search(&SearchQuery {
            last_name: Some("Samplewood".into()),
            state: Some("ma".into()),
            limit: Some(500),
            skip: Some(10),
            ..query()
        })
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    {
        let queries = recorded.queries.lock().unwrap();
        let search = queries.last().unwrap();
        assert_eq!(search["limit"], "50");
        assert_eq!(search["skip"], "10");
        assert_eq!(search["state"], "MA");
        assert_eq!(search["last_name"], "Samplewood");
    }
    // Search results warm the NPI cache.
    let before = recorded.queries.lock().unwrap().len();
    client.by_number(&npi("1677000014")).await.unwrap().unwrap();
    assert_eq!(recorded.queries.lock().unwrap().len(), before);

    // Invalid queries never reach the network.
    let before = recorded.queries.lock().unwrap().len();
    assert!(matches!(
        client
            .search(&SearchQuery {
                state: Some("MA".into()),
                ..query()
            })
            .await,
        Err(RegistryError::InvalidQuery(_))
    ));
    assert_eq!(recorded.queries.lock().unwrap().len(), before);

    // An NPPES Errors body surfaces as a rejection.
    let rejected = client
        .search(&SearchQuery {
            city: Some("Boston".into()),
            ..query()
        })
        .await;
    assert!(matches!(rejected, Err(RegistryError::Rejected(_))));

    client.clear_cache();
    client.by_number(&npi("1234567893")).await.unwrap();
    assert_eq!(
        recorded.queries.lock().unwrap().last().unwrap()["number"],
        "1234567893"
    );
}

#[tokio::test]
async fn client_cache_can_be_disabled() {
    let recorded = Recorded::default();
    let base = serve(recorded.clone()).await;
    let client = Client::with_config(ClientConfig {
        base_url: format!("{base}/api/"),
        cache_ttl: Duration::ZERO,
        ..ClientConfig::default()
    })
    .unwrap();
    client.by_number(&npi("1987654328")).await.unwrap();
    client.by_number(&npi("1987654328")).await.unwrap();
    assert_eq!(recorded.queries.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn client_refuses_redirects_errors_and_slow_answers() {
    let recorded = Recorded::default();
    let base = serve(recorded.clone()).await;
    let lookup = npi("1234567893");

    let moved = Client::with_base_url(format!("{base}/moved/")).unwrap();
    let Err(RegistryError::Unavailable(message)) = moved.by_number(&lookup).await else {
        panic!("a redirect must not be followed");
    };
    assert!(message.contains("307"), "{message}");
    assert!(recorded.queries.lock().unwrap().is_empty());

    let down = Client::with_base_url(format!("{base}/down/")).unwrap();
    assert!(matches!(
        down.by_number(&lookup).await,
        Err(RegistryError::Unavailable(_))
    ));

    let slow = Client::with_config(ClientConfig {
        base_url: format!("{base}/slow/"),
        timeout: Duration::from_millis(200),
        ..ClientConfig::default()
    })
    .unwrap();
    let secret = SearchQuery {
        last_name: Some("Zebulonsecret".into()),
        ..query()
    };
    let Err(RegistryError::Unavailable(message)) = slow.search(&secret).await else {
        panic!("a slow answer must time out");
    };
    assert!(!message.contains("Zebulonsecret"), "{message}");
    assert!(!message.contains("127.0.0.1"), "{message}");
}

/// One real NPPES call. Run with `NPI_REGISTRY_LIVE=1`.
#[tokio::test]
async fn live_nppes_lookup() {
    if std::env::var("NPI_REGISTRY_LIVE").as_deref() != Ok("1") {
        return;
    }
    let client = Client::new().unwrap();
    let results = client
        .search(&SearchQuery {
            last_name: Some("smith".into()),
            state: Some("MA".into()),
            limit: Some(2),
            ..query()
        })
        .await
        .unwrap();
    assert!(!results.is_empty());
    let first = &results[0];
    let again = client.by_number(&first.npi).await.unwrap().unwrap();
    assert_eq!(&again, first);
    assert!(matches!(
        client
            .search(&SearchQuery {
                first_name: Some("a*".into()),
                city: Some("Boston".into()),
                ..query()
            })
            .await,
        Err(RegistryError::InvalidQuery(_))
    ));
}
