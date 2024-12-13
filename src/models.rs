use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: PinSpec,
    pub status: Option<PinStatus>,
}

impl From<gevulot::Pin> for Pin {
    fn from(proto: gevulot::Pin) -> Self {
        let mut spec: PinSpec = proto.spec.unwrap().into();
        spec.cid = proto
            .status
            .as_ref()
            .map(|s| s.cid.clone())
            .or_else(|| proto.metadata.as_ref().map(|m| m.id.clone()));
        Pin {
            kind: "Pin".to_string(),
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
                workflow_ref: None, //@TODO: implement workflow_ref
            },
            status: proto.status.map(|s| s.into()),
            spec,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinSpec {
    pub cid: Option<String>,
    pub bytes: i64,
    pub time: i64,
    pub redundancy: i64,
    #[serde(rename = "fallbackUrls")]
    pub fallback_urls: Option<Vec<String>>,
}

impl From<gevulot::PinSpec> for PinSpec {
    fn from(proto: gevulot::PinSpec) -> Self {
        PinSpec {
            cid: None,
            bytes: proto.bytes as i64,
            time: proto.time as i64,
            redundancy: proto.redundancy as i64,
            fallback_urls: Some(proto.fallback_urls),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinStatus {
    #[serde(rename = "assignedWorkers")]
    pub assigned_workers: Vec<String>,
    #[serde(rename = "workerAcks")]
    pub worker_acks: Vec<PinAck>,
    pub cid: Option<String>,
}

impl From<gevulot::PinStatus> for PinStatus {
    fn from(proto: gevulot::PinStatus) -> Self {
        PinStatus {
            assigned_workers: proto.assigned_workers,
            worker_acks: proto
                .worker_acks
                .into_iter()
                .map(|a| PinAck {
                    worker: a.worker,
                    block_height: a.block_height as i64,
                    success: a.success,
                    error: if a.error.is_empty() { None } else { Some(a.error) },
                })
                .collect(),
            cid: Some(proto.cid),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: TaskSpec,
    pub status: Option<TaskStatus>,
}

impl From<gevulot::Task> for Task {
    fn from(proto: gevulot::Task) -> Self {
        let workflow_ref = match proto.spec.as_ref() {
            Some(spec) if !spec.workflow_ref.is_empty() => Some(spec.workflow_ref.clone()),
            _ => None,
        };
        Task {
            kind: "Task".to_string(),
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
                workflow_ref,
            },
            spec: proto.spec.unwrap().into(),
            status: proto.status.map(|s| s.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpec {
    pub image: String,
    pub command: Vec<String>,
    pub args: Vec<String>,
    pub env: Vec<TaskEnv>,
    #[serde(rename = "inputContexts")]
    pub input_contexts: Vec<InputContext>,
    #[serde(rename = "outputContexts")]
    pub output_contexts: Vec<OutputContext>,
    pub resources: TaskResources,
    #[serde(rename = "storeStdout")]
    pub store_stdout: Option<bool>,
    #[serde(rename = "storeStderr")]
    pub store_stderr: Option<bool>,
}

impl From<gevulot::TaskSpec> for TaskSpec {
    fn from(proto: gevulot::TaskSpec) -> Self {
        TaskSpec {
            image: proto.image,
            command: proto.command,
            args: proto.args,
            env: proto
                .env
                .into_iter()
                .map(|e| TaskEnv {
                    name: e.name,
                    value: e.value,
                })
                .collect(),
            input_contexts: proto
                .input_contexts
                .into_iter()
                .map(|ic| InputContext {
                    source: ic.source,
                    target: ic.target,
                })
                .collect(),
            output_contexts: proto
                .output_contexts
                .into_iter()
                .map(|oc| OutputContext {
                    source: oc.source,
                    retention_period: oc.retention_period as i64,
                })
                .collect(),
            resources: TaskResources {
                cpus: proto.cpus,
                gpus: proto.gpus,
                memory: proto.memory,
                time: proto.time,
            },
            store_stdout: Some(proto.store_stdout),
            store_stderr: Some(proto.store_stderr),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEnv {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputContext {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputContext {
    pub source: String,
    #[serde(rename = "retentionPeriod")]
    pub retention_period: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResources {
    pub cpus: u64,
    pub gpus: u64,
    pub memory: u64,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatus {
    pub state: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    #[serde(rename = "completedAt")]
    pub completed_at: i64,
    #[serde(rename = "assignedWorkers")]
    pub assigned_workers: Vec<String>,
    #[serde(rename = "activeWorker")]
    pub active_worker: String,
    #[serde(rename = "exitCode")]
    pub exit_code: i64,
    #[serde(rename = "outputContexts")]
    pub output_contexts: Vec<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub error: String,
}

impl From<gevulot::TaskStatus> for TaskStatus {
    fn from(proto: gevulot::TaskStatus) -> Self {
        TaskStatus {
            state: match proto.state {
                0 => "Pending".to_string(),
                1 => "Running".to_string(),
                2 => "Declined".to_string(),
                3 => "Done".to_string(),
                4 => "Failed".to_string(),
                _ => "Unknown".to_string(),
            },
            created_at: proto.created_at as i64,
            started_at: proto.started_at as i64,
            completed_at: proto.completed_at as i64,
            assigned_workers: proto.assigned_workers,
            active_worker: proto.active_worker,
            exit_code: proto.exit_code,
            output_contexts: proto.output_contexts,
            error: proto.error,
            stdout: if proto.stdout.is_empty() {
                None
            } else {
                Some(proto.stdout)
            },
            stderr: if proto.stderr.is_empty() {
                None
            } else {
                Some(proto.stderr)
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workflow {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: WorkflowSpec,
    pub status: Option<WorkflowStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStage {
    pub tasks: Vec<TaskSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowSpec {
    pub stages: Vec<WorkflowStage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStageStatus {
    pub task_ids: Vec<String>,
    pub finished_tasks: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStatus {
    pub state: String,
    pub current_stage: u64,
    pub stages: Vec<WorkflowStageStatus>,
}

impl From<gevulot::Workflow> for Workflow {
    fn from(proto: gevulot::Workflow) -> Self {
        Workflow {
            kind: "Workflow".to_string(),
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
            spec: proto.spec.map(|s| s.into()).unwrap(),
            status: proto.status.map(|s| s.into()),
        }
    }
}

impl From<gevulot::WorkflowSpec> for WorkflowSpec {
    fn from(proto: gevulot::WorkflowSpec) -> Self {
        WorkflowSpec {
            stages: proto
                .stages
                .into_iter()
                .map(|stage| WorkflowStage {
                    tasks: stage.tasks.into_iter().map(|t| t.into()).collect(),
                })
                .collect(),
        }
    }
}

impl From<gevulot::WorkflowStatus> for WorkflowStatus {
    fn from(proto: gevulot::WorkflowStatus) -> Self {
        WorkflowStatus {
            state: match proto.state {
                0 => "Pending".to_string(),
                1 => "Running".to_string(),
                2 => "Done".to_string(),
                3 => "Failed".to_string(),
                _ => "Unknown".to_string(),
            },
            current_stage: proto.current_stage,
            stages: proto
                .stages
                .into_iter()
                .map(|s| WorkflowStageStatus {
                    task_ids: s.task_ids,
                    finished_tasks: s.finished_tasks,
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub id: Option<String>,
    pub name: String,
    pub creator: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub labels: Vec<Label>,
    #[serde(rename = "workflowRef")]
    pub workflow_ref: Option<String>, // Only used in TaskMetadata
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub key: String,
    pub value: String,
}

impl From<gevulot::Label> for Label {
    fn from(proto: gevulot::Label) -> Self {
        Label {
            key: proto.key,
            value: proto.value,
        }
    }
}

impl From<Label> for gevulot::Label {
    fn from(val: Label) -> Self {
        gevulot::Label {
            key: val.key,
            value: val.value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinAck {
    pub worker: String,
    #[serde(rename = "blockHeight")]
    pub block_height: i64,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Generic {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: serde_json::Value,
    pub status: Option<serde_json::Value>,
}
