/*!
 * # Worker Builder Types
 *
 * This module provides builders for creating worker-related messages in the Gevulot network.
 * These include messages for registering, updating, deleting, and managing worker nodes.
 */
use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self, Label},
};

use super::common::ByteSize;

/// Builder for registering a worker node with the Gevulot blockchain.
///
/// This struct represents the parameters needed to register a new compute worker
/// in the Gevulot network. Workers provide computational resources for executing tasks.
///
/// # Fields
///
/// * `creator` - Identity of the account registering the worker
/// * `name` - Human-readable name for the worker
/// * `description` - Detailed description of the worker
/// * `cpus` - Number of CPU cores available
/// * `gpus` - Number of GPU units available
/// * `memory` - Amount of memory available
/// * `disk` - Amount of disk space available
/// * `labels` - Key-value pairs for worker capabilities and metadata
/// * `tags` - Simple string tags for categorization
///
/// # Examples
///
/// ## Registering a CPU worker
///
/// ```
/// use gevulot_rs::builders::{MsgCreateWorkerBuilder, ByteSize, ByteUnit};
/// use gevulot_rs::proto::gevulot::gevulot::Label;
///
/// let msg = MsgCreateWorkerBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .name("CPU Worker Node 1".to_string())
///     .description("General purpose compute node with 16 cores".to_string())
///     .cpus(16_000) // 16 cores
///     .gpus(0) // No GPUs
///     .memory(ByteSize::new(32, ByteUnit::Gigabyte))
///     .disk(ByteSize::new(1024, ByteUnit::Gigabyte))
///     .labels(vec![])
///     .tags(vec![])
///     .build()
///     .unwrap();
/// ```
///
/// ## Registering a GPU worker with labels
///
/// ```
/// use gevulot_rs::builders::{MsgCreateWorkerBuilder, ByteSize, ByteUnit};
/// use gevulot_rs::proto::gevulot::gevulot::Label;
///
/// let msg = MsgCreateWorkerBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .name("ML Inference Node 3".to_string())
///     .description("High performance GPU node for ML workloads".to_string())
///     .cpus(32_000) // 32 cores
///     .gpus(4_000) // 4 GPUs
///     .memory(ByteSize::new(128, ByteUnit::Gigabyte))
///     .disk(ByteSize::new(2048, ByteUnit::Gigabyte))
///     .labels(vec![
///         Label { key: "gpu_type".to_string(), value: "nvidia_a100".to_string() },
///         Label { key: "location".to_string(), value: "us-west".to_string() },
///     ])
///     .tags(vec!["gpu".to_string(), "ml".to_string(), "inference".to_string()])
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgCreateWorker {
    /// Identity of the account registering the worker
    /// This must be a valid Gevulot account address
    pub creator: String,
    
    /// Human-readable name for the worker
    /// Used for display and searching purposes
    pub name: String,
    
    /// Detailed description of the worker's capabilities
    /// Provides context about the worker's hardware and purpose
    pub description: String,
    
    /// Number of CPU cores available in millicores (1000 = 1 CPU)
    /// Used for task scheduling and matching
    pub cpus: u64,
    
    /// Number of GPU units available in milli-GPU units (1000 = 1 GPU)
    /// Used for task scheduling and matching
    pub gpus: u64,
    
    /// Amount of memory available for tasks
    /// Used for task scheduling and matching
    pub memory: ByteSize,
    
    /// Amount of disk space available for tasks
    /// Used for task scheduling and matching
    pub disk: ByteSize,
    
    /// Key-value pairs for worker capabilities and metadata
    /// These provide structured information about the worker
    pub labels: Vec<Label>,
    
    /// Simple string tags for categorization
    /// These provide a basic way to group related workers
    pub tags: Vec<String>,
}

impl MsgCreateWorkerBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgCreateWorker>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{MsgCreateWorkerBuilder, ByteSize, ByteUnit};
    ///
    /// let proto_msg = MsgCreateWorkerBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .name("Worker Node 1".to_string())
    ///     .description("General purpose compute node".to_string())
    ///     .cpus(8_000) // 8 cores
    ///     .gpus(0)
    ///     .memory(ByteSize::new(16, ByteUnit::Gigabyte))
    ///     .disk(ByteSize::new(500, ByteUnit::Gigabyte))
    ///     .labels(vec![])
    ///     .tags(vec![])
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgCreateWorker> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgCreateWorker {
            creator: msg.creator,
            name: msg.name,
            description: msg.description,
            cpus: msg.cpus,
            gpus: msg.gpus,
            memory: msg.memory.to_bytes(),
            disk: msg.disk.to_bytes(),
            labels: msg.labels,
            tags: msg.tags,
        })
    }
}

