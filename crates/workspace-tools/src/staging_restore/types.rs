use super::{Error, Product, Result, refuse};
use serde::{Deserialize, Serialize};

/// A disposable instance name, associated with the product encoded in its prefix.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub(super) struct RestoreTarget {
    name: String,
    product: Product,
}
impl RestoreTarget {
    pub(super) fn new(product: Product, name: String) -> Result<Self> {
        let prefix = format!("rust-restore-{}-", product.name());
        if !name.starts_with(&prefix)
            || name.len() <= prefix.len()
            || name.len() > 63
            || !name
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            || !name
                .as_bytes()
                .last()
                .is_some_and(u8::is_ascii_alphanumeric)
        {
            return Err(refuse(
                "target must be a fresh rust-restore-<product>-<run> instance",
            ));
        }
        Ok(Self { name, product })
    }
    pub(super) fn as_str(&self) -> &str {
        &self.name
    }
    pub(super) fn product(&self) -> Product {
        self.product
    }
}
impl TryFrom<String> for RestoreTarget {
    type Error = Error;
    fn try_from(name: String) -> Result<Self> {
        let product = if name.starts_with("rust-restore-physician-") {
            Product::Physician
        } else if name.starts_with("rust-restore-duet-") {
            Product::Duet
        } else {
            return Err(refuse(
                "target must be a fresh rust-restore-<product>-<run> instance",
            ));
        };
        Self::new(product, name)
    }
}
impl From<RestoreTarget> for String {
    fn from(target: RestoreTarget) -> Self {
        target.name
    }
}

/// Cloud SQL IDs remain strings: leading zeroes are part of the supplied identity.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub(super) struct BackupId(String);
impl BackupId {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for BackupId {
    type Error = Error;
    fn try_from(id: String) -> Result<Self> {
        if id.is_empty() || !id.bytes().all(|c| c.is_ascii_digit()) {
            return Err(refuse("backup must be a numeric Cloud SQL backup ID"));
        }
        Ok(Self(id))
    }
}
impl From<BackupId> for String {
    fn from(id: BackupId) -> Self {
        id.0
    }
}

/// Preserve the provider's existing hex/hyphen grammar without UUID normalization.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub(super) struct SqlOperationId(String);
impl SqlOperationId {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for SqlOperationId {
    type Error = Error;
    fn try_from(id: String) -> Result<Self> {
        if id.len() != 36
            || !id.bytes().enumerate().all(|(index, byte)| {
                if [8, 13, 18, 23].contains(&index) {
                    byte == b'-'
                } else {
                    byte.is_ascii_hexdigit()
                }
            })
        {
            return Err(refuse("invalid Cloud SQL operation ID"));
        }
        Ok(Self(id))
    }
}
impl From<SqlOperationId> for String {
    fn from(id: SqlOperationId) -> Self {
        id.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OperationKind {
    Create,
    RestoreVolume,
    Delete,
}
impl TryFrom<&str> for OperationKind {
    type Error = Error;
    fn try_from(kind: &str) -> Result<Self> {
        match kind {
            "CREATE" => Ok(Self::Create),
            "RESTORE_VOLUME" => Ok(Self::RestoreVolume),
            "DELETE" => Ok(Self::Delete),
            _ => Err(refuse("unknown operation type")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OperationStatus {
    Pending,
    Running,
    Done,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("restore refused: unknown operation status")]
pub(crate) struct UnknownOperationStatus;

impl TryFrom<&str> for OperationStatus {
    type Error = UnknownOperationStatus;
    fn try_from(status: &str) -> std::result::Result<Self, Self::Error> {
        match status {
            "PENDING" => Ok(Self::Pending),
            "RUNNING" => Ok(Self::Running),
            "DONE" => Ok(Self::Done),
            _ => Err(UnknownOperationStatus),
        }
    }
}
