use npi_registry::{ClinicianKind, FixtureDirectory};
use physician_auth::{AuthStore, Membership, MemoryAuthStore, NpiAssociationError as Error};

#[tokio::test]
async fn npi_association_is_unique_clearable_and_never_authority() {
    let store = MemoryAuthStore::new();
    let mut clinician = FixtureDirectory::sample().clinicians().remove(0);
    clinician.person.as_mut().unwrap().last = "García López".into();
    store.seed_account("one", "one", "one@test", "Dr. GARCIÁ LOPEZ, MD", false);
    store.seed_account("two", "two", "two@test", "Garcia Lopez", false);
    let first = store.account_by_email("one@test").await.unwrap();
    let second = store.account_by_email("two@test").await.unwrap();
    let result = store
        .claim_npi(&first.identity, clinician.clone(), 42, false)
        .await
        .unwrap();
    assert!(result.name_matched);
    assert_eq!(result.lookup_checked_at, 42);
    let after = store.account_by_identity(&first.identity).await.unwrap();
    assert_eq!(after.admin, first.admin);
    assert_eq!(after.active, first.active);
    assert_eq!(after.sessions_ended_before, first.sessions_ended_before);
    assert_eq!(
        store.membership(&first.identity, "unassigned").await,
        Membership::Denied
    );
    assert_eq!(
        store
            .claim_npi(&second.identity, clinician.clone(), 43, false)
            .await,
        Err(Error::Conflict)
    );
    store.clear_npi(&first.identity).await.unwrap();
    assert!(
        store
            .account_by_identity(&first.identity)
            .await
            .unwrap()
            .npi
            .is_none()
    );
    store
        .claim_npi(&second.identity, clinician, 44, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn npi_name_policy_rejects_organizations_substrings_and_non_admin_override() {
    let store = MemoryAuthStore::new();
    let mut clinician = FixtureDirectory::sample().clinicians().remove(0);
    clinician.person.as_mut().unwrap().last = "Li".into();
    store.seed_account("p", "p", "p@test", "Alice Smith", false);
    store.seed_account("p", "admin", "admin@test", "Other Name", true);
    let user = store.account_by_email("p@test").await.unwrap().identity;
    let admin = store.account_by_email("admin@test").await.unwrap().identity;
    assert_eq!(
        store.claim_npi(&user, clinician.clone(), 1, false).await,
        Err(Error::NameMismatch)
    );
    assert_eq!(
        store.claim_npi(&user, clinician.clone(), 1, true).await,
        Err(Error::Forbidden)
    );
    let association = store
        .claim_npi(&admin, clinician.clone(), 1, true)
        .await
        .unwrap();
    assert!(!association.name_matched);
    clinician.kind = ClinicianKind::Organization;
    assert_eq!(
        store.claim_npi(&admin, clinician, 1, true).await,
        Err(Error::NotIndividual)
    );
    store.deactivate("admin@test").await;
    assert_eq!(store.clear_npi(&admin).await, Err(Error::Inactive));
}
