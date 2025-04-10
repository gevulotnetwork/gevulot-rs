//! Worker model and related types for managing worker nodes in the Gevulot network.
//!
//! This module provides the core worker model used throughout the system, including:
//! - Worker specification and status
//! - Resource tracking (CPU, GPU, memory, disk)
//! - Metadata like tags and labels
//! - Protobuf serialization/deserialization
//!
//! Workers are computing nodes in the Gevulot network that process tasks. They advertise
//! their resources and capabilities, and the network allocates tasks to them based on
//! resource requirements and availability.
//!
//! # Key Components
//!
//! - [`Worker`] - The complete worker definition including metadata, specification, and status
//! - [`WorkerSpec`] - Defines available resources on a worker (CPUs, GPUs, memory, disk)
//! - [`WorkerStatus`] - Tracks current resource utilization and worker lifecycle status
//!
//! # Common Operations
//!
//! - Creating a worker definition for registration
//! - Querying available workers and their capabilities 
//! - Monitoring worker resource utilization
//! - Converting between JSON/protobuf representations and internal models

use super::{
    metadata::{Label, Metadata},
    ByteUnit, CoreUnit, DefaultFactorOneMegabyte,
};
use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Represents a complete worker definition with metadata, specification and status.
///
/// The `Worker` struct is the primary representation of a worker node in the Gevulot network.
/// It combines identifying information, resource capabilities, and current status in a
/// single structure.
///
/// # Fields
///
/// * `kind` - Entity type identifier, always "Worker" for worker entities
/// * `version` - Schema version, typically "v0"
/// * `metadata` - Worker identification, description, and classification information
/// * `spec` - Resource specifications including CPU, GPU, memory and disk capacities
/// * `status` - Optional current status including resource utilization and lifecycle state
///
/// # Examples
///
/// Creating a basic worker:
/// ```
/// use gevulot_rs::models::Worker;
///
/// let worker = serde_json::from_str::<Worker>(r#"{
///     "kind": "Worker",
///     "version": "v0",
///     "metadata": {
///         "name": "worker-1",
///         "tags": ["compute"],
///         "description": "Worker #1",
///         "labels": [
///             {
///                 "key": "my-key",
///                 "value": "my-label"
///             }
///         ]
///     },
///     "spec": {
///         "cpus": "8 cores",
///         "gpus": "1 gpu",
///         "memory": "16 GiB",
///         "disk": "100 GiB"
///     }
/// }"#).unwrap();
/// ```
///
/// Converting from protobuf:
/// ```
/// use gevulot_rs::proto::gevulot::gevulot;
/// use gevulot_rs::models::Worker;
///
/// let proto_worker = gevulot::Worker {
///     metadata: Some(gevulot::Metadata {
///         name: "worker-1".to_string(),
///         desc: "Worker #1".to_string(),
///         ..Default::default()
///     }),
///     spec: Some(gevulot::WorkerSpec {
///         cpus: 8,
///         gpus: 1,
///         memory: 16000000000,
///         disk: 100000000000,
///     }),
///     ..Default::default()
/// };
///
/// let worker = Worker::from(proto_worker);
/// ```
///
/// # Worker Identification
///
/// Workers can be identified by:
/// - A unique ID (assigned by the system)
/// - A human-readable name
/// - Tags for grouping similar workers
/// - Labels for custom metadata (key-value pairs)
#[derive(Serialize, Deserialize, Debug)]
pub struct Worker {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: WorkerSpec,
    pub status: Option<WorkerStatus>,
}

/// Converts a protobuf Worker message to the internal Worker model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
impl From<gevulot::Worker> for Worker {
    fn from(proto: gevulot::Worker) -> Self {
        // Convert protobuf worker to our internal worker model
        Worker {
            kind: "Worker".to_string(),
            version: "v0".to_string(),
            metadata: Metadata {
                id: proto.metadata.as_ref().map(|m| m.id.clone()),
                name: proto
                    .metadata
                    .as_ref()
                    .map(|m| m.name.clone())
                    .unwrap_or_default(),
                creator: proto.metadata.as_ref().map(|m| m.creator.clone()),
                description: proto
                    .metadata
                    .as_ref()
                    .map(|m| m.desc.clone())
                    .unwrap_or_default(),
                tags: proto
                    .metadata
                    .as_ref()
                    .map(|m| m.tags.clone())
                    .unwrap_or_default(),
                labels: proto
                    .metadata
                    .as_ref()
                    .map(|m| m.labels.clone())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|l| Label {
                        key: l.key,
                        value: l.value,
                    })
                    .collect(),
                workflow_ref: None,
            },
            spec: proto.spec.unwrap().into(),
            status: proto.status.map(|s| s.into()),
        }
    }
}

