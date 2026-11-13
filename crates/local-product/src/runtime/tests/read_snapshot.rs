//! Successive chat work reuses the recipe and skill package frozen at acceptance.
use super::*;

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn successive_chat_work_reuses_frozen_recipe_and_package() {
    struct Answers {
        opened: bool,
        instruction: String,
        guidance: String,
        cache_key: harness::PromptCacheKey,
    }
    impl Provider for Answers {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<Turn, engine::Cause> {
            assert_eq!(request.cache_key.as_ref(), Some(&self.cache_key));
            assert!(
                request
                    .instructions
                    .as_str()
                    .contains(self.instruction.as_str())
            );
            if !self.opened {
                self.opened = true;
                return Ok(Turn {
                    items: vec![engine::Item::Call(Invocation {
                        id: "guidance".into(),
                        name: harness::skills::SKILL_VIEW.into(),
                        arguments: serde_json::json!({"name":"patient-record-retrieval"})
                            .to_string(),
                    })],
                    usage: Default::default(),
                    receipt: serde_json::json!({}),
                });
            }
            assert!(request.items.iter().any(|item| matches!(item,
                engine::Item::MaterializedResult { call_id, output }
                if call_id == "guidance" && output.as_str().contains(self.guidance.as_str()))));
            Ok(Turn {
                items: vec![engine::Item::assistant("Reviewed the available guidance.")],
                usage: Default::default(),
                receipt: serde_json::json!({}),
            })
        }
    }
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let scope = delivery_scope(&Uuid::new_v4().to_string(), "frozen-inputs");
    let instruction = format!("Recipe-{}", Uuid::new_v4());
    let guidance = format!("Guidance-{}", Uuid::new_v4());
    let mut package = serde_json::to_value(test_skills()).unwrap();
    package["skills"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|skill| skill["name"] == "patient-record-retrieval")
        .unwrap()["content"] = serde_json::json!(guidance);
    let skills: PhysicianSkills = serde_json::from_value(package).unwrap();
    let mut recipe = test_chat_recipe();
    recipe.instructions.push_str(&instruction);
    let other_chat = Scope {
        chat: "other-frozen-input-chat".into(),
        ..scope.clone()
    };
    for (scope, input) in [
        (&scope, "first"),
        (&scope, "second"),
        (&other_chat, "third"),
    ] {
        // A fresh store must resolve the same frozen inputs after restart.
        let reopened = Store::connect(&url).await.unwrap();
        reopened
            .accept_authorized(
                scope,
                &delivery_input(input),
                vec![5; 32],
                &skills,
                &recipe,
                None,
            )
            .await
            .unwrap();
        let claim = reopened.claim(scope, 60).await.unwrap().unwrap();
        PhysicianHost {
            store: reopened,
            claim,
            authority: OpenAuthority,
            shown_steps: crate::runtime::ShownSteps::new(()),
        }
        .run(
            &mut Answers {
                opened: false,
                instruction: instruction.clone(),
                guidance: guidance.clone(),
                cache_key: scope.cache_key("chords-chat"),
            },
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    }
    let row = conversation::Entity::find_by_id(keys(&scope))
        .one(&store.db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        row.state["bindings"].as_object().unwrap().len(),
        2,
        "the two accepted inputs must have completed distinct works"
    );
    assert!(
        row.state["bindings"]
            .as_object()
            .unwrap()
            .values()
            .all(|reference| reference["id"].is_string()),
        "accepted works must reference one saved recipe instead of copying its body"
    );
    assert!(
        row.state["skills"].is_string(),
        "accepted work must reference its saved package"
    );
    use sea_orm::sea_query::{Alias, Asterisk, Query};
    for table in [
        "physician_runtime_recipes",
        "physician_runtime_skill_packages",
    ] {
        let query = Query::select()
            .expr_as(Expr::col(Asterisk).count(), Alias::new("saved"))
            .from(Alias::new(table))
            .and_where(Expr::col(Alias::new("practice")).eq(scope.practice.as_str()))
            .and_where(Expr::col(Alias::new("physician")).eq(scope.physician.as_str()))
            .to_owned();
        let count = store.db.query_one(&query).await.unwrap().unwrap();
        assert_eq!(count.try_get::<i64>("", "saved").unwrap(), 1, "{table}");
    }
}
