use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{SimulateResponse, Tx};
use cosmos_sdk_proto::prost::{Message, Name};
use cosmos_sdk_proto::tendermint::types::Block;
use cosmrs::{auth::BaseAccount, Coin};
use tonic::transport::{Channel, ClientTlsConfig};

use crate::error::{Error, Result};
use crate::signer::GevulotSigner;

// Type aliases for various clients used in the BaseClient
type AuthQueryClient<T> = cosmrs::proto::cosmos::auth::v1beta1::query_client::QueryClient<T>;
type BankQueryClient<T> = cosmrs::proto::cosmos::bank::v1beta1::query_client::QueryClient<T>;
type GovQueryClient<T> = cosmrs::proto::cosmos::gov::v1beta1::query_client::QueryClient<T>;
type GevulotQueryClient<T> = crate::proto::gevulot::gevulot::query_client::QueryClient<T>;
type TxServiceClient<T> = cosmrs::proto::cosmos::tx::v1beta1::service_client::ServiceClient<T>;
type TendermintClient<T> =
    cosmrs::proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient<T>;

/// BaseClient is a struct that provides various functionalities to interact with the blockchain.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct BaseClient {
    // Query clients
    pub auth_client: AuthQueryClient<Channel>,
    pub bank_client: BankQueryClient<Channel>,
    pub gevulot_client: GevulotQueryClient<Channel>,
    pub gov_client: GovQueryClient<Channel>,
    pub tendermint_client: TendermintClient<Channel>,
    // Message client
    pub tx_client: TxServiceClient<Channel>,

    gas_price: f64,
    denom: String,
    gas_multiplier: f64,

    // Data from signer
    pub address: Option<String>,
    pub pub_key: Option<cosmrs::crypto::PublicKey>,
    #[derivative(Debug = "ignore")]
    priv_key: Option<cosmrs::crypto::secp256k1::SigningKey>,

    // Latest account sequence
    pub account_sequence: Option<u64>,
}

impl BaseClient {
    /// Creates a new instance of BaseClient.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint URL to connect to.
    /// * `gas_price` - The gas price to be used.
    /// * `gas_multiplier` - The gas multiplier to be used.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of BaseClient or an error.
    pub async fn new(endpoint: &str, gas_price: f64, gas_multiplier: f64) -> Result<Self> {
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
            denom: "ucredit".to_owned(),
            gas_price,
            gas_multiplier,
            address: None,
            pub_key: None,
            priv_key: None,
            account_sequence: None,
        })
    }

    /// Sets the signer for the client.
    ///
    /// # Arguments
    ///
    /// * `signer` - The GevulotSigner to be set.
    pub fn set_signer(&mut self, signer: GevulotSigner) {
        self.address = Some(signer.0.public_address.to_string());
        self.pub_key = Some(signer.0.public_key);
        self.priv_key = Some(signer.0.private_key);
    }

    /// Sets the mnemonic for the client and initializes the signer.
    ///
    /// # Arguments
    ///
    /// * `mnemonic` - The mnemonic string to be used.
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure.
    pub fn set_mnemonic(&mut self, mnemonic: &str, password: Option<&str>) -> Result<()> {
        let signer = GevulotSigner::from_mnemonic(mnemonic, password)?;
        self.set_signer(signer);
        Ok(())
    }

    /// Retrieves the account information for a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing the BaseAccount or an error.
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

    /// Retrieves the account balance for a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account, which balance to get.
    ///
    /// # Returns
    ///
    /// A Result containing the balance or an error.
    pub async fn get_account_balance(&mut self, address: &str) -> Result<Coin> {
        let request = cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest {
            address: address.to_string(),
            denom: String::from("ucredit"),
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
    ) -> Result<SimulateResponse> {
        let msg = cosmrs::Any::from_msg(&msg)?;
        let gas = 100_000u64;
        let chain_id: cosmrs::tendermint::chain::Id = "gevulot"
            .parse()
            .map_err(|_| Error::Parse("fail".to_string()))?;
        let tx_body = cosmrs::tx::BodyBuilder::new().msg(msg).memo(memo).finish();
        let signer_info = cosmrs::tx::SignerInfo::single_direct(self.pub_key, sequence);
        let fee = cosmrs::tx::Fee::from_amount_and_gas(
            Coin {
                denom: self.denom.parse()?,
                amount: (self.gas_price * gas as f64) as u128,
            },
            gas,
        );
        let auth_info = signer_info.auth_info(fee);
        let sign_doc = cosmrs::tx::SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;
        let tx_raw = sign_doc.sign(self.priv_key.as_ref().ok_or("Private key not set")?)?;
        let tx_bytes = tx_raw.to_bytes()?;
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
        // Use simulate_msg to estimate gas
        let (account_number, sequence) = self.get_account_details().await?;
        let simulate_response = self
            .simulate_msg(msg.clone(), memo, account_number, sequence)
            .await?;
        log::debug!("simulate_response: {:#?}", simulate_response);
        let gas_info = simulate_response.gas_info.ok_or("Failed to get gas info")?;
        let gas_limit = ((gas_info.gas_used * (100. * self.gas_multiplier) as u64) / 100) + 1; // Adjust gas limit based on simulation
        let fee = cosmrs::tx::Fee::from_amount_and_gas(
            Coin {
                denom: self.denom.parse()?,
                amount: (self.gas_price * gas_limit as f64) as u128,
            },
            gas_limit,
        );

        log::debug!("fee: {:?}", fee);

        let msg = cosmrs::Any::from_msg(&msg)?;
        let chain_id: cosmrs::tendermint::chain::Id = "gevulot"
            .parse()
            .map_err(|_| Error::Parse("fail".to_string()))?;
        let tx_body = cosmrs::tx::BodyBuilder::new().msg(msg).memo(memo).finish();
        let signer_info = cosmrs::tx::SignerInfo::single_direct(self.pub_key, sequence);
        let auth_info = signer_info.auth_info(fee);
        let sign_doc = cosmrs::tx::SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;
        let tx_raw = sign_doc.sign(self.priv_key.as_ref().ok_or("Private key not set")?)?;
        let tx_bytes = tx_raw.to_bytes()?;

        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest {
            tx_bytes,
            mode: 2, // BROADCAST_MODE_SYNC -> Wait for the tx to be processed, but not in-block
        };
        let resp = self.tx_client.broadcast_tx(request).await?;
        let resp = resp.into_inner();
        log::debug!("broadcast_tx response: {:#?}", resp);
        let tx_response = resp.tx_response.ok_or("Tx response not found")?;
        if tx_response.code != 0 {
            return Err(Error::Unknown(format!(
                "Transaction failed with code: {} ({})",
                tx_response.code, tx_response.raw_log
            )));
        }

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
