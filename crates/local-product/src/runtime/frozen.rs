//! Immutable execution inputs shared by a physician's accepted work.
//! The database canonicalizes JSON before assigning content IDs, so object key
//! order cannot create duplicate inputs.
use super::entities::{frozen_recipe, skill_package};
use super::{PhysicianSkills, RequestRecipe, Scope, conversion_error, error};
use harness::engine::recipe::RequestBinding;
use sea_orm::{
    ConnectionTrait, DbErr, EntityTrait, QuerySelect, Set,
    sea_query::{Alias, Expr, ExprTrait, Func, OnConflict, Query},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Shared provider policy, cache binding, and optional starting chart version
/// accepted for this work.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingRef {
    id: String,
    cache_key: Option<harness::PromptCacheKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pinned_source_version: Option<u64>,
}

impl BindingRef {
    /// Attaches the workspace chart observation version pinned when chat work began.
    pub(super) fn with_pinned_source_version(mut self, version: u64) -> Self {
        self.pinned_source_version = Some(version);
        self
    }

    /// Workspace chart observation version pinned when chat work began, if recorded.
    pub(super) fn pinned_source_version(&self) -> Option<u64> {
        self.pinned_source_version
    }
}

/// Identity of one exact packaged skill selection within its physician owner.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SkillPackageId(String);

async fn identity(db: &impl ConnectionTrait, body: &Value) -> Result<String, DbErr> {
    let canonical = Expr::value(body.clone())
        .cast_as(Alias::new("jsonb"))
        .cast_as(Alias::new("text"));
    let bytes = Func::cust(Alias::new("convert_to"))
        .arg(canonical)
        .arg("UTF8");
    let digest = Func::cust(Alias::new("sha256")).arg(bytes);
    let query = Query::select()
        .expr_as(
            Func::cust(Alias::new("encode")).arg(digest).arg("hex"),
            Alias::new("id"),
        )
        .to_owned();
    db.query_one(&query)
        .await?
        .ok_or_else(|| error("frozen input identity missing"))?
        .try_get("", "id")
}

pub(super) async fn keep_owner_binding(
    db: &impl ConnectionTrait,
    practice: &str,
    owner: &str,
    binding: &RequestBinding,
) -> Result<BindingRef, DbErr> {
    let cache_key = binding.cache_key.clone();
    let body = serde_json::to_value(&binding.recipe)
        .map_err(conversion_error("RequestRecipe", "frozen recipe body"))?;
    let id = identity(db, &body).await?;
    frozen_recipe::Entity::insert(frozen_recipe::ActiveModel {
        practice: Set(practice.to_owned()),
        physician: Set(owner.to_owned()),
        id: Set(id.clone()),
        body: Set(body),
    })
    .on_conflict(
        OnConflict::columns([
            frozen_recipe::Column::Practice,
            frozen_recipe::Column::Physician,
            frozen_recipe::Column::Id,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(db)
    .await?;
    Ok(BindingRef {
        id,
        cache_key,
        pinned_source_version: None,
    })
}

pub(super) async fn keep_binding(
    db: &impl ConnectionTrait,
    scope: &Scope,
    binding: &RequestBinding,
) -> Result<BindingRef, DbErr> {
    scope.validate()?;
    keep_owner_binding(db, &scope.practice, &scope.physician, binding).await
}

pub(super) async fn keep_owner_skills(
    db: &impl ConnectionTrait,
    practice: &str,
    owner: &str,
    skills: &PhysicianSkills,
) -> Result<SkillPackageId, DbErr> {
    let body = serde_json::to_value(skills)
        .map_err(conversion_error("PhysicianSkills", "skill package body"))?;
    let id = identity(db, &body).await?;
    skill_package::Entity::insert(skill_package::ActiveModel {
        practice: Set(practice.to_owned()),
        physician: Set(owner.to_owned()),
        id: Set(id.clone()),
        body: Set(body),
    })
    .on_conflict(
        OnConflict::columns([
            skill_package::Column::Practice,
            skill_package::Column::Physician,
            skill_package::Column::Id,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec_without_returning(db)
    .await?;
    Ok(SkillPackageId(id))
}

pub(super) async fn keep_skills(
    db: &impl ConnectionTrait,
    scope: &Scope,
    skills: &PhysicianSkills,
) -> Result<SkillPackageId, DbErr> {
    scope.validate()?;
    keep_owner_skills(db, &scope.practice, &scope.physician, skills).await
}

pub(super) async fn owner_binding(
    db: &impl ConnectionTrait,
    practice: &str,
    owner: &str,
    reference: &BindingRef,
) -> Result<RequestBinding, DbErr> {
    let body = frozen_recipe::Entity::find_by_id((
        practice.to_owned(),
        owner.to_owned(),
        reference.id.clone(),
    ))
    .select_only()
    .column(frozen_recipe::Column::Body)
    .into_tuple::<Value>()
    .one(db)
    .await?
    .ok_or_else(|| error("accepted recipe is missing"))?;
    let recipe: RequestRecipe = serde_json::from_value(body)
        .map_err(conversion_error("frozen recipe body", "RequestRecipe"))?;
    Ok(recipe.bind(reference.cache_key.clone()))
}

pub(super) async fn binding(
    db: &impl ConnectionTrait,
    scope: &Scope,
    reference: &BindingRef,
) -> Result<RequestBinding, DbErr> {
    scope.validate()?;
    owner_binding(db, &scope.practice, &scope.physician, reference).await
}

pub(super) async fn owner_skills(
    db: &impl ConnectionTrait,
    practice: &str,
    owner: &str,
    id: &SkillPackageId,
) -> Result<PhysicianSkills, DbErr> {
    let body =
        skill_package::Entity::find_by_id((practice.to_owned(), owner.to_owned(), id.0.clone()))
            .select_only()
            .column(skill_package::Column::Body)
            .into_tuple::<Value>()
            .one(db)
            .await?
            .ok_or_else(|| error("accepted skill package is missing"))?;
    serde_json::from_value(body).map_err(conversion_error("skill package body", "PhysicianSkills"))
}

pub(super) async fn skills(
    db: &impl ConnectionTrait,
    scope: &Scope,
    id: &SkillPackageId,
) -> Result<PhysicianSkills, DbErr> {
    scope.validate()?;
    owner_skills(db, &scope.practice, &scope.physician, id).await
}
