//! Explicit model settings for synthetic physician servers.

pub fn environment() -> impl Iterator<Item = (String, String)> {
    [
        ("CHAT", "gpt-6.1-sol", "high"),
        ("BACKGROUND", "gpt-6-luna", "max"),
    ]
    .into_iter()
    .flat_map(|(role, model, effort)| {
        [
            ("MODEL", model),
            ("EFFORT", effort),
            ("CONTEXT_WINDOW_TOKENS", "272000"),
            ("INPUT_LIMIT_TOKENS", "922000"),
        ]
        .into_iter()
        .map(move |(suffix, value)| (format!("CODA_{role}_{suffix}"), value.to_owned()))
    })
}

/// The same accepted bindings as the synthetic site and worker environment.
pub fn roles() -> local_product::ModelRoles {
    let settings: Vec<_> = environment().collect();
    let values: Vec<_> = settings
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    local_product::model_roles("responses", &values).unwrap()
}

#[allow(
    dead_code,
    reason = "shared fixture used by the worker and acceptance suites"
)]
pub fn chat_recipe() -> harness::engine::recipe::RequestRecipe {
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    local_product::physician_chat::recipe(
        &roles().chat,
        local_product::physician_chat::INSTRUCTIONS,
        &skills,
    )
}

#[allow(
    dead_code,
    reason = "shared fixture used by the revision acceptance suite"
)]
pub fn revision_recipe() -> harness::engine::recipe::RequestRecipe {
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    local_product::runtime::reply::recipe(&roles().background, &skills)
}