/// Builder for updating an existing worker node in the Gevulot blockchain.
///
/// This struct represents the parameters needed to update a previously registered
/// worker in the Gevulot network. It allows for changing the worker's specifications,
/// capabilities, and metadata.
///
/// # Fields
///
/// * `creator` - Identity of the account updating the worker
/// * `id` - Unique identifier of the worker to update
/// * `name` - Updated human-readable name
/// * `description` - Updated detailed description
/// * `cpus` - Updated number of CPU cores available
/// * `gpus` - Updated number of GPU units available
/// * `memory` - Updated amount of memory available
/// * `disk` - Updated amount of disk space available
/// * `labels` - Updated key-value pairs for capabilities and metadata
/// * `tags` - Updated simple string tags for categorization
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::{MsgUpdateWorkerBuilder, ByteSize, ByteUnit};
/// use gevulot_rs::proto::gevulot::gevulot::Label;
///
/// let msg = MsgUpdateWorkerBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .id("worker-123456".to_string())
///     .name("Updated GPU Worker".to_string())
///     .description("Updated high performance GPU node".to_string())
///     .cpus(32_000) // 32 cores
///     .gpus(4_000) // 4 GPUs
///     .memory(ByteSize::new(128, ByteUnit::Gigabyte))
///     .disk(ByteSize::new(2048, ByteUnit::Gigabyte))
///     .labels(vec![
///         Label { key: "gpu_type".to_string(), value: "nvidia_a100".to_string() },
///         Label { key: "cuda_version".to_string(), value: "12.0".to_string() },
///     ])
///     .tags(vec!["gpu".to_string(), "ml".to_string(), "inference".to_string()])
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgUpdateWorker {
    /// Identity of the account updating the worker
    /// This must match the original creator or be an admin account
    pub creator: String,
    
    /// Unique identifier of the worker to update
    /// This is the blockchain-assigned ID for the worker
    pub id: String,
    
    /// Updated human-readable name for the worker
    /// Used for display and searching purposes
    pub name: String,
    
    /// Updated detailed description of the worker's capabilities
    /// Provides context about the worker's hardware and purpose
    pub description: String,
    
    /// Updated number of CPU cores available in millicores (1000 = 1 CPU)
    /// Used for task scheduling and matching
    pub cpus: u64,
    
    /// Updated number of GPU units available in milli-GPU units (1000 = 1 GPU)
    /// Used for task scheduling and matching
    pub gpus: u64,
    
    /// Updated amount of memory available for tasks
    /// Used for task scheduling and matching
    pub memory: ByteSize,
    
    /// Updated amount of disk space available for tasks
    /// Used for task scheduling and matching
    pub disk: ByteSize,
    
    /// Updated key-value pairs for worker capabilities and metadata
    /// These provide structured information about the worker
    pub labels: Vec<Label>,
    
    /// Updated simple string tags for categorization
    /// These provide a basic way to group related workers
    pub tags: Vec<String>,
}

impl MsgUpdateWorkerBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgUpdateWorker>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{MsgUpdateWorkerBuilder, ByteSize, ByteUnit};
    /// use gevulot_rs::proto::gevulot::gevulot::Label;
    ///
    /// let msg = MsgUpdateWorkerBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .id("worker-123456".to_string())
    ///     .name("Updated GPU Worker".to_string())
    ///     .description("Updated high performance GPU node".to_string())
    ///     .cpus(32_000) // 32 cores
    ///     .gpus(4_000) // 4 GPUs
    ///     .memory(ByteSize::new(128, ByteUnit::Gigabyte))
    ///     .disk(ByteSize::new(2048, ByteUnit::Gigabyte))
    ///     .labels(vec![
    ///         Label { key: "gpu_type".to_string(), value: "nvidia_a100".to_string() },
    ///         Label { key: "cuda_version".to_string(), value: "12.0".to_string() },
    ///     ])
    ///     .tags(vec!["gpu".to_string(), "ml".to_string(), "inference".to_string()])
    ///     .build()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgUpdateWorker> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgUpdateWorker {
            creator: msg.creator,
            id: msg.id,
            name: msg.name,
            description: msg.description,
            cpus: msg.cpus,
            gpus: msg.gpus,
            memory: msg.memory.to_bytes(),
            disk: msg.disk.to_bytes(),
            labels: msg.labels,
            tags: msg.tags,
        })
    }
}

/// Builder for creating worker deletion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to request deletion of a previously
/// registered worker in the Gevulot network. Only the original creator or an admin
/// can delete a worker.
///
/// # Fields
///
/// * `creator` - Identity of the account requesting deletion
/// * `id` - Unique identifier of the worker to delete
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgDeleteWorkerBuilder;
///
/// let msg = MsgDeleteWorkerBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .id("worker-123456".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgDeleteWorker {
    /// Identity of the account requesting worker deletion
    /// This must match the original creator or be an admin account
    pub creator: String,
    
    /// Unique identifier of the worker to delete
    /// This is the blockchain-assigned ID for the worker
    pub id: String,
}

impl MsgDeleteWorkerBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgDeleteWorker>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgDeleteWorkerBuilder;
    ///
    /// let proto_msg = MsgDeleteWorkerBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .id("worker-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgDeleteWorker> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeleteWorker {
            creator: msg.creator,
            id: msg.id,
        })
    }
}

/// Builder for announcing worker node shutdown to the Gevulot blockchain.
///
/// This struct represents the parameters needed to inform the blockchain that
/// a worker node is exiting the network gracefully. This allows for proper cleanup
/// and reassignment of any in-progress tasks.
///
/// # Fields
///
/// * `creator` - Identity of the account announcing worker exit
/// * `worker_id` - Identifier of the worker that is exiting
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgAnnounceWorkerExitBuilder;
///
/// let msg = MsgAnnounceWorkerExitBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .worker_id("worker-123456".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgAnnounceWorkerExit {
    /// Identity of the account announcing worker exit
    /// This should match the worker's registered owner
    pub creator: String,
    
    /// Identifier of the worker that is exiting
    /// This is the blockchain-assigned ID for the worker
    pub worker_id: String,
}

impl MsgAnnounceWorkerExitBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgAnnounceWorkerExit>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgAnnounceWorkerExitBuilder;
    ///
    /// let proto_msg = MsgAnnounceWorkerExitBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .worker_id("worker-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgAnnounceWorkerExit> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgAnnounceWorkerExit {
            creator: msg.creator,
            worker_id: msg.worker_id,
        })
    }
} 