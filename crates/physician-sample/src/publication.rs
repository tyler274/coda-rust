//! Publication delegates to the existing physician-auth authority owner.
//! Its fence covers direct store mutations as well as HTTP mutations.
use crate::auth::{AuthContext, now};

pub(crate) async fn publish(
    ctx: &AuthContext,
    workspace: &str,
    effect: &mut (dyn FnMut() -> Result<(), String> + Send),
) -> Result<(), String> {
    ctx.auth
        .store
        .publish_authorized(&ctx.token, &ctx.identity, workspace, now(), effect)
        .await
}
