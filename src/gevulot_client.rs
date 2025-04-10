use crate::base_client::{BaseClient, FuelPolicy};
use crate::error::Result;
use crate::gov_client::GovClient;
use crate::pin_client::PinClient;
use crate::sudo_client::SudoClient;
use crate::task_client::TaskClient;
use crate::worker_client::WorkerClient;
use crate::workflow_client::WorkflowClient;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main client for interacting with the Gevulot network.
///
/// The `GevulotClient` is the primary entry point for applications that need to interact
/// with the Gevulot network. It provides access to specialized client modules for managing
/// various aspects of the network, such as pins, tasks, workers, and workflows.
///
/// The client is thread-safe and can be safely shared across multiple threads or async tasks.
/// It uses an internal read-write lock to ensure concurrent access to the underlying base client.
///
/// # Specialized Clients
///
/// * `pins` - For managing content pins (data availability)
/// * `tasks` - For creating and monitoring computational tasks
/// * `workflows` - For managing sequences of interdependent tasks
/// * `workers` - For registering and managing compute workers
/// * `gov` - For interacting with the governance system
/// * `sudo` - For performing administrative operations (requires sudo privileges)
///
/// # Examples
///
/// Creating a client and querying workers:
///
/// ```
/// use gevulot_rs::GevulotClientBuilder;
///
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a client using the builder pattern
///     let mut client = GevulotClientBuilder::new()
///         .endpoint("http://localhost:9090")
///         .mnemonic("your mnemonic seed phrase")
///         .build()
///         .await?;
///
///     // Use the workers client to query all workers
///     let workers = client.workers.list().await?;
///     println!("Found {} workers", workers.len());
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GevulotClient {
    /// Client for managing content pins in the Gevulot network.
    ///
    /// Use this to create, query, and delete pins that ensure data availability.
    pub pins: PinClient,
    
    /// Client for managing computational tasks in the Gevulot network.
    ///
    /// Use this to create, query, and monitor individual compute jobs.
    pub tasks: TaskClient,
    
    /// Client for managing workflows in the Gevulot network.
    ///
    /// Use this to create and monitor sequences of interconnected tasks.
    pub workflows: WorkflowClient,
    
    /// Client for managing worker nodes in the Gevulot network.
    ///
    /// Use this to register, query, and manage compute resource providers.
    pub workers: WorkerClient,
    
    /// Client for interacting with the governance system.
    ///
    /// Use this to query and vote on governance proposals.
    pub gov: GovClient,
    
    /// Client for performing administrative operations.
    ///
    /// Use this for operations that require sudo privileges in the network.
    pub sudo: SudoClient,
    
    /// Direct access to the underlying base client.
    ///
    /// This provides low-level access to the Gevulot blockchain node.
    pub base_client: Arc<RwLock<BaseClient>>,
}

/// Converts a BaseClient into a GevulotClient.
///
/// This implementation allows for easy creation of a GevulotClient from
/// a pre-configured BaseClient instance. It wraps the BaseClient in a
/// thread-safe container and initializes all the specialized client modules.
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

/// Builder for constructing a properly configured GevulotClient.
///
/// The `GevulotClientBuilder` provides a fluent interface for configuring and constructing
/// a `GevulotClient` with custom settings. It handles all the details of connecting to the
/// Gevulot network, configuring gas parameters, and setting up authentication.
///
/// # Default Configuration
///
/// By default, the builder is configured with:
/// * Endpoint: `http://127.0.0.1:9090`
/// * Gas policy: Dynamic with price 0.025 and multiplier 1.2
/// * Chain ID and token denomination: Default values from the base client
///
/// # Examples
///
/// Basic usage with default endpoint:
///
/// ```
/// use gevulot_rs::GevulotClientBuilder;
///
/// async fn create_client() -> Result<(), Box<dyn std::error::Error>> {
///     let client = GevulotClientBuilder::new()
///         .mnemonic("your mnemonic seed phrase")
///         .build()
///         .await?;
///     
///     // Use the client...
///     
///     Ok(())
/// }
/// ```
///
/// Advanced configuration with custom gas parameters:
///
/// ```
/// use gevulot_rs::GevulotClientBuilder;
///
/// async fn create_custom_client() -> Result<(), Box<dyn std::error::Error>> {
///     let client = GevulotClientBuilder::new()
///         .endpoint("https://gevulot-testnet.example.com:9090")
///         .chain_id("gevulot-testnet-1")
///         .denom("utest")
///         .gas_price(0.05)
///         .gas_multiplier(1.5)
///         .mnemonic("your mnemonic seed phrase")
///         .build()
///         .await?;
///     
///     // Use the client...
///     
///     Ok(())
/// }
/// ```
pub struct GevulotClientBuilder {
    /// The gRPC endpoint URL for the Gevulot node.
    endpoint: String,
    
    /// Optional custom chain ID for the Gevulot network.
    chain_id: Option<String>,
    
    /// Optional custom token denomination for transaction fees.
    denom: Option<String>,
    
    /// Gas policy configuration for transaction fee estimation.
    gas_config: FuelPolicy,
    
    /// Optional mnemonic seed phrase for account authentication.
    mnemonic: Option<String>,
    
    /// Optional hex-encoded private key for account authentication.
    private_key: Option<String>,
    
    /// Optional password for the mnemonic (BIP39 passphrase).
    password: Option<String>,
}

