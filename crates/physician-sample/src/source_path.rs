//! Where a value sits in a record. Number preservation, the general view and layout binding all
//! name a value by [`physician_wire::source_path::SourcePath`], so a member named
//! `valueQuantity.value` and the `value` member of `valueQuantity` stay two values.
pub(crate) use physician_wire::source_path::{SourcePath, Step, nodes};

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
