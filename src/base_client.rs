use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{SimulateResponse, Tx};
use cosmos_sdk_proto::prost::{Message, Name};
use cosmos_sdk_proto::tendermint::types::Block;
use cosmrs::{auth::BaseAccount, Coin};
use tonic::transport::{Channel, ClientTlsConfig};

use crate::error::{Error, Result};
use crate::signer::GevulotSigner;

/// Client type for querying Cosmos Auth module endpoints.
/// 
/// This client is used to query account information from the blockchain.
type AuthQueryClient<T> = cosmrs::proto::cosmos::auth::v1beta1::query_client::QueryClient<T>;

/// Client type for querying Cosmos Bank module endpoints.
/// 
/// This client is used to query token balances and supply information.
type BankQueryClient<T> = cosmrs::proto::cosmos::bank::v1beta1::query_client::QueryClient<T>;

/// Client type for querying Cosmos Governance module endpoints.
/// 
/// This client is used to query proposal information and voting status.
type GovQueryClient<T> = cosmrs::proto::cosmos::gov::v1beta1::query_client::QueryClient<T>;

/// Client type for querying Gevulot-specific module endpoints.
/// 
/// This client is used to query Gevulot-specific entities like workers, pins, and tasks.
type GevulotQueryClient<T> = crate::proto::gevulot::gevulot::query_client::QueryClient<T>;

/// Client type for interacting with the transaction service.
/// 
/// This client is used to simulate, broadcast, and query transactions.
type TxServiceClient<T> = cosmrs::proto::cosmos::tx::v1beta1::service_client::ServiceClient<T>;

/// Client type for querying Tendermint RPC endpoints.
/// 
/// This client is used to query blockchain information like blocks and consensus state.
type TendermintClient<T> =
    cosmrs::proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient<T>;

/// Default chain ID for the Gevulot network.
/// 
/// This value is used when creating a new client unless overridden.
pub const DEFAULT_CHAIN_ID: &str = "gevulot";

/// Default token denomination for the Gevulot network.
/// 
/// This is the smallest unit of the native token, used for gas fees and transactions.
pub const DEFAULT_TOKEN_DENOM: &str = "ucredit";

/// Core client implementation for interacting with the Gevulot blockchain.
/// 
/// The `BaseClient` provides a foundation for all interactions with the Gevulot network,
/// including:
/// 
/// - Account and balance queries
/// - Transaction construction and signing
/// - Transaction broadcasting and simulation
/// - Block queries and monitoring
/// 
/// It manages connection to the blockchain node, transaction signing, and sequence tracking
/// for the configured account.
/// 
/// # Examples
/// 
/// ```
/// use gevulot_rs::base_client::{BaseClient, FuelPolicy};
/// 
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a client with dynamic gas estimation
///     let mut client = BaseClient::new(
///         "http://localhost:9090",
///         FuelPolicy::Dynamic {
///             gas_price: 0.025,
///             gas_multiplier: 1.2
///         }
///     ).await?;
///     
///     // Configure the client with a mnemonic seed phrase
///     client.set_mnemonic("your mnemonic seed phrase", None)?;
///     
///     // Get the address
///     let address = client.address.clone();
///     
///     // Query account balance if address is available
///     if let Some(addr) = address {
///         let balance = client.get_account_balance(&addr).await?;
///         println!("Balance: {}", balance);
///     }
///     
///     Ok(())
/// }
/// ```
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct BaseClient {
    /// Client for querying the Auth module (account information)
    pub auth_client: AuthQueryClient<Channel>,
    
    /// Client for querying the Bank module (token balances and transfers)
    pub bank_client: BankQueryClient<Channel>,
    
    /// Client for querying the Gevulot module (workers, pins, tasks)
    pub gevulot_client: GevulotQueryClient<Channel>,
    
    /// Client for querying the Gov module (governance proposals)
    pub gov_client: GovQueryClient<Channel>,
    
    /// Client for querying Tendermint RPC endpoints (blocks, validators)
    pub tendermint_client: TendermintClient<Channel>,
    
    /// Client for transaction services (simulate, broadcast, query)
    pub tx_client: TxServiceClient<Channel>,

    /// Gas policy configuration for transaction fee estimation
    fuel_policy: FuelPolicy,
    
    /// Token denomination used for transactions and queries
    pub denom: String,
    
    /// Chain ID used for transaction signing
    pub chain_id: String,

    /// Blockchain address of the configured account
    pub address: Option<String>,
    
    /// Public key of the configured account
    pub pub_key: Option<cosmrs::crypto::PublicKey>,
    
    /// Private key of the configured account (not included in Debug output)
    #[derivative(Debug = "ignore")]
    priv_key: Option<cosmrs::crypto::secp256k1::SigningKey>,

    /// Account sequence number used for transaction ordering
    pub account_sequence: Option<u64>,
}

