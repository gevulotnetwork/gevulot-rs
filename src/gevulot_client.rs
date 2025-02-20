use crate::base_client::BaseClient;
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

/// Builder for GevulotClient
pub struct GevulotClientBuilder {
    endpoint: String,
    gas_price: f64,
    gas_multiplier: f64,
    mnemonic: Option<String>,
    private_key: Option<String>,
    password: Option<String>,
}

impl Default for GevulotClientBuilder {
    /// Provides default values for GevulotClientBuilder
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:9090".to_string(),
            gas_price: 0.025,
            gas_multiplier: 1.2,
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

    /// Sets the gas price for the GevulotClient
    pub fn gas_price(mut self, gas_price: f64) -> Self {
        self.gas_price = gas_price;
        self
    }

    /// Sets the gas multiplier for the GevulotClient
    pub fn gas_multiplier(mut self, gas_multiplier: f64) -> Self {
        self.gas_multiplier = gas_multiplier;
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
            BaseClient::new(&self.endpoint, self.gas_price, self.gas_multiplier).await?,
        ));

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
