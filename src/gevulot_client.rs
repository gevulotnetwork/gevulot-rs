use crate::base_client::{BaseClient, GasConfig};
use crate::error::Result;
use crate::gov_client::GovClient;
use crate::pin_client::PinClient;
use crate::sudo_client::SudoClient;
use crate::task_client::TaskClient;
use crate::worker_client::WorkerClient;
use crate::workflow_client::WorkflowClient;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GevulotClient exposes all gevulot specific functionality
/// * pins
/// * tasks
/// * workers
/// * workflows
#[derive(Debug, Clone)]
pub struct GevulotClient {
    pub pins: PinClient,
    pub tasks: TaskClient,
    pub workflows: WorkflowClient,
    pub workers: WorkerClient,
    pub gov: GovClient,
    pub sudo: SudoClient,
    // raw access to base functionality so we don't lock out ourselves
    pub base_client: Arc<RwLock<BaseClient>>,
}

impl From<BaseClient> for GevulotClient {
    fn from(base_client: BaseClient) -> Self {
        let base_client = Arc::new(RwLock::new(base_client));
        Self {
            pins: PinClient::new(base_client.clone()),
            tasks: TaskClient::new(base_client.clone()),
            workflows: WorkflowClient::new(base_client.clone()),
            workers: WorkerClient::new(base_client.clone()),
            gov: GovClient::new(base_client.clone()),
            sudo: SudoClient::new(base_client.clone()),
            base_client,
        }
    }
}

/// Builder for GevulotClient
pub struct GevulotClientBuilder {
    endpoint: String,
    chain_id: Option<String>,
    denom: Option<String>,
    gas_config: GasConfig,
    mnemonic: Option<String>,
    private_key: Option<String>,
    password: Option<String>,
}

impl Default for GevulotClientBuilder {
    /// Provides default values for GevulotClientBuilder
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:9090".to_string(),
            chain_id: None,
            denom: None,
            gas_config: GasConfig::Dynamic {
                gas_price: 0.025,
                gas_multiplier: 1.2,
            },
            mnemonic: None,
            private_key: None,
            password: None,
        }
    }
}

impl GevulotClientBuilder {
    /// Creates a new GevulotClientBuilder with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the endpoint for the GevulotClient
    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    /// Sets the chain ID for the GevulotClient
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_string());
        self
    }

    /// Sets the token denomination for the GevulotClient
    pub fn denom(mut self, denom: &str) -> Self {
        self.denom = Some(denom.to_string());
        self
    }

    /// Sets the gas price for the GevulotClient
    pub fn gas_price(mut self, gas_price: f64) -> Self {
        match self.gas_config {
            GasConfig::Dynamic { gas_multiplier, .. } => {
                self.gas_config = GasConfig::Dynamic {
                    gas_price,
                    gas_multiplier,
                };
            }
            GasConfig::Fixed { gas_limit, .. } => {
                self.gas_config = GasConfig::Fixed {
                    gas_limit,
                    gas_price,
                };
            }
        }
        self
    }

    /// Sets the gas multiplier for the GevulotClient
    pub fn gas_multiplier(mut self, gas_multiplier: f64) -> Self {
        match self.gas_config {
            GasConfig::Dynamic { gas_price, .. } => {
                self.gas_config = GasConfig::Dynamic {
                    gas_price,
                    gas_multiplier,
                };
            }
            GasConfig::Fixed { gas_price, .. } => {
                self.gas_config = GasConfig::Dynamic {
                    gas_price,
                    gas_multiplier,
                };
            }
        }
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        match self.gas_config {
            GasConfig::Dynamic { gas_price, .. } => {
                self.gas_config = GasConfig::Fixed {
                    gas_price,
                    gas_limit,
                };
            }
            GasConfig::Fixed { gas_price, .. } => {
                self.gas_config = GasConfig::Fixed {
                    gas_price,
                    gas_limit,
                };
            }
        }
        self
    }

    /// Sets the mnemonic for the GevulotClient
    pub fn mnemonic(mut self, mnemonic: &str) -> Self {
        self.mnemonic = Some(mnemonic.to_string());
        self
    }

    /// Sets the hex-encoded private key for the GevulotClient
    pub fn private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    /// Sets the password for the GevulotClient
    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    /// Builds the GevulotClient with the provided configuration
    pub async fn build(self) -> Result<GevulotClient> {
        // Create a new BaseClient with the provided endpoint, gas price, and gas multiplier
        let base_client = Arc::new(RwLock::new(
            BaseClient::new(&self.endpoint, self.gas_config).await?,
        ));

        // If chain ID is provided, set it in the BaseClient
        if let Some(chain_id) = self.chain_id {
            base_client.write().await.chain_id = chain_id;
        }

        // If token denomination is provided, set it in the BaseClient
        if let Some(denom) = self.denom {
            base_client.write().await.denom = denom;
        }

        // If a mnemonic is provided, set it in the BaseClient
        if let Some(mnemonic) = self.mnemonic {
            base_client
                .write()
                .await
                .set_mnemonic(&mnemonic, self.password.as_deref())?;
        } else if let Some(private_key) = self.private_key {
            base_client.write().await.set_private_key(&private_key)?;
        }

        // Create and return the GevulotClient with the initialized clients
        Ok(GevulotClient {
            pins: PinClient::new(base_client.clone()),
            tasks: TaskClient::new(base_client.clone()),
            workflows: WorkflowClient::new(base_client.clone()),
            workers: WorkerClient::new(base_client.clone()),
            gov: GovClient::new(base_client.clone()),
            sudo: SudoClient::new(base_client.clone()),
            base_client,
        })
    }
}