/// Specification of worker resources and capabilities.
///
/// Contains the maximum resources available on this worker node, which are used for:
/// - Advertising available resources to the network
/// - Task scheduling and allocation decisions
/// - Resource capacity planning
///
/// # Fields
///
/// * `cpus` - Number of CPU cores available on the worker, represented as a string like "8 cores"
/// * `gpus` - Number of GPU devices available on the worker, represented as a string like "2 gpus" 
/// * `memory` - Total memory available on the worker, with human-readable formatting (e.g., "16 GiB")
/// * `disk` - Total disk space available on the worker, with human-readable formatting (e.g., "100 GiB")
///
/// # Example
///
/// ```
/// use gevulot_rs::models::{WorkerSpec, CoreUnit, ByteUnit, DefaultFactorOneMegabyte};
///
/// let spec = WorkerSpec {
///     cpus: CoreUnit::from(4),
///     gpus: CoreUnit::from(1),
///     memory: ByteUnit::<DefaultFactorOneMegabyte>::from(8 * 1024),
///     disk: ByteUnit::<DefaultFactorOneMegabyte>::from(50 * 1024),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkerSpec {
    pub cpus: CoreUnit,
    pub gpus: CoreUnit,
    pub memory: ByteUnit<DefaultFactorOneMegabyte>,
    pub disk: ByteUnit<DefaultFactorOneMegabyte>,
}

/// Converts a protobuf WorkerSpec message to the internal WorkerSpec model.
///
/// This implementation handles the conversion from numeric protobuf values to
/// human-readable string representations with appropriate units.
impl From<gevulot::WorkerSpec> for WorkerSpec {
    fn from(proto: gevulot::WorkerSpec) -> Self {
        // Convert protobuf spec to internal spec
        WorkerSpec {
            cpus: proto.cpus.into(),
            gpus: proto.gpus.into(),
            memory: proto.memory.into(),
            disk: proto.disk.into(),
        }
    }
}

/// Current status and resource utilization of a worker.
///
/// Tracks the current state of a worker node including:
/// - Currently used resources (CPU, GPU, memory, disk)
/// - Lifecycle information such as when the worker will exit
///
/// This information is used for:
/// - Task scheduling decisions
/// - Resource utilization monitoring
/// - Worker health tracking
///
/// # Fields
///
/// * `cpus_used` - Number of CPU cores currently in use, in human-readable format
/// * `gpus_used` - Number of GPU devices currently in use, in human-readable format
/// * `memory_used` - Amount of memory currently in use, with human-readable formatting
/// * `disk_used` - Amount of disk space currently in use, with human-readable formatting
/// * `exit_announced_at` - Unix timestamp (in seconds) when the worker announced it will exit
///
/// # Example
///
/// ```
/// use gevulot_rs::models::{WorkerStatus, CoreUnit, ByteUnit, DefaultFactorOneMegabyte};
///
/// let status = WorkerStatus {
///     cpus_used: CoreUnit::from(2),
///     gpus_used: CoreUnit::from(1),
///     memory_used: "4GiB".parse::<ByteUnit<DefaultFactorOneMegabyte>>().unwrap(),
///     disk_used: "20GiB".parse::<ByteUnit<DefaultFactorOneMegabyte>>().unwrap(),
///     exit_announced_at: 1671534000,
/// };
/// ```
///
/// Status update from worker, including current resource usage.
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{WorkerStatus, CoreUnit, ByteUnit, DefaultFactorOneMegabyte};
///
/// let status = WorkerStatus {
///     cpus_used: CoreUnit::from(1),
///     gpus_used: CoreUnit::from(0),
///     memory_used: ByteUnit::<DefaultFactorOneMegabyte>::from(512),
///     disk_used: ByteUnit::<DefaultFactorOneMegabyte>::from(1024),
///     exit_announced_at: 0,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerStatus {
    #[serde(rename = "cpusUsed")]
    pub cpus_used: CoreUnit,
    #[serde(rename = "gpusUsed")]
    pub gpus_used: CoreUnit,
    #[serde(rename = "memoryUsed")]
    pub memory_used: ByteUnit<DefaultFactorOneMegabyte>,
    #[serde(rename = "diskUsed")]
    pub disk_used: ByteUnit<DefaultFactorOneMegabyte>,
    #[serde(rename = "exitAnnouncedAt")]
    pub exit_announced_at: i64,
}

/// Converts a protobuf WorkerStatus message to the internal WorkerStatus model.
///
/// This implementation handles the conversion from numeric protobuf values to
/// human-readable string representations with appropriate units, and manages
/// the timestamp conversions.
impl From<gevulot::WorkerStatus> for WorkerStatus {
    fn from(proto: gevulot::WorkerStatus) -> Self {
        // Convert protobuf status to internal status
        WorkerStatus {
            cpus_used: proto.cpus_used.into(),
            gpus_used: proto.gpus_used.into(),
            memory_used: proto.memory_used.into(),
            disk_used: proto.disk_used.into(),
            exit_announced_at: proto.exit_announced_at as i64,
        }
    }
}