/// Gas policy configuration for transaction fee estimation.
/// 
/// This enum defines how transaction gas limits are determined:
/// - `Fixed`: Uses a predefined gas limit for all transactions
/// - `Dynamic`: Estimates gas through transaction simulation and applies a multiplier
#[derive(Debug)]
pub enum FuelPolicy {
    /// Fixed gas limit for all transactions.
    /// 
    /// # Parameters
    /// 
    /// * `gas_price` - The price of gas in the native token denomination.
    /// * `gas_limit` - The fixed gas limit to use for all transactions.
    Fixed { gas_price: f64, gas_limit: u64 },
    
    /// Dynamic gas estimation based on transaction simulation.
    /// 
    /// # Parameters
    /// 
    /// * `gas_price` - The price of gas in the native token denomination.
    /// * `gas_multiplier` - A multiplier applied to the simulated gas (e.g., 1.2 adds 20% margin).
    Dynamic { gas_price: f64, gas_multiplier: f64 },
}

impl BaseClient {
    /// Creates a new instance of BaseClient with connection to the specified endpoint.
    ///
    /// This method establishes a connection to the Gevulot blockchain node at the
    /// provided endpoint, configuring all necessary client components for interaction
    /// with the network. It uses exponential backoff with jitter for connection retries.
    ///
    /// # Parameters
    ///
    /// * `endpoint` - The gRPC endpoint URL of the Gevulot node (e.g., "http://localhost:9090").
    /// * `fuel_policy` - The gas policy configuration for transaction fee estimation.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new instance of `BaseClient` on success,
    /// or an `Error` if the connection fails after retries.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::base_client::{BaseClient, FuelPolicy};
    ///
    /// async fn create_client() -> Result<BaseClient, gevulot_rs::error::Error> {
    ///     BaseClient::new(
    ///         "http://localhost:9090",
    ///         FuelPolicy::Dynamic {
    ///             gas_price: 0.025,
    ///             gas_multiplier: 1.2
    ///         }
    ///     ).await
    /// }
    /// ```
    pub async fn new(endpoint: &str, fuel_policy: FuelPolicy) -> Result<Self> {
        use rand::Rng;
        use tokio::time::{sleep, Duration};

        let mut retries = 5;
        let mut delay = Duration::from_secs(1);

        // Attempt to create a channel with retries and exponential backoff
        let channel = loop {
            match Channel::from_shared(endpoint.to_owned())
                .map_err(|e| crate::error::Error::RpcConnectionError(e.to_string()))?
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .map_err(|e| crate::error::Error::RpcConnectionError(e.to_string()))?
                .connect()
                .await
            {
                Ok(channel) => break channel,
                Err(_) if retries > 0 => {
                    retries -= 1;
                    let jitter: u64 = rand::thread_rng().gen_range(0..1000);
                    sleep(delay + Duration::from_millis(jitter)).await;
                    delay *= 2;
                }
                Err(e) => return Err(e.into()),
            }
        };

        // Initialize the BaseClient with the created channel
        Ok(Self {
            auth_client: AuthQueryClient::new(channel.clone()),
            bank_client: BankQueryClient::new(channel.clone()),
            gevulot_client: GevulotQueryClient::new(channel.clone()),
            gov_client: GovQueryClient::new(channel.clone()),
            tendermint_client: TendermintClient::new(channel.clone()),
            tx_client: TxServiceClient::new(channel),
            denom: DEFAULT_TOKEN_DENOM.to_string(),
            chain_id: DEFAULT_CHAIN_ID.to_string(),
            fuel_policy,
            address: None,
            pub_key: None,
            priv_key: None,
            account_sequence: None,
        })
    }