impl Default for GevulotClientBuilder {
    /// Provides default values for GevulotClientBuilder.
    ///
    /// This implementation sets sensible defaults for connecting to a local
    /// Gevulot node with standard gas parameters.
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:9090".to_string(),
            chain_id: None,
            denom: None,
            gas_config: FuelPolicy::Dynamic {
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
    /// Creates a new GevulotClientBuilder with default values.
    ///
    /// This method initializes a builder with sensible defaults for connecting
    /// to a local Gevulot node. You can then customize the configuration using
    /// the various setter methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the endpoint URL for the Gevulot node.
    ///
    /// # Parameters
    ///
    /// * `endpoint` - The gRPC endpoint URL (e.g., "http://localhost:9090")
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .endpoint("http://gevulot-node.example.com:9090");
    /// ```
    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    /// Sets the chain ID for the Gevulot network.
    ///
    /// This is used to specify which network to connect to when there are multiple
    /// Gevulot networks (e.g., mainnet, testnet).
    ///
    /// # Parameters
    ///
    /// * `chain_id` - The chain identifier (e.g., "gevulot-testnet-1")
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .chain_id("gevulot-testnet-1");
    /// ```
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_string());
        self
    }

    /// Sets the token denomination for transaction fees.
    ///
    /// # Parameters
    ///
    /// * `denom` - The token denomination (e.g., "ucredit")
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .denom("ucredit");
    /// ```
    pub fn denom(mut self, denom: &str) -> Self {
        self.denom = Some(denom.to_string());
        self
    }

    /// Sets the gas price for transaction fees.
    ///
    /// # Parameters
    ///
    /// * `gas_price` - The price of gas in the native token denomination
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .gas_price(0.025);
    /// ```
    pub fn gas_price(mut self, gas_price: f64) -> Self {
        match self.gas_config {
            FuelPolicy::Dynamic { gas_multiplier, .. } => {
                self.gas_config = FuelPolicy::Dynamic {
                    gas_price,
                    gas_multiplier,
                };
            }
            FuelPolicy::Fixed { gas_limit, .. } => {
                self.gas_config = FuelPolicy::Fixed {
                    gas_limit,
                    gas_price,
                };
            }
        }
        self
    }

    /// Sets the gas multiplier for dynamic gas estimation.
    ///
    /// This value is used as a multiplier on the simulated gas to provide a
    /// buffer against estimation errors. For example, a value of 1.2 adds a
    /// 20% margin to the estimated gas.
    ///
    /// Note: This setting switches the gas policy to `Dynamic` if it was
    /// previously `Fixed`.
    ///
    /// # Parameters
    ///
    /// * `gas_multiplier` - The multiplier to apply to simulated gas (e.g., 1.2)
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .gas_multiplier(1.5); // Add 50% buffer to simulated gas
    /// ```
    pub fn gas_multiplier(mut self, gas_multiplier: f64) -> Self {
        match self.gas_config {
            FuelPolicy::Dynamic { gas_price, .. } => {
                self.gas_config = FuelPolicy::Dynamic {
                    gas_price,
                    gas_multiplier,
                };
            }
            FuelPolicy::Fixed { gas_price, .. } => {
                self.gas_config = FuelPolicy::Dynamic {
                    gas_price,
                    gas_multiplier,
                };
            }
        }
        self
    }

    /// Sets a fixed gas limit for all transactions.
    ///
    /// This switches the gas policy to `Fixed`, which uses the same gas limit
    /// for all transactions without simulation.
    ///
    /// # Parameters
    ///
    /// * `gas_limit` - The fixed gas limit to use for all transactions
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .gas_limit(200_000); // Use a fixed gas limit of 200,000 for all transactions
    /// ```
    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        match self.gas_config {
            FuelPolicy::Dynamic { gas_price, .. } => {
                self.gas_config = FuelPolicy::Fixed {
                    gas_price,
                    gas_limit,
                };
            }
            FuelPolicy::Fixed { gas_price, .. } => {
                self.gas_config = FuelPolicy::Fixed {
                    gas_price,
                    gas_limit,
                };
            }
        }
        self
    }

    /// Sets the mnemonic seed phrase for account authentication.
    ///
    /// The mnemonic is used to derive the private key for signing transactions.
    /// This is the most common way to configure account authentication.
    ///
    /// # Parameters
    ///
    /// * `mnemonic` - The BIP-39 mnemonic seed phrase (typically 12 or 24 words)
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .mnemonic("word1 word2 ... word12");
    /// ```
    pub fn mnemonic(mut self, mnemonic: &str) -> Self {
        self.mnemonic = Some(mnemonic.to_string());
        self
    }

    /// Sets the hex-encoded private key for account authentication.
    ///
    /// This is an alternative to providing a mnemonic seed phrase.
    ///
    /// # Parameters
    ///
    /// * `private_key` - The hex-encoded private key (with optional 0x prefix)
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .private_key("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
    /// ```
    pub fn private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    /// Sets the password (BIP-39 passphrase) for the mnemonic.
    ///
    /// This adds an additional layer of security to the mnemonic.
    /// Only relevant when using a mnemonic for authentication.
    ///
    /// # Parameters
    ///
    /// * `password` - The password or passphrase to use with the mnemonic
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// let builder = GevulotClientBuilder::new()
    ///     .mnemonic("word1 word2 ... word12")
    ///     .password("my_secure_password");
    /// ```
    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    /// Builds the GevulotClient with the configured settings.
    ///
    /// This method establishes a connection to the Gevulot network using
    /// the configured endpoint and authentication credentials, and constructs
    /// a fully initialized `GevulotClient` ready for use.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `GevulotClient` on success,
    /// or an `Error` if the connection or authentication fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::GevulotClientBuilder;
    ///
    /// async fn create_client() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = GevulotClientBuilder::new()
    ///         .endpoint("http://localhost:9090")
    ///         .mnemonic("your mnemonic seed phrase")
    ///         .build()
    ///         .await?;
    ///     
    ///     // Use the client...
    ///     
    ///     Ok(())
    /// }
    /// ```
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
