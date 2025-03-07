//! Worker model and related types for managing worker nodes.
//!
//! This module provides the core worker model used throughout the system, including:
//! - Worker specification and status
//! - Resource tracking (CPU, GPU, memory, disk)
//! - Metadata like tags and labels
//! - Protobuf serialization/deserialization

use super::{
    metadata::{Label, Metadata},
    ByteUnit, CoreUnit, DefaultFactorOneMegabyte,
};
use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Represents a complete worker definition with metadata, specification and status
///
/// # Examples
///
/// Creating a basic worker:
/// ```
/// use crate::models::Worker;
/// CoreUnit
/// let worker = serde_json::from_str::<Worker>(r#"{
///     "kind": "Worker",
///     "version": "v0",
///     "metadata": {
///         "name": "worker-1",
///         "tags": ["compute"]
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
/// use crate::proto::gevulot::gevulot;
/// use crate::models::Worker;
///
/// let proto_worker = gevulot::Worker {
///     metadata: Some(gevulot::Metadata {
///         name: "worker-1".to_string(),
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Worker {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: WorkerSpec,
    pub status: Option<WorkerStatus>,
}

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

/// Specification of worker resources and capabilities
///
/// Contains the maximum resources available on this worker:
/// - CPU cores
/// - GPU devices  
/// - Memory in bytes
/// - Disk space in bytes
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerSpec {
    pub cpus: CoreUnit,
    pub gpus: CoreUnit,
    pub memory: ByteUnit<DefaultFactorOneMegabyte>,
    pub disk: ByteUnit<DefaultFactorOneMegabyte>,
}

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

/// Current status and resource utilization of a worker
///
/// Tracks:
/// - Currently used resources (CPU, GPU, memory, disk)
/// - When the worker announced it will exit
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
