//! A chat turn that ends the work through its finish, for the runtime
//! integration tests.
use local_product::physician_sources::citations::FINISH_TOOL;
use local_product::runtime::engine;

/// `text` as narration, then the same text handed in through the finish as
/// an answer that cites no source. At a limit only the narration is kept.
pub fn answered(text: &str) -> Vec<engine::Item> {
    vec![
        engine::Item::assistant(text),
        engine::Item::Call(engine::Invocation {
            id: "finish".into(),
            name: FINISH_TOOL.into(),
            arguments: serde_json::json!({
                "answer": text,
                "quotes": [],
                "descriptions": [],
                "basis": "Fixture basis.",
                "cites_none": true
            })
            .to_string(),
        }),
    ]
}
