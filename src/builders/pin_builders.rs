/*!
 * # Pin Builder Types
 *
 * This module provides builders for creating pin-related messages in the Gevulot network.
 * These include messages for creating, deleting, and acknowledging data pins.
 */
use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self, Label},
};

use super::common::ByteSize;

/// Builder for creating data pinning messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to pin data in the Gevulot network,
/// making it available for computational tasks. Pinning data ensures it remains
/// accessible for a specified period.
///
/// # Fields
///
/// * `creator` - Identity of the account creating the pin
/// * `cid` - Optional Content Identifier for existing data
/// * `bytes` - Size of the data being pinned
/// * `name` - Human-readable name for the pin
/// * `redundancy` - Number of redundant copies to maintain 
/// * `time` - How long to pin the data (in seconds)
/// * `description` - Optional detailed description
/// * `fallback_urls` - Alternative sources for the data
/// * `tags` - Simple string tags for categorization
/// * `labels` - Key-value pairs for metadata and filtering
///
/// # Examples
///
/// ## Creating a pin for existing data
///
/// ```
/// use gevulot_rs::builders::{MsgCreatePinBuilder, ByteSize, ByteUnit};
/// use gevulot_rs::proto::gevulot::gevulot::Label;
///
/// let msg = MsgCreatePinBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .cid(Some("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()))
///     .name("Training Dataset v1".to_string())
///     .bytes(ByteSize::new(20, ByteUnit::Gigabyte))
///     .redundancy(3)
///     .time(2592000) // 30 days
///     .description("Machine learning training dataset for image classification".to_string())
///     .fallback_urls(vec![])
///     .tags(vec![])
///     .labels(vec![])
///     .build()
///     .unwrap();
/// ```
///
/// ## Creating a pin with fallback URLs and metadata
///
/// ```
/// use gevulot_rs::builders::{MsgCreatePinBuilder, ByteSize, ByteUnit};
/// use gevulot_rs::proto::gevulot::gevulot::Label;
///
/// let msg = MsgCreatePinBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .cid(Some("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()))
///     .name("Reference Dataset 2023".to_string())
///     .bytes(ByteSize::new(5, ByteUnit::Gigabyte))
///     .redundancy(2)
///     .time(7776000) // 90 days
///     .description("Reference dataset for 2023 research".to_string())
///     .fallback_urls(vec![
///         "https://example.com/datasets/ref2023.tar.gz".to_string(),
///         "ipfs://QmUNLLsPACCz1vLxQVkXqqLX5R1X345qqfHbsf67hvA3Nn".to_string(),
///     ])
///     .tags(vec!["dataset".to_string(), "reference".to_string(), "2023".to_string()])
///     .labels(vec![
///         Label { key: "department".to_string(), value: "research".to_string() },
///         Label { key: "sensitivity".to_string(), value: "public".to_string() },
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgCreatePin {
    /// Identity of the account creating the pin
    /// This must be a valid Gevulot account address
    pub creator: String,
    
    /// Content Identifier (CID) of the data to pin
    /// If the data is being uploaded, this can be None
    pub cid: Option<String>,
    
    /// Size of the data being pinned
    /// This is used for resource allocation and billing
    pub bytes: ByteSize,
    
    /// Human-readable name for the pin
    /// Used for display and searching purposes
    pub name: String,
    
    /// Number of redundant copies to maintain in the network
    /// Higher values increase availability at the cost of resources
    pub redundancy: u64,
    
    /// How long to pin the data, in seconds
    /// After this period, data may be garbage collected
    pub time: u64,
    
    /// Optional detailed description of the pinned data
    /// Provides context about the data's purpose and contents
    pub description: String,
    
    /// Alternative sources where the data can be fetched
    /// These URLs are used as fallbacks if the data isn't in the network
    pub fallback_urls: Vec<String>,
    
    /// Simple string tags for categorization and filtering
    /// These provide a basic way to group related pins
    pub tags: Vec<String>,
    
    /// Key-value pairs for detailed metadata and advanced filtering
    /// These provide more structured metadata than tags
    pub labels: Vec<Label>,
}

