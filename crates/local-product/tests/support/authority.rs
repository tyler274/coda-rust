//! The publication authority shared by the runtime integration tests.
use local_product::runtime::{Authority, PublicationTransaction, Scope, async_trait};
use physician_auth::{PhysicianIdentity, PostgresAuthStore};

/// Holds the real auth owner's locks on the publication transaction.
pub struct Locks;
#[async_trait]
impl Authority for Locks {
    async fn check(
        &mut self,
        _: &PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            hash.ok_or("missing session")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())
    }
}
