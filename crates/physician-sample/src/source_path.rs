//! Where a value sits in a record. Number preservation, the general view and layout binding all
//! name a value by its [`SourcePath`], so a member named `valueQuantity.value` and the `value`
//! member of `valueQuantity` stay two values.
use serde_json::Value;
use std::fmt;

/// One reference token of an RFC 6901 JSON Pointer, unescaped: an object member or an array
/// index.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Step {
    Member(String),
    Index(usize),
}

/// A value's path from the record root: the reference tokens of its RFC 6901 JSON Pointer,
/// each kept as the member or index it names in the record. Within one record two paths are
/// equal exactly when their pointers are.
///
/// It displays in the readable form [`physician_wire::SourceField::path`] carries:
/// `component[1].valueQuantity.value`, with a member whose name holds `.`, `[`, `]`, `"` or `\`,
/// or is empty, written as a quoted JSON string in brackets, such as `["valueQuantity.value"]`.
/// Two different paths never display alike.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SourcePath(Vec<Step>);

impl SourcePath {
    /// The record itself.
    pub(crate) fn root() -> Self {
        Self::default()
    }

    /// The member `name` of the object at this path.
    pub(crate) fn member(&self, name: &str) -> Self {
        let mut path = self.clone();
        path.push_member(name);
        path
    }

    /// The item at `index` of the array at this path.
    pub(crate) fn index(&self, index: usize) -> Self {
        let mut path = self.clone();
        path.push_index(index);
        path
    }

    pub(crate) fn push_member(&mut self, name: &str) {
        self.0.push(Step::Member(name.to_owned()));
    }

    pub(crate) fn push_index(&mut self, index: usize) {
        self.0.push(Step::Index(index));
    }

    /// The steps from the record root.
    pub(crate) fn steps(&self) -> &[Step] {
        &self.0
    }

    /// Whether this path is `path` or lies under it.
    pub(crate) fn is_under(&self, path: &SourcePath) -> bool {
        self.0.starts_with(&path.0)
    }

    /// The steps from `path` down to this one; empty when this is `path` or does not lie
    /// under it.
    pub(crate) fn below(&self, path: &SourcePath) -> &[Step] {
        self.0.strip_prefix(path.0.as_slice()).unwrap_or_default()
    }
}

impl fmt::Display for SourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (at, step) in self.0.iter().enumerate() {
            match step {
                Step::Index(index) => write!(f, "[{index}]")?,
                Step::Member(name) if plain(name) => {
                    if at > 0 {
                        f.write_str(".")?;
                    }
                    f.write_str(name)?;
                }
                // A JSON string: quoted, with `"` and `\` escaped, so it ends where it says.
                Step::Member(name) => write!(f, "[{}]", Value::from(name.as_str()))?,
            }
        }
        Ok(())
    }
}

/// Every value in `record`, objects and arrays included, with its path, in record order.
pub(crate) fn nodes(record: &Value) -> Vec<(SourcePath, &Value)> {
    fn visit<'r>(value: &'r Value, path: SourcePath, out: &mut Vec<(SourcePath, &'r Value)>) {
        match value {
            Value::Array(items) => {
                out.push((path.clone(), value));
                for (index, item) in items.iter().enumerate() {
                    visit(item, path.index(index), out);
                }
            }
            Value::Object(object) => {
                out.push((path.clone(), value));
                for (key, member) in object {
                    visit(member, path.member(key), out);
                }
            }
            _ => out.push((path, value)),
        }
    }
    let mut out = Vec::new();
    visit(record, SourcePath::root(), &mut out);
    out
}

/// Whether a member name reads unquoted without running into the next step.
fn plain(name: &str) -> bool {
    !name.is_empty() && !name.contains(['.', '[', ']', '"', '\\'])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_member_whose_name_reads_as_a_path_is_quoted() {
        let root = SourcePath::root();
        let nested = root.member("valueQuantity").member("value");
        let dotted = root.member("valueQuantity.value");
        assert_ne!(nested, dotted);
        assert_eq!(nested.to_string(), "valueQuantity.value");
        assert_eq!(dotted.to_string(), r#"["valueQuantity.value"]"#);
        assert_eq!(
            root.member("component").index(1).member("a[0]").to_string(),
            r#"component[1]["a[0]"]"#
        );
        assert_eq!(root.member("a").index(0).to_string(), "a[0]");
        assert_eq!(root.member("a").member("0").to_string(), "a.0");
        assert_eq!(
            root.member("").member("q\"]").to_string(),
            r#"[""]["q\"]"]"#
        );
        assert_eq!(root.member("a/b").member("c~d").to_string(), "a/b.c~d");
    }

    #[test]
    fn a_path_lies_under_its_own_ancestors_only() {
        let root = SourcePath::root();
        let quantity = root.member("valueQuantity");
        let value = quantity.member("value");
        assert!(value.is_under(&quantity));
        assert!(value.is_under(&value));
        assert!(!root.member("valueQuantity.value").is_under(&quantity));
        assert!(!root.member("valueQuantityX").is_under(&quantity));
        assert_eq!(value.below(&quantity), [Step::Member("value".into())]);
    }
}
