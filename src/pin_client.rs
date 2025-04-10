use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::{Error, Result},
    proto::gevulot::gevulot::{
        MsgAckPin, MsgAckPinResponse, MsgCreatePin, MsgCreatePinResponse, MsgDeletePin,
        MsgDeletePinResponse,
    },
};

/// Client for managing pins in the Gevulot system.
///
/// PinClient provides a high-level interface for interacting with the data pinning
/// functionality of the Gevulot blockchain. It allows clients to create, query, 
/// and manage pinned data across the network.
///
/// # Pin Lifecycle
///
/// 1. Pins are created with specific size, time, and redundancy requirements
/// 2. The system assigns workers to store the pinned data
/// 3. Workers acknowledge successful data storage
/// 4. Data remains available for the specified time period
/// 5. Pins can be manually deleted before expiration if needed
///
/// # Examples
///
/// ## Creating a client
///
/// ```no_run
/// use std::sync::Arc;
/// use tokio::sync::RwLock;
/// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, pin_client::PinClient};
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
///     // Initialize the pin client
///     let mut pin_client = PinClient::new(base_client);
///     
///     // Now ready to use pin_client
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PinClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl PinClient {
    /// Creates a new instance of PinClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient, which handles the
    ///   underlying communication with the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// A new instance of PinClient.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, pin_client::PinClient};
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
    /// // Initialize the pin client
    /// let pin_client = PinClient::new(base_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all pins in the Gevulot network.
    ///
    /// Retrieves a complete list of all pins currently in the system,
    /// regardless of their status or ownership.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Pin>>` - A vector containing all pins in the system, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The connection to the Gevulot blockchain fails
    /// - The request times out
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, pin_client::PinClient};
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Get all pins in the system
    ///     let pins = pin_client.list().await?;
    ///     
    ///     // Display pin information
    ///     for pin in pins {
    ///         if let Some(metadata) = &pin.metadata {
    ///             println!("Pin ID: {}, Name: {}", 
    ///                 metadata.id, 
    ///                 metadata.name);
    ///         }
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&mut self) -> Result<Vec<crate::proto::gevulot::gevulot::Pin>> {
        let request = crate::proto::gevulot::gevulot::QueryAllPinRequest { pagination: None };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .pin_all(request)
            .await?;
        Ok(response.into_inner().pin)
    }

    /// Gets a pin by its CID.
    ///
    /// Retrieves detailed information about a specific pin, including its
    /// current status, storage requirements, and associated metadata.
    ///
    /// # Arguments
    ///
    /// * `cid` - The Content Identifier (CID) of the pin to retrieve
    ///
    /// # Returns
    ///
    /// * `Result<Pin>` - The requested pin details on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified pin does not exist
    /// - The connection to the Gevulot blockchain fails
    /// - The request times out
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, pin_client::PinClient};
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Retrieve a specific pin by its CID
    ///     let cid = "bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu";
    ///     match pin_client.get(cid).await {
    ///         Ok(pin) => {
    ///             println!("Pin found:");
    ///             if let Some(metadata) = &pin.metadata {
    ///                 println!("  ID: {}", metadata.id);
    ///                 println!("  Name: {}", metadata.name);
    ///                 println!("  Creator: {}", metadata.creator);
    ///             }
    ///             if let Some(spec) = &pin.spec {
    ///                 println!("  Size: {} bytes", spec.bytes);
    ///                 println!("  Redundancy: {}", spec.redundancy);
    ///             }
    ///         },
    ///         Err(e) => println!("Error retrieving pin: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&mut self, cid: &str) -> Result<crate::proto::gevulot::gevulot::Pin> {
        let request = crate::proto::gevulot::gevulot::QueryGetPinRequest {
            cid: cid.to_owned(),
        };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .pin(request)
            .await?;
        response.into_inner().pin.ok_or(Error::NotFound)
    }

    /// Creates a new pin in the Gevulot network.
    ///
    /// Submits a new data pinning request to make data available in the network.
    /// The pin specifies data size, storage duration, and redundancy requirements.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing all the pin creation parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgCreatePinResponse>` - Response containing the created pin's ID on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required fields are missing or invalid
    /// - The creator account doesn't exist or lacks permissions
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ## Creating a basic pin with existing CID
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     pin_client::PinClient,
    ///     builders::{MsgCreatePinBuilder, ByteSize, ByteUnit},
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Build a pin creation message
    ///     let msg = MsgCreatePinBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .cid(Some("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()))
    ///         .name("Dataset v1".to_string())
    ///         .bytes(ByteSize::new(1, ByteUnit::Gigabyte))
    ///         .redundancy(3)
    ///         .time(2592000) // 30 days
    ///         .description("ML training dataset".to_string())
    ///         .fallback_urls(vec![])
    ///         .tags(vec![])
    ///         .labels(vec![])
    ///         .into_message()?;
    ///     
    ///     // Submit the pin
    ///     let response = pin_client.create(msg).await?;
    ///     println!("Pin created with ID: {}", response.id);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Creating a pin with fallback URLs and metadata
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     pin_client::PinClient,
    ///     builders::{MsgCreatePinBuilder, ByteSize, ByteUnit},
    ///     proto::gevulot::gevulot::Label,
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Build a pin creation message with fallback URLs and metadata
    ///     let msg = MsgCreatePinBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .cid(None)
    ///         .name("Research Dataset 2023".to_string())
    ///         .bytes(ByteSize::new(5, ByteUnit::Gigabyte))
    ///         .redundancy(2)
    ///         .time(7776000) // 90 days
    ///         .description("Reference dataset for 2023 research".to_string())
    ///         .fallback_urls(vec![
    ///             "https://example.com/datasets/ref2023.tar.gz".to_string(),
    ///             "ipfs://QmUNLLsPACCz1vLxQVkXqqLX5R1X345qqfHbsf67hvA3Nn".to_string(),
    ///         ])
    ///         .tags(vec!["dataset".to_string(), "reference".to_string(), "2023".to_string()])
    ///         .labels(vec![
    ///             Label { key: "department".to_string(), value: "research".to_string() },
    ///             Label { key: "sensitivity".to_string(), value: "public".to_string() },
    ///         ])
    ///         .into_message()?;
    ///     
    ///     // Submit the pin
    ///     let response = pin_client.create(msg).await?;
    ///     println!("Pin created with ID: {}", response.id);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&mut self, msg: MsgCreatePin) -> Result<MsgCreatePinResponse> {
        let resp: MsgCreatePinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a pin from the Gevulot network.
    ///
    /// Removes a previously created pin from the system. Only the pin's creator
    /// can delete it. Note that this doesn't immediately remove the data from
    /// storage nodes, but signals that the data is no longer required.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the pin deletion parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgDeletePinResponse>` - Response confirming deletion on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified pin does not exist
    /// - The caller is not the pin creator
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     pin_client::PinClient,
    ///     builders::MsgDeletePinBuilder,
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Build a pin deletion message
    ///     let msg = MsgDeletePinBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///         .id("pin-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Delete the pin
    ///     match pin_client.delete(msg).await {
    ///         Ok(_) => println!("Pin successfully deleted"),
    ///         Err(e) => println!("Failed to delete pin: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&mut self, msg: MsgDeletePin) -> Result<MsgDeletePinResponse> {
        let resp: MsgDeletePinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Acknowledges a pin in the Gevulot network.
    ///
    /// Used by workers to signal that they have successfully stored (or failed to store)
    /// the pinned data. This acknowledgment is essential for the network to track
    /// data availability and redundancy.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the pin acknowledgment details
    ///
    /// # Returns
    ///
    /// * `Result<MsgAckPinResponse>` - Response confirming acknowledgment on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified pin does not exist
    /// - The worker is not assigned to the pin
    /// - The caller is not the worker's registered owner
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ## Acknowledging successful storage
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     pin_client::PinClient,
    ///     builders::MsgAckPinBuilder,
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Build a successful pin acknowledgment message
    ///     let msg = MsgAckPinBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///         .id("pin-123456".to_string())
    ///         .worker_id("worker-789012".to_string())
    ///         .success(true)
    ///         .error(None)
    ///         .into_message()?;
    ///     
    ///     // Send the acknowledgment
    ///     match pin_client.ack(msg).await {
    ///         Ok(_) => println!("Pin successfully acknowledged"),
    ///         Err(e) => println!("Failed to acknowledge pin: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Reporting a storage failure
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     pin_client::PinClient,
    ///     builders::MsgAckPinBuilder,
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
    ///     let mut pin_client = PinClient::new(base_client);
    ///     
    ///     // Build a pin failure acknowledgment message
    ///     let msg = MsgAckPinBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///         .id("pin-123456".to_string())
    ///         .worker_id("worker-789012".to_string())
    ///         .success(false)
    ///         .error(Some("Failed to retrieve data from fallback URLs".to_string()))
    ///         .into_message()?;
    ///     
    ///     // Send the failure acknowledgment
    ///     match pin_client.ack(msg).await {
    ///         Ok(_) => println!("Pin failure successfully reported"),
    ///         Err(e) => println!("Failed to report pin failure: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn ack(&mut self, msg: MsgAckPin) -> Result<MsgAckPinResponse> {
        let resp: MsgAckPinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
