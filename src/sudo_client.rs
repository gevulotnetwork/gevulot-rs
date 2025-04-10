use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::Result,
    proto::gevulot::gevulot::{
        MsgSudoDeletePin, MsgSudoDeletePinResponse, MsgSudoDeleteTask, MsgSudoDeleteTaskResponse,
        MsgSudoDeleteWorker, MsgSudoDeleteWorkerResponse, MsgSudoFreezeAccount,
        MsgSudoFreezeAccountResponse,
    },
};

/// Client for managing sudo operations in the Gevulot system.
///
/// SudoClient provides a high-level interface for performing administrative
/// operations on the Gevulot blockchain. These operations require special
/// administrative privileges and can only be executed by accounts with sudo
/// permissions.
///
/// # Administrative Capabilities
///
/// The SudoClient can perform various privileged operations:
/// 1. Delete pins to forcibly remove data that's being retained
/// 2. Delete workers to remove non-responsive or malicious worker nodes
/// 3. Delete tasks to terminate problematic computations
/// 4. Freeze accounts to prevent malicious users from performing actions
///
/// # Examples
///
/// ## Creating a client
///
/// ```no_run
/// use std::sync::Arc;
/// use tokio::sync::RwLock;
/// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, sudo_client::SudoClient};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a connection to the Gevulot node
///     let base_client = Arc::new(RwLock::new(
///         BaseClient::new(
///             "http://localhost:9090",
///             FuelPolicy::Dynamic { 
///                 gas_price: 0.025, 
///                 gas_multiplier: 1.2 
///             }
///         ).await?
///     ));
///     
///     // Initialize the sudo client
///     let mut sudo_client = SudoClient::new(base_client);
///     
///     // Now ready to use sudo_client for administrative operations
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SudoClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl SudoClient {
    /// Creates a new instance of SudoClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient, which handles the
    ///   underlying communication with the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// A new instance of SudoClient.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, sudo_client::SudoClient};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Create a connection to the Gevulot node
    /// let base_client = Arc::new(RwLock::new(
    ///     BaseClient::new(
    ///         "http://localhost:9090",
    ///         FuelPolicy::Dynamic { 
    ///             gas_price: 0.025, 
    ///             gas_multiplier: 1.2 
    ///         }
    ///     ).await?
    /// ));
    /// 
    /// // Initialize the sudo client
    /// let sudo_client = SudoClient::new(base_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Deletes a pin from the Gevulot network.
    ///
    /// Forcibly removes a pin from the network, allowing the associated data to be
    /// garbage collected. This operation requires administrative privileges and
    /// should be used with caution, as it may lead to data loss if the content is
    /// not replicated elsewhere.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the details of the pin to delete.
    ///
    /// # Returns
    ///
    /// * `Result<MsgSudoDeletePinResponse>` - Response confirming deletion on success, or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The caller lacks administrative privileges
    /// - The specified CID does not exist or is not pinned
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     sudo_client::SudoClient,
    ///     builders::MsgSudoDeletePinBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut sudo_client = SudoClient::new(base_client);
    ///     
    ///     // Build a pin deletion message
    ///     let msg = MsgSudoDeletePinBuilder::default()
    ///         .authority("gevulot1admin".to_string())
    ///         .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///         .into_message()?;
    ///     
    ///     // Delete the pin
    ///     match sudo_client.delete_pin(msg).await {
    ///         Ok(_) => println!("Pin successfully deleted"),
    ///         Err(e) => println!("Failed to delete pin: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_pin(&mut self, msg: MsgSudoDeletePin) -> Result<MsgSudoDeletePinResponse> {
        let resp: MsgSudoDeletePinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a worker from the Gevulot network.
    ///
    /// Forcibly removes a worker node from the network. This operation requires
    /// administrative privileges and should be used for managing problematic
    /// or non-responsive worker nodes.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the details of the worker to delete.
    ///
    /// # Returns
    ///
    /// * `Result<MsgSudoDeleteWorkerResponse>` - Response confirming deletion on success, or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The caller lacks administrative privileges
    /// - The specified worker does not exist
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     sudo_client::SudoClient,
    ///     builders::MsgSudoDeleteWorkerBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut sudo_client = SudoClient::new(base_client);
    ///     
    ///     // Build a worker deletion message
    ///     let msg = MsgSudoDeleteWorkerBuilder::default()
    ///         .authority("gevulot1admin".to_string())
    ///         .id("worker-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Delete the worker
    ///     match sudo_client.delete_worker(msg).await {
    ///         Ok(_) => println!("Worker successfully deleted"),
    ///         Err(e) => println!("Failed to delete worker: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_worker(
        &mut self,
        msg: MsgSudoDeleteWorker,
    ) -> Result<MsgSudoDeleteWorkerResponse> {
        let resp: MsgSudoDeleteWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a task from the Gevulot network.
    ///
    /// Forcibly removes a task from the network, regardless of its state or ownership.
    /// This operation requires administrative privileges and should be used for
    /// managing problematic or resource-consuming tasks that cannot be handled through
    /// standard channels.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the details of the task to delete.
    ///
    /// # Returns
    ///
    /// * `Result<MsgSudoDeleteTaskResponse>` - Response confirming deletion on success, or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The caller lacks administrative privileges
    /// - The specified task does not exist
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     sudo_client::SudoClient,
    ///     builders::MsgSudoDeleteTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut sudo_client = SudoClient::new(base_client);
    ///     
    ///     // Build a task deletion message
    ///     let msg = MsgSudoDeleteTaskBuilder::default()
    ///         .authority("gevulot1admin".to_string())
    ///         .id("task-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Delete the task
    ///     match sudo_client.delete_task(msg).await {
    ///         Ok(_) => println!("Task successfully deleted"),
    ///         Err(e) => println!("Failed to delete task: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_task(
        &mut self,
        msg: MsgSudoDeleteTask,
    ) -> Result<MsgSudoDeleteTaskResponse> {
        let resp: MsgSudoDeleteTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Freezes an account in the Gevulot network.
    ///
    /// Restricts a user account from performing operations on the network. This
    /// operation requires administrative privileges and should be used for dealing
    /// with malicious or compromised accounts.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the details of the account to freeze.
    ///
    /// # Returns
    ///
    /// * `Result<MsgSudoFreezeAccountResponse>` - Response confirming the account freeze on success, or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The caller lacks administrative privileges
    /// - The specified account does not exist
    /// - The account is already frozen
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     sudo_client::SudoClient,
    ///     builders::MsgSudoFreezeAccountBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut sudo_client = SudoClient::new(base_client);
    ///     
    ///     // Build an account freeze message
    ///     let msg = MsgSudoFreezeAccountBuilder::default()
    ///         .authority("gevulot1admin".to_string())
    ///         .account("gevulot1malicious".to_string())
    ///         .into_message()?;
    ///     
    ///     // Freeze the account
    ///     match sudo_client.freeze_account(msg).await {
    ///         Ok(_) => println!("Account successfully frozen"),
    ///         Err(e) => println!("Failed to freeze account: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn freeze_account(
        &mut self,
        msg: MsgSudoFreezeAccount,
    ) -> Result<MsgSudoFreezeAccountResponse> {
        let resp: MsgSudoFreezeAccountResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
