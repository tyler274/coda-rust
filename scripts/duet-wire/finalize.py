#!/usr/bin/env python3
"""Reproduce existing Rust generator compatibility adaptations, not hand edits.

OpenAPI Generator collapses discriminator aliases sharing a schema. Derive the
existing ActionsInner dispatch from the input mapping so no action is lost.
Keep the checked-in client configuration template across generator upgrades.
"""
import json
import html
from pathlib import Path
import re
import sys

spec = json.loads(Path(sys.argv[1]).read_text())
out = Path(sys.argv[2])
# The Rust README template omits authorization, although API pages link to each
# scheme's exact name. Derive the section and anchors from the composed contract.
readme = out / "README.md"
authorization = "\n## Authorization\n"
for name, scheme in spec["components"].get("securitySchemes", {}).items():
    authorization += f'\n<a id="{html.escape(name, quote=True)}"></a>\n\n### {name}\n\n'
    authorization += "```json\n" + json.dumps(scheme, indent=2) + "\n```\n"
readme_text = readme.read_text()
author = readme_text.index("\n## Author\n")
readme.write_text(readme_text[:author] + authorization + readme_text[author:])

# These emitted type links have no generated model page. Keep their type labels;
# fix the inline response's filename to match its actual generated model page.
for document in [readme, *(out / "docs").glob("*.md")]:
    documentation = re.sub(
        r"\[([^\]]+)\]\((?:serde_json::Value|SerdeJson__Value|Uuid__Uuid|Null)\.md\)",
        r"\1", document.read_text(),
    ).replace("(readDurableAccountContext_200_response.md)", "(ReadDurableAccountContext200Response.md)")
    # The new index has a blank footer; retain existing model-page spacing.
    if document == readme:
        documentation = documentation.rstrip() + "\n"
    # Generated endpoint index rows contain trailing spaces.
    document.write_text("\n".join(line.rstrip() for line in documentation.splitlines()) + "\n")
actions = out / "src/models/actions_inner.rs"
text = actions.read_text()
# The discriminator lives on ConversationMessage.actions.items in the reference.
schemas = spec["components"]["schemas"]
mappings = []


def walk(value):
    if isinstance(value, dict):
        discriminator = value.get("discriminator", {})
        mapping = discriminator.get("mapping", {})
        if discriminator.get("propertyName") == "kind" and "save_doctor_question" in mapping:
            mappings.append(mapping)
        for child in value.values():
            walk(child)
    elif isinstance(value, list):
        for child in value:
            walk(child)


walk(schemas)
if not mappings or any(mapping != mappings[0] for mapping in mappings):
    raise ValueError("ActionsInner discriminator mapping is absent or ambiguous")
# Preserve public enum ordering and Default from the existing wire contract.
order = ["save_doctor_question", "prepare_doctor_summary", "propose_doctor_page",
         "create_saved_routine", "update_saved_routine", "create_follow_up",
         "update_follow_up", "pause_follow_up", "resume_follow_up", "cancel_follow_up"]
mapping = mappings[0]
if set(mapping) != set(order):
    raise ValueError("ActionsInner contract changed: review compatibility generator")
variants = [(key, "".join(part.title() for part in key.split("_")), mapping[key].split("/")[-1]) for key in order]
header = text[:text.index("use crate::models;")]
status = text[text.index("///\n#[derive(Clone, Copy"):]  # existing emitted Status
body = "use crate::models;\nuse serde::de::Error as _;\nuse serde::{Deserialize, Serialize};\n\n"
body += "#[derive(Clone, Debug, PartialEq)]\npub enum ActionsInner {\n"
body += "".join(f"    {variant}(Box<models::{model}>),\n" for _, variant, model in variants)
body += "}\n\nimpl Default for ActionsInner {\n    fn default() -> Self {\n        Self::PrepareDoctorSummary(Default::default())\n    }\n}\n\n"
body += "impl Serialize for ActionsInner {\n    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n    where S: serde::Serializer,\n    {\n        match self {\n"
body += "".join(f"            Self::{variant}(value) => value.serialize(serializer),\n" for _, variant, _ in variants)
body += "        }\n    }\n}\n\n"
body += '''impl<'de> Deserialize<'de> for ActionsInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value.get("kind").and_then(|kind| kind.as_str()).unwrap_or("");
        fn parse<T, E>(value: serde_json::Value) -> Result<T, E>
        where T: serde::de::DeserializeOwned, E: serde::de::Error,
        { serde_json::from_value(value).map_err(E::custom) }
        match kind {
'''
body += "".join(f'            "{key}" => Ok(Self::{variant}(Box::new(parse(value)?))),\n' for key, variant, _ in variants)
body += '            other => Err(D::Error::custom(format!("unknown action kind {other}"))),\n        }\n    }\n}\n\n'
actions.write_text(header + body + status)
# SourcesInner variants themselves carry a required literal `kind`. Internally
# tagged serde enums consume it before deserializing the variant, making valid
# published native-fact sources fail with `missing field kind`. Untagged dispatch
# preserves the field; each variant's literal enum still validates the kind.
sources = out / "src/models/sources_inner.rs"
source_text = sources.read_text()
if '#[serde(tag = "kind")]' not in source_text:
    raise ValueError("SourcesInner generator shape changed; review compatibility")
sources.write_text(source_text.replace('#[serde(tag = "kind")]', '#[serde(untagged)]'))
lib = out / "src/lib.rs"
lib.write_text("#![allow(missing_docs)]\n" + lib.read_text())
# Pin the existing client support surface independently of generated DTOs.
template = Path(__file__).with_name("configuration.rs.template")
(out / "src/apis/configuration.rs").write_text(template.read_text())
(out / "Cargo.toml").write_text(Path(__file__).with_name("Cargo.toml.template").read_text())
