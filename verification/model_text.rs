use serde_json::Value;

pub fn assert_clean_model_text(text: &str) {
    let lower = text.to_ascii_lowercase();
    for label in [
        "ported from",
        "tool names adapted",
        "instructions version:",
        "library sha256",
        "model-request budget",
    ] {
        assert!(
            !lower.contains(label),
            "model-facing implementation label: {label}"
        );
    }
    for word in text.split(|c: char| !c.is_ascii_alphanumeric() && c != '-') {
        // Coda's ticket namespace. Clinical names such as PHQ-9, ICD-10 and
        // COVID-19, and protocol names such as UTF-8, are not ticket IDs.
        if let Some(number) = word.strip_prefix("ENG-") {
            assert!(
                number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()),
                "model-facing ticket ID: {word}"
            );
        }
    }
}

fn fields<'a>(schema: &'a Value, names: &mut Vec<&'a str>) {
    match schema {
        Value::Object(object) => {
            if let Some(properties) = object.get("properties").and_then(Value::as_object) {
                names.extend(properties.keys().map(String::as_str));
            }
            for value in object.values() {
                fields(value, names);
            }
        }
        Value::Array(values) => {
            for value in values {
                fields(value, names);
            }
        }
        _ => (),
    }
}

/// Guard skill text against the tools a job offers. It checks tool and field
/// names and banned labels; it cannot check what the prose tells the model to do.
pub fn assert_skill_tools(package: &Value, tools: &[Value], result_fields: &[&str]) {
    let mut allowed_fields = result_fields.to_vec();
    for tool in tools {
        fields(&tool["parameters"], &mut allowed_fields);
    }
    for skill in package["skills"].as_array().unwrap() {
        let mut bodies = vec![skill["content"].as_str().unwrap()];
        if let Some(resources) = skill["resources"].as_array() {
            bodies.extend(
                resources
                    .iter()
                    .map(|resource| resource["content"].as_str().unwrap()),
            );
        }
        for body in bodies {
            assert_clean_model_text(body);
            for word in body.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
                if !word.contains('_') || allowed_fields.contains(&word) {
                    continue;
                }
                assert!(
                    tools.iter().any(|tool| tool["name"] == word),
                    "listed skill {} names unavailable tool or field {word}",
                    skill["name"]
                );
            }
        }
    }
}
