use serde::{Deserialize, Serialize};
use crate::proto::gevulot::gevulot;
use super::metadata::{Label, Metadata};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerSpec {
    pub cpus: i64,
    pub gpus: i64,
    pub memory: i64,
    pub disk: i64,
}

impl From<gevulot::WorkerSpec> for WorkerSpec {
    fn from(proto: gevulot::WorkerSpec) -> Self {
        WorkerSpec {
            cpus: proto.cpus as i64,
            gpus: proto.gpus as i64,
            memory: proto.memory as i64,
            disk: proto.disk as i64,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerStatus {
    #[serde(rename = "cpusUsed")]
    pub cpus_used: i64,
    #[serde(rename = "gpusUsed")]
    pub gpus_used: i64,
    #[serde(rename = "memoryUsed")]
    pub memory_used: i64,
    #[serde(rename = "diskUsed")]
    pub disk_used: i64,
    #[serde(rename = "exitAnnouncedAt")]
    pub exit_announced_at: i64,
}

impl From<gevulot::WorkerStatus> for WorkerStatus {
    fn from(proto: gevulot::WorkerStatus) -> Self {
        WorkerStatus {
            cpus_used: proto.cpus_used as i64,
            gpus_used: proto.gpus_used as i64,
            memory_used: proto.memory_used as i64,
            disk_used: proto.disk_used as i64,
            exit_announced_at: proto.exit_announced_at as i64,
        }
    }
}
