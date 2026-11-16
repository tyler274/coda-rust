//! How a physician process reaches the model: the transport its environment
//! selects, the role bindings, and the provider that runs each request. The
//! worker and the local rounds command both use this module, so they reach
//! the model the same way.
use std::sync::Arc;

use crate::hosting;
use harness::engine::Cancellation;
use harness::engine::RunLimits;
use harness::engine::recipe::Provider;
use harness::provider::ResponsesProvider;
use local_product::runtime::{PublicationTransaction, Store};
use telemetry::genai;

/// `get(key)` trimmed, when it holds anything.
pub fn configured(get: &impl Fn(&str) -> Option<String>, key: &str) -> Option<String> {
    get(key)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn role_environment(get: &impl Fn(&str) -> Option<String>) -> Vec<(String, String)> {
    local_product::MODEL_SETTING_KEYS
        .into_iter()
        .filter_map(|key| configured(get, key).map(|value| (key.to_owned(), value)))
        .collect()
}

/// Credentials stay in the process; both choices run the same durable job.
pub enum ModelTransport {
    /// A Responses endpoint and its key.
    Responses {
        /// The endpoint.
        endpoint: String,
        /// The key.
        key: String,
    },
    /// A developer's own ChatGPT plan.
    DeveloperPlan,
}

impl ModelTransport {
    fn adapter(&self) -> &'static str {
        match self {
            Self::Responses { .. } => "openai-responses",
            Self::DeveloperPlan => "chatgpt-plan",
        }
    }

    /// The transport the environment selects, with the telemetry origin of
    /// the calls it makes.
    pub fn from_env(
        get: &impl Fn(&str) -> Option<String>,
        hosted: bool,
    ) -> Result<(Self, genai::Origin), hosting::StartupError> {
        Ok(
            if let Some(port) = configured(get, "PHYSICIAN_RESPONSES_STUB_PORT") {
                if hosted {
                    return Err(
                        "hosted physician worker requires the public Responses endpoint".into(),
                    );
                }
                let port: u16 = port
                    .parse()
                    .map_err(|_| "invalid local Responses stub port")?;
                (
                    Self::Responses {
                        endpoint: format!("http://127.0.0.1:{port}/responses"),
                        key: "local-proof".to_owned(),
                    },
                    genai::Origin::Simulation,
                )
            } else {
                match if hosted {
                    local_product::ModelExecution::Api
                } else {
                    local_product::model_execution(
                        configured(get, "CODA_LOCAL_MODEL_EXECUTION").as_deref(),
                    )?
                } {
                    local_product::ModelExecution::Subscription => {
                        local_product::chatgpt::require_sign_in()
                            .map_err(|error| error.to_string())?;
                        (Self::DeveloperPlan, genai::Origin::Real)
                    }
                    local_product::ModelExecution::Api => {
                        let key = local_product::open_ai_api_key(
                            configured(get, "CODA_ALLOW_MODEL_CALLS").as_deref(),
                            configured(get, "OPENAI_API_KEY").as_deref(),
                        )?;
                        (
                            Self::Responses {
                                endpoint: harness::responses::ENDPOINT.to_owned(),
                                key,
                            },
                            genai::Origin::Real,
                        )
                    }
                    local_product::ModelExecution::Disabled => {
                        return Err(
                        "physician worker requires CODA_LOCAL_MODEL_EXECUTION=subscription or api"
                            .into(),
                    );
                    }
                }
            },
        )
    }
}

/// Transport and model configuration shared by chat, reply revision and
/// patient rounds.
pub struct ModelSource {
    /// The transport.
    pub transport: ModelTransport,
    /// The model binding of each role.
    pub roles: local_product::ModelRoles,
}

impl ModelSource {
    /// `transport` with the role bindings the environment sets.
    pub fn from_env(
        transport: ModelTransport,
        get: &impl Fn(&str) -> Option<String>,
    ) -> Result<Self, hosting::StartupError> {
        let owned = role_environment(get);
        let environment: Vec<_> = owned
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();
        let roles = local_product::model_roles(transport.adapter(), &environment)
            .map_err(|error| error.to_string())?;
        Ok(Self { transport, roles })
    }

    /// The provider that runs each request over this transport.
    pub async fn provider(
        &self,
        quota: Arc<dyn harness::provider::QuotaGate>,
    ) -> Result<ModelProvider, harness::engine::Cause> {
        let provider = match &self.transport {
            ModelTransport::Responses { endpoint, key } => {
                ResponsesProvider::new(endpoint.clone(), key.clone(), quota.clone())?
            }
            ModelTransport::DeveloperPlan => {
                let token = local_product::chatgpt::access_token().await?;
                ResponsesProvider::chatgpt_plan(token, quota.clone())?
            }
        };
        Ok(ModelProvider {
            provider,
            developer_plan: matches!(self.transport, ModelTransport::DeveloperPlan),
            quota,
        })
    }

    /// Whether this transport may read only the synthetic practice: a
    /// developer's own ChatGPT plan never reaches patient data.
    pub fn synthetic_only(&self) -> bool {
        match self.transport {
            ModelTransport::DeveloperPlan => true,
            ModelTransport::Responses { .. } => false,
        }
    }
}

/// Refuses `workspace` when the transport may read only the synthetic
/// practice and the workspace holds a source outside it. Reads on the
/// caller's transaction, which already holds one of the process's pooled
/// connections, rather than waiting for a second.
pub async fn source_boundary(
    synthetic_only: bool,
    tx: &PublicationTransaction,
    workspace: &str,
) -> Result<(), String> {
    if !synthetic_only {
        return Ok(());
    }
    let outside = Store::has_sources_outside_in(tx, workspace, crate::synthetic_chart::SOURCE_ID)
        .await
        .map_err(|e| e.to_string())?;
    if outside {
        return Err("Developer ChatGPT plans can read only the synthetic practice".into());
    }
    Ok(())
}

/// The plan grant may refresh during a long durable run. Request execution and
/// quota handling remain with the shared Responses provider.
pub struct ModelProvider {
    /// The provider that sends each request.
    pub provider: ResponsesProvider,
    /// Whether the transport is a developer's own ChatGPT plan.
    pub developer_plan: bool,
    /// The quota gate the provider waits on.
    pub quota: Arc<dyn harness::provider::QuotaGate>,
}

impl Provider for ModelProvider {
    async fn ready(&mut self) -> Result<(), harness::engine::Cause> {
        self.provider.ready().await
    }

    async fn exchange(
        &mut self,
        request: &harness::engine::recipe::Request,
        cancel: &Cancellation,
        limits: &RunLimits,
        observer: &mut dyn harness::engine::recipe::Observer,
    ) -> Result<harness::engine::Turn, harness::engine::Cause> {
        tracing::Span::current().record("gen_ai.request.model", request.model.as_str());
        if self.developer_plan {
            let token = local_product::chatgpt::access_token().await?;
            self.provider = ResponsesProvider::chatgpt_plan(token, self.quota.clone())?;
        }
        self.provider
            .exchange(request, cancel, limits, observer)
            .await
    }
}