    /// Configures the client with a pre-initialized signer.
    ///
    /// This method sets up the client with a signer that provides the address,
    /// public key, and private key needed for transaction signing.
    ///
    /// # Parameters
    ///
    /// * `signer` - A pre-initialized `GevulotSigner` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::{base_client::BaseClient, signer::GevulotSigner};
    ///
    /// fn configure_client(mut client: BaseClient) -> BaseClient {
    ///     let signer = GevulotSigner::from_mnemonic("your mnemonic", None).unwrap();
    ///     client.set_signer(signer);
    ///     client
    /// }
    /// ```
    pub fn set_signer(&mut self, signer: GevulotSigner) {
        self.address = Some(signer.0.public_address.to_string());
        self.pub_key = Some(signer.0.public_key);
        self.priv_key = Some(signer.0.private_key);
    }

    /// Configures the client with a mnemonic seed phrase.
    ///
    /// This method derives a private key from the provided mnemonic seed phrase
    /// and configures the client with the resulting signer.
    ///
    /// # Parameters
    ///
    /// * `mnemonic` - The BIP-39 mnemonic seed phrase (typically 12 or 24 words).
    /// * `password` - Optional password for additional security (BIP-39 passphrase).
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the key derivation process.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::base_client::BaseClient;
    ///
    /// fn configure_client_with_mnemonic(mut client: BaseClient) -> Result<(), gevulot_rs::error::Error> {
    ///     client.set_mnemonic("word1 word2 ... word12", None)
    /// }
    /// ```
    pub fn set_mnemonic(&mut self, mnemonic: &str, password: Option<&str>) -> Result<()> {
        let signer = GevulotSigner::from_mnemonic(mnemonic, password)?;
        self.set_signer(signer);
        Ok(())
    }

    /// Configures the client with a hex-encoded private key.
    ///
    /// This method sets up the client with a signer derived from the provided 
    /// hex-encoded private key.
    ///
    /// # Parameters
    ///
    /// * `hex_key` - The hex-encoded private key string (with optional 0x prefix).
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the key parsing process.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::base_client::BaseClient;
    ///
    /// fn configure_client_with_private_key(mut client: BaseClient) -> Result<(), gevulot_rs::error::Error> {
    ///     client.set_private_key("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
    /// }
    /// ```
    pub fn set_private_key(&mut self, hex_key: &str) -> Result<()> {
        let key_bytes = hex::decode(hex_key.trim_start_matches("0x"))?;
        let signing_key = cosmrs::crypto::secp256k1::SigningKey::from_slice(&key_bytes)?;
        let signer = GevulotSigner::from_signing_key(signing_key)?;
        self.set_signer(signer);
        Ok(())
    }