impl MsgCreatePinBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgCreatePin>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{MsgCreatePinBuilder, ByteSize, ByteUnit};
    ///
    /// let proto_msg = MsgCreatePinBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .cid(Some("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()))
    ///     .name("Dataset XYZ".to_string())
    ///     .bytes(ByteSize::new(1, ByteUnit::Gigabyte))
    ///     .redundancy(2)
    ///     .time(604800) // 1 week
    ///     .description("Example dataset".to_string())
    ///     .fallback_urls(vec![])
    ///     .tags(vec![])
    ///     .labels(vec![])
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgCreatePin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgCreatePin {
            creator: msg.creator,
            cid: msg.cid.unwrap_or_default(),
            bytes: msg.bytes.to_bytes(),
            name: msg.name,
            redundancy: msg.redundancy,
            time: msg.time,
            description: msg.description,
            fallback_urls: msg.fallback_urls,
            tags: msg.tags,
            labels: msg.labels,
        })
    }
}

/// Builder for creating pin deletion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to request deletion of a previously
/// created pin in the Gevulot network. Only the original creator or an admin can
/// delete a pin before its expiration time.
///
/// # Fields
///
/// * `creator` - Identity of the account requesting deletion
/// * `cid` - Content Identifier of the pinned data
/// * `id` - Unique identifier of the pin to delete
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgDeletePinBuilder;
///
/// let proto_msg = MsgDeletePinBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
///     .id("pin-123456".to_string())
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
#[derive(Builder)]
pub struct MsgDeletePin {
    /// Identity of the account requesting pin deletion
    /// This must match the original creator or be an admin account
    pub creator: String,
    
    /// Content Identifier (CID) of the pinned data
    /// This is the unique hash identifying the data
    pub cid: String,
    
    /// Unique identifier of the pin to delete
    /// This is the blockchain-assigned ID for the pin
    pub id: String,
}

impl MsgDeletePinBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgDeletePin>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgDeletePinBuilder;
    ///
    /// let proto_msg = MsgDeletePinBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///     .id("pin-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgDeletePin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeletePin {
            creator: msg.creator,
            cid: msg.cid,
            id: msg.id,
        })
    }
}

/// Builder for creating pin acknowledgment messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to acknowledge that a worker
/// has successfully (or unsuccessfully) pinned requested data. This is an important
/// part of the data availability protocol in Gevulot.
///
/// # Fields
///
/// * `creator` - Identity of the account sending the acknowledgment
/// * `cid` - Content Identifier of the pinned data
/// * `id` - Unique identifier of the pin
/// * `worker_id` - Identifier of the worker that attempted pinning
/// * `success` - Whether the pinning operation succeeded
/// * `error` - Optional error message in case of failure
///
/// # Examples
///
/// ## Acknowledging successful pinning
///
/// ```
/// use gevulot_rs::builders::MsgAckPinBuilder;
///
/// let proto_msg = MsgAckPinBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
///     .id("pin-123456".to_string())
///     .worker_id("worker-789012".to_string())
///     .success(true)
///     .error(None)
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
///
/// ## Reporting a pinning failure
///
/// ```
/// use gevulot_rs::builders::MsgAckPinBuilder;
///
/// let proto_msg = MsgAckPinBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
///     .id("pin-123456".to_string())
///     .worker_id("worker-789012".to_string())
///     .success(false)
///     .error(Some("Failed to retrieve data from fallback URLs".to_string()))
///     .into_message()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgAckPin {
    /// Identity of the account sending the acknowledgment
    /// This should match the worker's registered owner
    pub creator: String,
    
    /// Content Identifier (CID) of the pinned data
    /// This is the unique hash identifying the data
    pub cid: String,
    
    /// Unique identifier of the pin
    /// This is the blockchain-assigned ID for the pin
    pub id: String,
    
    /// Identifier of the worker that attempted pinning
    /// This is the blockchain-assigned ID for the worker
    pub worker_id: String,
    
    /// Whether the pinning operation succeeded
    /// True indicates successful data storage, false indicates failure
    pub success: bool,
    
    /// Optional error message in case of failure
    /// Provides details about why pinning failed, if applicable
    pub error: Option<String>,
}

impl MsgAckPinBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgAckPin>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgAckPinBuilder;
    ///
    /// let proto_msg = MsgAckPinBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///     .id("pin-123456".to_string())
    ///     .worker_id("worker-789012".to_string())
    ///     .success(true)
    ///     .error(None)
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgAckPin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgAckPin {
            creator: msg.creator,
            cid: msg.cid,
            id: msg.id,
            worker_id: msg.worker_id,
            success: msg.success,
            error: msg.error.unwrap_or_default(),
        })
    }
} 