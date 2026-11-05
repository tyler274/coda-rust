use super::{Error, NETWORK, PROJECT, Result, receipt::Receipt};
use serde::Deserialize;
use std::{collections::BTreeMap, process::Command, time::Duration};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Instance {
    pub name: String,
    pub project: String,
    pub region: String,
    pub state: String,
    pub database_version: String,
    pub instance_type: String,
    #[serde(default)]
    pub create_time: String,
    pub settings: Settings,
    #[serde(default)]
    pub ip_addresses: Vec<Address>,
    pub master_instance_name: Option<String>,
    #[serde(default)]
    pub replica_names: Vec<String>,
}
impl Instance {
    pub fn staging_network(&self) -> bool {
        let network = &self.settings.ip_configuration.private_network;
        network == NETWORK || network == &format!("https://www.googleapis.com/compute/v1/{NETWORK}")
    }
    pub fn private_only(&self) -> bool {
        !self.settings.ip_configuration.ipv4_enabled
            && self
                .settings
                .ip_configuration
                .authorized_networks
                .is_empty()
            && !self.ip_addresses.is_empty()
            && self.ip_addresses.iter().all(|ip| ip.kind == "PRIVATE")
    }
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Settings {
    pub ip_configuration: IpConfiguration,
    pub storage_auto_resize: bool,
    pub storage_auto_resize_limit: String,
    #[serde(default)]
    pub deletion_protection_enabled: bool,
    #[serde(default)]
    pub user_labels: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IpConfiguration {
    pub ipv4_enabled: bool,
    pub ssl_mode: String,
    pub private_network: String,
    #[serde(default)]
    pub authorized_networks: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, Deserialize)]
pub(super) struct Address {
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Clone, Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Backup {
    pub id: String,
    pub instance: String,
    pub status: String,
    pub end_time: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Operation {
    pub name: String,
    pub target_id: String,
    pub target_project: String,
    pub operation_type: String,
    pub status: String,
    pub error: Option<serde_json::Value>,
}

/// The orchestrator owns safety checks; this replaceable boundary owns cloud I/O only.
pub(super) trait Cloud {
    fn instances(&mut self) -> Result<Vec<Instance>>;
    fn instance(&mut self, target: &str) -> Result<Instance>;
    fn backup(&mut self, source: &str, id: &str) -> Result<Backup>;
    fn create(&mut self, receipt: &Receipt) -> Result<Operation>;
    fn restore(&mut self, receipt: &Receipt) -> Result<Operation>;
    fn operation(&mut self, id: &str) -> Result<Operation>;
    fn delete(&mut self, target: &str) -> Result<Operation>;
    fn pause(&mut self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

pub(super) struct Gcloud;
impl Gcloud {
    fn json<T: serde::de::DeserializeOwned>(&self, args: &[&str]) -> Result<T> {
        // Bound the CLI too: operation polling alone cannot bound a hung credential refresh.
        // coreutils timeout is supplied by the dev shell, not downloaded by this command.
        let output = Command::new("timeout")
            .args(["--signal=TERM", "--kill-after=5s", "120s", "gcloud"])
            .args(args)
            .args(["--project", PROJECT, "--quiet", "--format=json"])
            .env("CLOUDSDK_CORE_DISABLE_PROMPTS", "1")
            .output()?;
        if !output.status.success() {
            return Err(Error::Cloud {
                code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }
        Ok(serde_json::from_slice(&output.stdout)?)
    }
}
impl Cloud for Gcloud {
    fn instances(&mut self) -> Result<Vec<Instance>> {
        self.json(&["sql", "instances", "list"])
    }
    fn instance(&mut self, target: &str) -> Result<Instance> {
        self.json(&["sql", "instances", "describe", target])
    }
    fn backup(&mut self, source: &str, id: &str) -> Result<Backup> {
        self.json(&["sql", "backups", "describe", id, "--instance", source])
    }
    fn create(&mut self, receipt: &Receipt) -> Result<Operation> {
        let args = create_args(receipt);
        self.json(&args.iter().map(String::as_str).collect::<Vec<_>>())
    }
    fn restore(&mut self, receipt: &Receipt) -> Result<Operation> {
        self.json(&[
            "sql",
            "backups",
            "restore",
            &receipt.backup.id,
            "--backup-instance",
            &receipt.source,
            "--restore-instance",
            &receipt.target,
            "--async",
        ])
    }
    fn operation(&mut self, id: &str) -> Result<Operation> {
        self.json(&["sql", "operations", "describe", id])
    }
    fn delete(&mut self, target: &str) -> Result<Operation> {
        self.json(&["sql", "instances", "delete", target, "--async"])
    }
}

pub(super) fn create_args(receipt: &Receipt) -> Vec<String> {
    let labels = format!(
        "coda-purpose=restore-drill,coda-product={},coda-run={}",
        receipt.product.name(),
        receipt.run_id
    );
    // No bindings, SQL users, Cloud Run revisions, subscriptions, worker processes or messages.
    // Existing release identities retain their instance-scoped grants; this adds no IAM.
    [
        "beta",
        "sql",
        "instances",
        "create",
        &receipt.target,
        "--database-version=POSTGRES_17",
        "--edition=ENTERPRISE",
        "--tier=db-custom-1-3840",
        "--availability-type=ZONAL",
        "--region",
        &receipt.region,
        "--network",
        NETWORK,
        "--no-assign-ip",
        "--no-deletion-protection",
        "--storage-size=20",
        "--storage-auto-increase",
        "--storage-auto-increase-limit=50",
        "--ssl-mode=ENCRYPTED_ONLY",
        "--labels",
        &labels,
        "--async",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