    /// Retrieves account information for a given address.
    ///
    /// This method queries the blockchain for the account associated with the specified
    /// address, returning detailed account information including sequence number
    /// and account number.
    ///
    /// # Parameters
    ///
    /// * `address` - The blockchain address to query, in bech32 format.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `BaseAccount` information on success,
    /// or an `Error` if the account cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::base_client::BaseClient;
    ///
    /// async fn get_account_info(
    ///     mut client: BaseClient,
    ///     address: &str
    /// ) -> Result<(), gevulot_rs::error::Error> {
    ///     let account = client.get_account(address).await?;
    ///     println!("Account number: {}", account.account_number);
    ///     println!("Sequence: {}", account.sequence);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account(&mut self, address: &str) -> Result<BaseAccount> {
        let request = cosmrs::proto::cosmos::auth::v1beta1::QueryAccountRequest {
            address: address.to_owned(),
        };
        let response = self.auth_client.account(request).await?;
        if let Some(cosmrs::Any { type_url: _, value }) = response.into_inner().account {
            let base_account = BaseAccount::try_from(
                cosmrs::proto::cosmos::auth::v1beta1::BaseAccount::decode(value.as_ref())?,
            )?;

            Ok(base_account)
        } else {
            Err("Can't load the associated account.".into())
        }
    }

    /// Retrieves the token balance for a given address.
    ///
    /// This method queries the blockchain for the balance of the specified address
    /// in the configured token denomination.
    ///
    /// # Parameters
    ///
    /// * `address` - The blockchain address to query, in bech32 format.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Coin` representing the balance on success,
    /// or an `Error` if the balance cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::base_client::BaseClient;
    ///
    /// async fn get_balance(
    ///     mut client: BaseClient,
    ///     address: &str
    /// ) -> Result<(), gevulot_rs::error::Error> {
    ///     let balance = client.get_account_balance(address).await?;
    ///     println!("Balance: {} {}", balance.amount, balance.denom);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_balance(&mut self, address: &str) -> Result<Coin> {
        let request = cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest {
            address: address.to_string(),
            denom: self.denom.clone(),
        };
        let response = self.bank_client.balance(request).await?;

        if let Some(coin) = response.into_inner().balance {
            let coin = Coin::try_from(coin)?;
            Ok(coin)
        } else {
            Err(Error::Unknown(format!(
                "Can't query the account balance for {}",
                address
            )))
        }
    }

    /// Transfer tokens to a given address.
    ///
    /// # Arguments
    ///
    /// * `to_address` - The address of the receiving account.
    /// * `amount` - Amount of coins to transfer.
    ///
    /// # Returns
    ///
    /// An empty result or an error.
    pub async fn token_transfer(&mut self, to_address: &str, amount: u128) -> Result<()> {
        let address = self.address.as_ref().ok_or("Address not set")?.to_owned();
        let msg = cosmrs::proto::cosmos::bank::v1beta1::MsgSend {
            from_address: address,
            to_address: to_address.to_string(),
            amount: vec![Coin {
                denom: self.denom.parse()?,
                amount,
            }
            .into()],
        };

        log::debug!("token transfer msg: {:?}", msg);

        self.send_msg_sync::<_, cosmrs::proto::cosmos::bank::v1beta1::MsgSendResponse>(
            msg,
            "token transfer",
        )
        .await?;

        Ok(())
    }

    /// Retrieves the account details including account number and sequence.
    ///
    /// # Returns
    ///
    /// A Result containing a tuple of account number and sequence or an error.
    async fn get_account_details(&mut self) -> Result<(u64, u64)> {
        let address = self.address.as_ref().ok_or("Address not set")?.to_owned();
        let account = self.get_account(&address).await?;
        let sequence = match self.account_sequence {
            Some(sequence) if sequence > account.sequence => sequence,
            _ => {
                self.account_sequence = Some(account.sequence);
                account.sequence
            }
        };
        Ok((account.account_number, sequence))
    }

    /// Creates a signed transaction document with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be included in the transaction.
    /// * `memo` - The memo to be included in the transaction.
    /// * `account_number` - The account number.
    /// * `sequence` - The sequence number.
    /// * `gas_limit` - The gas limit for the transaction.
    ///
    /// # Returns
    ///
    /// A Result containing the raw transaction and its bytes or an error.
    async fn create_signed_tx<M: Message + Name>(
        &self,
        msg: &M,
        memo: &str,
        account_number: u64,
        sequence: u64,
        gas_limit: u64,
        gas_price: f64,
    ) -> Result<(cosmrs::tx::Raw, Vec<u8>)> {
        let msg = cosmrs::Any::from_msg(msg)?;
        let chain_id: cosmrs::tendermint::chain::Id = self
            .chain_id
            .parse()
            .map_err(|_| Error::Parse("fail".to_string()))?;

        let tx_body = cosmrs::tx::BodyBuilder::new().msg(msg).memo(memo).finish();
        let signer_info = cosmrs::tx::SignerInfo::single_direct(self.pub_key, sequence);

        let gas_per_ucredit = (1.0 / gas_price).floor() as u128;
        let fee = cosmrs::tx::Fee::from_amount_and_gas(
            Coin {
                denom: self.denom.parse()?,
                amount: (gas_limit as u128 / gas_per_ucredit) + 1,
            },
            gas_limit,
        );

        let auth_info = signer_info.auth_info(fee);
        let sign_doc = cosmrs::tx::SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;
        let tx_raw = sign_doc.sign(self.priv_key.as_ref().ok_or("Private key not set")?)?;
        let tx_bytes = tx_raw.to_bytes()?;

        Ok((tx_raw, tx_bytes))
    }

    /// Simulates a message to estimate gas usage.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be simulated.
    /// * `memo` - The memo to be included in the transaction.
    /// * `account_number` - The account number.
    /// * `sequence` - The sequence number.
    ///
    /// # Returns
    ///
    /// A Result containing the SimulateResponse or an error.
    pub async fn simulate_msg<M: Message + Name>(
        &mut self,
        msg: M,
        memo: &str,
        account_number: u64,
        sequence: u64,
        gas_price: f64,
    ) -> Result<SimulateResponse> {
        // Use a default gas limit for simulation
        let gas_limit = 100_000u64;
        let (_, tx_bytes) = self
            .create_signed_tx(&msg, memo, account_number, sequence, gas_limit, gas_price)
            .await?;

        let mut tx_client = self.tx_client.clone();

        #[allow(deprecated)]
        // we have to specify the tx field in this raw struct initialization to avoid a compilation warning
        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::SimulateRequest { tx_bytes, tx: None };

        let response = tx_client.simulate(request).await?;
        Ok(response.into_inner())
    }

    /// Sends a message and returns the transaction hash.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be sent.
    /// * `memo` - The memo to be included in the transaction.
    ///
    /// # Returns
    ///
    /// A Result containing the transaction hash or an error.
    pub async fn send_msg<M: Message + Name + Clone>(
        &mut self,
        msg: M,
        memo: &str,
    ) -> Result<String> {
        let (account_number, sequence) = self.get_account_details().await?;
        let (gas_limit, gas_price) = match self.fuel_policy {
            FuelPolicy::Fixed {
                gas_limit,
                gas_price,
            } => (gas_limit, gas_price),
            FuelPolicy::Dynamic {
                gas_multiplier,
                gas_price,
            } => {
                // Use simulate_msg to estimate gas
                log::debug!("Estimating gas limit...");
                let simulate_response = self
                    .simulate_msg(msg.clone(), memo, account_number, sequence, gas_price)
                    .await?;
                log::debug!("simulate_response: {:#?}", simulate_response);
                let gas_info = simulate_response.gas_info.ok_or("Failed to get gas info")?;
                let gas_limit = (gas_info.gas_used * ((gas_multiplier * 10000.0) as u64)) / 10000; // Adjust gas limit based on simulation
                (gas_limit, gas_price)
            }
        };

        log::debug!("Using gas limit: {}", gas_limit);

        // Create and sign the transaction with the calculated gas limit
        let (_, tx_bytes) = self
            .create_signed_tx(&msg, memo, account_number, sequence, gas_limit, gas_price)
            .await?;

        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest {
            tx_bytes,
            mode: 2, // BROADCAST_MODE_SYNC -> Wait for the tx to be processed, but not in-block
        };

        let resp = self.tx_client.broadcast_tx(request).await?;
        let resp = resp.into_inner();
        log::debug!("broadcast_tx response: {:#?}", resp);

        let tx_response = resp.tx_response.ok_or("Tx response not found")?;
        Self::assert_tx_success(&tx_response)?;

        // Bump up the local account sequence after successful tx.
        self.account_sequence = Some(sequence + 1);
        let hash = tx_response.txhash;
        Ok(hash)
    }

    /// Sends a message and waits for the transaction to be included in a block.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be sent.
    /// * `memo` - The memo to be included in the transaction.
    ///
    /// # Returns
    ///
    /// A Result containing the response message or an error.
    pub async fn send_msg_sync<M: Message + Name + Clone, R: Message + Default>(
        &mut self,
        msg: M,
        memo: &str,
    ) -> Result<R> {
        let hash = self.send_msg(msg, memo).await?;
        self.wait_for_tx(&hash, Some(tokio::time::Duration::from_secs(10)))
            .await?;
        let tx_response: TxResponse = self.get_tx_response(&hash).await?;
        Self::assert_tx_success(&tx_response)?;
        let tx_msg_data = cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxMsgData::decode(
            &*hex::decode(tx_response.data)?,
        )?;
        if tx_msg_data.msg_responses.is_empty() {
            Err(Error::Unknown("no response message".to_string()))
        } else {
            let msg_response = &tx_msg_data.msg_responses[0];
            Ok(R::decode(&msg_response.value[..])?)
        }
    }

    /// Checks if Tx did not failed with non-zero code.
    ///
    /// # Arguments
    ///
    /// * `tx_response` - TxResponse.
    ///
    /// # Returns
    ///
    /// An empty Result or a Tx error.
    fn assert_tx_success(tx_response: &TxResponse) -> Result<()> {
        let (tx_hash, tx_code, raw_log) = (
            tx_response.txhash.to_owned(),
            tx_response.code,
            tx_response.raw_log.to_owned(),
        );
        if tx_code != 0 {
            return Err(Error::Tx(tx_hash, tx_code, raw_log));
        }

        Ok(())
    }

    /// Retrieves the latest block from the blockchain.
    ///
    /// # Returns
    ///
    /// A Result containing the latest Block or an error.
    pub async fn current_block(&mut self) -> Result<Block> {
        let request = cosmrs::proto::cosmos::base::tendermint::v1beta1::GetLatestBlockRequest {};
        let response = self.tendermint_client.get_latest_block(request).await?;
        let block: Block = response.into_inner().block.ok_or("Block not found")?;
        Ok(block)
    }

    /// Retrieves a block by its height.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the block to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the Block or an error.
    pub async fn get_block_by_height(&mut self, height: i64) -> Result<Block> {
        let request =
            cosmrs::proto::cosmos::base::tendermint::v1beta1::GetBlockByHeightRequest { height };
        let response = self.tendermint_client.get_block_by_height(request).await?;
        let block = response.into_inner().block.ok_or("Block not found")?;
        Ok(block)
    }

    /// Waits for a block to be produced at a specific height.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the block to wait for.
    ///
    /// # Returns
    ///
    /// A Result containing the Block or an error.
    pub async fn wait_for_block(&mut self, height: i64) -> Result<Block> {
        let mut current_block = self.current_block().await?;
        let mut current_height = current_block
            .header
            .as_ref()
            .ok_or("Header not found")?
            .height;
        while current_height < height {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            current_block = self.current_block().await?;
            current_height = current_block
                .header
                .as_ref()
                .ok_or("Header not found")?
                .height;
        }
        Ok(current_block)
    }

    /// Retrieves a transaction by its hash.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The hash of the transaction to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the Tx or an error.
    pub async fn get_tx(&mut self, tx_hash: &str) -> Result<Tx> {
        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxRequest {
            hash: tx_hash.to_owned(),
        };
        let response = self.tx_client.get_tx(request).await?.into_inner();
        let tx = response.tx.ok_or("Tx response not found")?;
        Ok(tx)
    }

    /// Retrieves the transaction respotransport::httpnse by its hash.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The hash of the transaction to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the TxResponse or an error.
    pub async fn get_tx_response(&mut self, tx_hash: &str) -> Result<TxResponse> {
        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxRequest {
            hash: tx_hash.to_owned(),
        };
        let response = self.tx_client.get_tx(request).await?.into_inner();
        let tx_response = response.tx_response.ok_or(
            "Tx r    }
        esponse not found",
        )?;
        Ok(tx_response)
    }

    /// Waits for a transaction to be included in a block.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The hash of the transaction to wait for.
    /// * `timeout` - An optional timeout duration.
    ///
    /// # Returns
    ///
    /// A Result containing the Tx or an error.
    pub async fn wait_for_tx(
        &mut self,
        tx_hash: &str,
        timeout: Option<tokio::time::Duration>,
    ) -> Result<Tx> {
        let start = std::time::Instant::now();
        loop {
            let tx = match self.get_tx(tx_hash).await {
                Ok(tx) => tx,
                Err(e) => {
                    if let Some(timeout) = timeout {
                        if start.elapsed() > timeout {
                            return Err(e);
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            };
            return Ok(tx);
        }
    }
}