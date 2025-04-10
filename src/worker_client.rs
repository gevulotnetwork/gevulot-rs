use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::{Error, Result},
    proto::{
        cosmos::base::query::v1beta1::PageRequest,
        gevulot::gevulot::{
            MsgAnnounceWorkerExit, MsgAnnounceWorkerExitResponse, MsgCreateWorker,
            MsgCreateWorkerResponse, MsgDeleteWorker, MsgDeleteWorkerResponse, MsgUpdateWorker,
            MsgUpdateWorkerResponse, QueryAllWorkerRequest,
        },
    },
    models::Worker,
};

/// Default page size for pagination.
const PAGE_SIZE: u64 = 100;

/// Client for managing worker nodes in the Gevulot network.
///
/// The `WorkerClient` provides a high-level interface for interacting with worker nodes
/// in the Gevulot blockchain. It enables registration, querying, updating, and deactivation
/// of compute workers that provide resources to the network.
///
/// Workers are essential components of the Gevulot network, providing CPU, GPU, memory,
/// and storage resources for executing user tasks and computations.
///
/// # Examples
///
/// ## Creating a client and listing workers
///
/// ```no_run
/// use std::sync::Arc;
/// use tokio::sync::RwLock;
/// use gevulot_rs::{
///     base_client::{BaseClient, FuelPolicy}, 
///     worker_client::WorkerClient,
///     models::Worker,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Initialize the base client
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
///     // Create the worker client
///     let mut worker_client = WorkerClient::new(base_client);
///     
///     // List all workers
///     let workers = worker_client.list().await?;
///     println!("Found {} workers", workers.len());
///     
///     // Display information about each worker
///     for worker in workers {
///         println!("Worker ID: {}", worker.metadata.id.unwrap_or_default());
///         println!("Name: {}", worker.metadata.name);
///         println!("Description: {}", worker.metadata.description);
///         println!("---");
///     }
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WorkerClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl WorkerClient {
    /// Creates a new instance of WorkerClient.
    ///
    /// This constructor initializes a new `WorkerClient` with the provided base client.
    /// The base client handles the low-level communication with the Gevulot blockchain.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient that manages 
    ///                   communication with the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// A new instance of WorkerClient.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient
    /// };
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
    /// // Initialize the worker client
    /// let worker_client = WorkerClient::new(base_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all registered worker nodes in the Gevulot network.
    ///
    /// This method retrieves all worker nodes registered on the blockchain, handling
    /// pagination automatically to ensure all workers are returned. The results include
    /// detailed information about each worker's capabilities, resources, and metadata.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of Worker models or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The network connection fails
    /// - The Gevulot client returns an error response
    /// - The response cannot be properly decoded
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient,
    ///     models::Worker,
    /// };
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
    ///     let mut worker_client = WorkerClient::new(base_client);
    ///     
    ///     // List all workers
    ///     let workers = worker_client.list().await?;
    ///     
    ///     // Display information about each worker
    ///     for worker in workers {
    ///         println!("Worker ID: {}", worker.metadata.id.unwrap_or_default());
    ///         println!("Name: {}", worker.metadata.name);
    ///         println!("Description: {}", worker.metadata.description);
    ///         println!("CPUs: {}", worker.spec.cpus);
    ///         println!("GPUs: {}", worker.spec.gpus);
    ///         println!("Memory: {}", worker.spec.memory);
    ///         println!("Disk: {}", worker.spec.disk);
    ///         println!("---");
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&mut self) -> Result<Vec<Worker>> {
        let mut all_workers = Vec::new();
        let mut next_key: Option<Vec<u8>> = None;

        loop {
            // Construct request with pagination for the current page.
            let pagination = Some(PageRequest {
                key: next_key.unwrap_or_default(),
                limit: PAGE_SIZE,
                ..Default::default()
            });
            let request = QueryAllWorkerRequest { pagination };

            let response = self
                .base_client
                .write()
                .await
                .gevulot_client
                .worker_all(request)
                .await?;

            let inner = response.into_inner();
            all_workers.extend(inner.worker.into_iter().map(Worker::from));

            // Handle next page.
            next_key = inner.pagination.and_then(|p| {
                if p.next_key.is_empty() {
                    None
                } else {
                    Some(p.next_key)
                }
            });
            if next_key.is_none() {
                break;
            }
        }

        Ok(all_workers)
    }

    /// Retrieves a specific worker node by its ID.
    ///
    /// This method fetches detailed information about a single worker node
    /// identified by its unique ID. The information includes the worker's
    /// computational resources, metadata, and current status.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the worker to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the Worker model or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The specified worker ID does not exist
    /// - The network connection fails
    /// - The Gevulot client returns an error response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient,
    ///     models::Worker,
    /// };
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
    ///     let mut worker_client = WorkerClient::new(base_client);
    ///     
    ///     // Retrieve a specific worker by ID
    ///     let worker_id = "worker-123456";
    ///     match worker_client.get(worker_id).await {
    ///         Ok(worker) => {
    ///             println!("Worker found:");
    ///             println!("Name: {}", worker.metadata.name);
    ///             println!("Description: {}", worker.metadata.description);
    ///             println!("CPUs: {}", worker.spec.cpus);
    ///             
    ///             // Print labels if any
    ///             if !worker.metadata.labels.is_empty() {
    ///                 println!("Labels:");
    ///                 for label in &worker.metadata.labels {
    ///                     println!("  {}: {}", label.key, label.value);
    ///                 }
    ///             }
    ///         },
    ///         Err(e) => println!("Error retrieving worker: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&mut self, id: &str) -> Result<Worker> {
        let request = crate::proto::gevulot::gevulot::QueryGetWorkerRequest { id: id.to_owned() };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .worker(request)
            .await?;
        response
            .into_inner()
            .worker
            .map(Worker::from)
            .ok_or(Error::NotFound)
    }

    /// Registers a new worker node with the Gevulot blockchain.
    ///
    /// This method submits a transaction to create a new worker record on the blockchain,
    /// making the worker's computational resources available for tasks in the network.
    /// The worker details include its hardware resources, capabilities, and metadata.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker creation details.
    ///
    /// # Returns
    ///
    /// A Result containing the response with the newly created worker's ID or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The transaction fails to be included in a block
    /// - The provided worker details are invalid
    /// - The sender lacks permission to create a worker
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient,
    ///     builders::{MsgCreateWorkerBuilder, ByteSize, ByteUnit},
    ///     proto::gevulot::gevulot::Label,
    /// };
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
    ///     let mut worker_client = WorkerClient::new(base_client);
    ///     
    ///     // Create a worker registration message
    ///     let worker_msg = MsgCreateWorkerBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .name("GPU Compute Node 1".to_string())
    ///         .description("High-performance GPU worker for ML workloads".to_string())
    ///         .cpus(16_000) // 16 cores (in millicores)
    ///         .gpus(2_000)  // 2 GPUs (in milli-GPUs)
    ///         .memory(ByteSize::new(64, ByteUnit::Gigabyte))
    ///         .disk(ByteSize::new(2048, ByteUnit::Gigabyte))
    ///         .labels(vec![
    ///             Label { key: "gpu_type".to_string(), value: "nvidia_a10".to_string() },
    ///             Label { key: "region".to_string(), value: "us-east".to_string() },
    ///         ])
    ///         .tags(vec!["gpu".to_string(), "ml".to_string()])
    ///         .into_message()?;
    ///     
    ///     // Register the worker
    ///     let response = worker_client.create(worker_msg).await?;
    ///     println!("Worker registered with ID: {}", response.id);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&mut self, msg: MsgCreateWorker) -> Result<MsgCreateWorkerResponse> {
        let resp: MsgCreateWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Updates an existing worker node on the Gevulot blockchain.
    ///
    /// This method allows modifying a previously registered worker's details,
    /// such as its resources, capabilities, or metadata. Only the original creator
    /// or an authorized account can update a worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the updated worker details.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The worker ID does not exist
    /// - The sender is not authorized to update the worker
    /// - The transaction fails to be included in a block
    /// - The provided worker details are invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient,
    ///     builders::{MsgUpdateWorkerBuilder, ByteSize, ByteUnit},
    ///     proto::gevulot::gevulot::Label,
    /// };
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
    ///     let mut worker_client = WorkerClient::new(base_client);
    ///     
    ///     // Create a worker update message
    ///     let update_msg = MsgUpdateWorkerBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .id("worker-123456".to_string())
    ///         .name("GPU Compute Node 1 - Upgraded".to_string())
    ///         .description("Upgraded high-performance GPU worker for ML workloads".to_string())
    ///         .cpus(32_000) // Upgraded to 32 cores (in millicores)
    ///         .gpus(4_000)  // Upgraded to 4 GPUs (in milli-GPUs)
    ///         .memory(ByteSize::new(128, ByteUnit::Gigabyte))
    ///         .disk(ByteSize::new(4096, ByteUnit::Gigabyte))
    ///         .labels(vec![
    ///             Label { key: "gpu_type".to_string(), value: "nvidia_a100".to_string() },
    ///             Label { key: "region".to_string(), value: "us-east".to_string() },
    ///             Label { key: "cuda_version".to_string(), value: "12.0".to_string() },
    ///         ])
    ///         .tags(vec!["gpu".to_string(), "ml".to_string(), "high-memory".to_string()])
    ///         .into_message()?;
    ///     
    ///     // Update the worker
    ///     match worker_client.update(update_msg).await {
    ///         Ok(_) => println!("Worker updated successfully"),
    ///         Err(e) => println!("Failed to update worker: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(&mut self, msg: MsgUpdateWorker) -> Result<MsgUpdateWorkerResponse> {
        let resp: MsgUpdateWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a worker node from the Gevulot blockchain.
    ///
    /// This method removes a previously registered worker from the network,
    /// making its resources unavailable for new tasks. Only the original creator
    /// or an authorized account can delete a worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker ID to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The worker ID does not exist
    /// - The sender is not authorized to delete the worker
    /// - The transaction fails to be included in a block
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient,
    ///     builders::MsgDeleteWorkerBuilder,
    /// };
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
    ///     let mut worker_client = WorkerClient::new(base_client);
    ///     
    ///     // Create a worker deletion message
    ///     let delete_msg = MsgDeleteWorkerBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .id("worker-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Delete the worker
    ///     match worker_client.delete(delete_msg).await {
    ///         Ok(_) => println!("Worker deleted successfully"),
    ///         Err(e) => println!("Failed to delete worker: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&mut self, msg: MsgDeleteWorker) -> Result<MsgDeleteWorkerResponse> {
        let resp: MsgDeleteWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Announces a worker node's planned exit from the Gevulot network.
    ///
    /// This method signals to the blockchain that a worker is gracefully
    /// shutting down. This allows the network to properly handle any in-progress
    /// tasks and update the worker's status, preventing new tasks from being
    /// assigned to it.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker exit announcement details.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The worker ID does not exist
    /// - The sender is not authorized to announce the worker's exit
    /// - The transaction fails to be included in a block
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy}, 
    ///     worker_client::WorkerClient,
    ///     builders::MsgAnnounceWorkerExitBuilder,
    /// };
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
    ///     let mut worker_client = WorkerClient::new(base_client);
    ///     
    ///     // Create a worker exit announcement message
    ///     let exit_msg = MsgAnnounceWorkerExitBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .worker_id("worker-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Announce the worker's exit
    ///     match worker_client.announce_exit(exit_msg).await {
    ///         Ok(_) => println!("Worker exit announced successfully"),
    ///         Err(e) => println!("Failed to announce worker exit: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn announce_exit(
        &mut self,
        msg: MsgAnnounceWorkerExit,
    ) -> Result<MsgAnnounceWorkerExitResponse> {
        let resp: MsgAnnounceWorkerExitResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
