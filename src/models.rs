use std::str::FromStr;

use crate::proto::gevulot::gevulot;
use bytesize::ByteSize;
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
    #[serde(default)]
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

#[derive(Serialize, Debug)]
pub struct PinSpec {
    #[serde(default)]
    pub cid: Option<String>,
    pub bytes: ComputeUnit,
    pub time: ComputeUnit,
    pub redundancy: i64,
    #[serde(rename = "fallbackUrls", default)]
    pub fallback_urls: Option<Vec<String>>,
}

impl<'de> Deserialize<'de> for PinSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create an intermediate struct for initial deserialization
        #[derive(Deserialize)]
        struct PinSpecHelper {
            #[serde(default)]
            cid: Option<String>,
            bytes: ComputeUnit,
            time: ComputeUnit,
            redundancy: Option<i64>,
            #[serde(rename = "fallbackUrls", default)]
            fallback_urls: Option<Vec<String>>,
        }

        // Deserialize to the helper struct
        let helper = PinSpecHelper::deserialize(deserializer)?;

        // Validate the fields
        if helper.cid.is_none() {
            // If no CID, must have non-empty fallback URLs
            match &helper.fallback_urls {
                None => {
                    return Err(serde::de::Error::custom(
                        "Either cid or fallbackUrls must be specified",
                    ))
                }
                Some(urls) if urls.is_empty() => {
                    return Err(serde::de::Error::custom(
                        "fallbackUrls must contain at least one URL when no cid is specified",
                    ))
                }
                _ => {}
            }
        }

        let redundancy = helper.redundancy.unwrap_or(1);
        // Convert to final struct
        Ok(PinSpec {
            cid: helper.cid,
            bytes: helper.bytes,
            time: helper.time,
            redundancy,
            fallback_urls: helper.fallback_urls,
        })
    }
}

impl From<gevulot::PinSpec> for PinSpec {
    fn from(proto: gevulot::PinSpec) -> Self {
        PinSpec {
            cid: None,
            bytes: (proto.bytes as i64).into(),
            time: (proto.time as i64).into(),
            redundancy: proto.redundancy as i64,
            fallback_urls: Some(proto.fallback_urls),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinStatus {
    #[serde(rename = "assignedWorkers", default)]
    pub assigned_workers: Vec<String>,
    #[serde(rename = "workerAcks", default)]
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
                    error: if a.error.is_empty() {
                        None
                    } else {
                        Some(a.error)
                    },
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
    #[serde(default)]
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
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: Vec<TaskEnv>,
    #[serde(rename = "inputContexts", default)]
    pub input_contexts: Vec<InputContext>,
    #[serde(rename = "outputContexts", default)]
    pub output_contexts: Vec<OutputContext>,
    pub resources: TaskResources,
    #[serde(rename = "storeStdout", default)]
    pub store_stdout: bool,
    #[serde(rename = "storeStderr", default)]
    pub store_stderr: bool,
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
                cpus: (proto.cpus as i64).into(),
                gpus: (proto.gpus as i64).into(),
                memory: (proto.memory as i64).into(),
                time: (proto.time as i64).into(),
            },
            store_stdout: proto.store_stdout,
            store_stderr: proto.store_stderr,
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
    pub cpus: ComputeUnit,
    pub gpus: ComputeUnit,
    pub memory: ComputeUnit,
    pub time: ComputeUnit,
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
    pub exit_code: Option<i64>,
    #[serde(rename = "outputContexts")]
    pub output_contexts: Vec<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub error: Option<String>,
}

impl From<gevulot::TaskStatus> for TaskStatus {
    fn from(proto: gevulot::TaskStatus) -> Self {
        let mut exit_code = None;
        let state = match proto.state {
            0 => "Pending".to_string(),
            1 => "Running".to_string(),
            2 => "Declined".to_string(),
            3 => {
                exit_code = Some(proto.exit_code);
                "Done".to_string()
            }
            4 => {
                exit_code = Some(proto.exit_code);
                "Failed".to_string()
            }
            _ => "Unknown".to_string(),
        };
        let error = if proto.error.is_empty() {
            None
        } else {
            Some(proto.error)
        };
        let stdout = if proto.stdout.is_empty() {
            None
        } else {
            Some(proto.stdout)
        };
        let stderr = if proto.stderr.is_empty() {
            None
        } else {
            Some(proto.stderr)
        };
        TaskStatus {
            state,
            created_at: proto.created_at as i64,
            started_at: proto.started_at as i64,
            completed_at: proto.completed_at as i64,
            assigned_workers: proto.assigned_workers,
            active_worker: proto.active_worker,
            exit_code,
            output_contexts: proto.output_contexts,
            error,
            stdout,
            stderr,
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

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ComputeUnit {
    Number(i64),
    String(String),
}

impl ComputeUnit {
    pub fn as_number(&self) -> Result<i64, String> {
        match self {
            ComputeUnit::Number(n) => Ok(*n),
            ComputeUnit::String(s) => Self::parse_string(s),
        }
    }

    fn parse_string(s: &str) -> Result<i64, String> {
        let numeric: String = s.chars().take_while(|c| c.is_digit(10)).collect();
        let unit = s[numeric.len()..].to_lowercase().replace(" ", "");

        if !unit.is_empty() {
            if let Ok(duration) = humantime::parse_duration(s) {
                return Ok(duration.as_secs() as i64);
            }

            if let Ok(bytes) = s.parse::<ByteSize>() {
                return Ok(bytes.0 as i64);
            }
        }

        let base: i64 = numeric
            .parse()
            .map_err(|e| format!("Invalid number: {}", e))?;
        Ok(base
            * match unit.as_str() {
                "byte" | "bytes" => 1, // this is not covered by bytesize crate
                "cpu" | "cpus" | "core" | "cores" => 1000,
                "gpu" | "gpus" => 1000,
                "mcpu" | "mcpus" | "millicpu" | "millicpus" => 1,
                "mgpu" | "mgpus" | "milligpu" | "milligpus" => 1,
                "mcore" | "mcores" | "millicore" | "millicores" => 1,
                "" => 1,
                _ => return Err(format!("Invalid unit: {}", unit)),
            })
    }

    pub fn as_string(&self) -> String {
        match self {
            ComputeUnit::Number(n) => n.to_string(),
            ComputeUnit::String(s) => s.clone(),
        }
    }
}

impl FromStr for ComputeUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = Self::parse_string(s)?;
        Ok(ComputeUnit::String(s.to_string()))
    }
}

impl From<i64> for ComputeUnit {
    fn from(val: i64) -> Self {
        ComputeUnit::Number(val)
    }
}

impl TryFrom<ComputeUnit> for i64 {
    type Error = String;

    fn try_from(val: ComputeUnit) -> Result<Self, Self::Error> {
        match &val {
            ComputeUnit::Number(n) => Ok(*n),
            ComputeUnit::String(_) => val.as_number(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    mod compute_unit_tests {
        use super::*;

        #[test]
        fn test_raw_number_deserialization() {
            // Test integer number
            let num = serde_json::from_value::<ComputeUnit>(json!(1234)).unwrap();
            assert_eq!(num.as_number(), Ok(1234));

            // Test string number
            let num = serde_json::from_value::<ComputeUnit>(json!("1234")).unwrap();
            assert_eq!(num.as_number(), Ok(1234));
        }

        #[test]
        fn test_unit_deserialization() {
            // Test kilobytes
            let str = serde_json::from_value::<ComputeUnit>(json!("1234KiB")).unwrap();
            assert_eq!(str.as_number(), Ok(1234 * 1024));
            assert_eq!(str.as_string(), "1234KiB");

            // Test megabytes
            let str = serde_json::from_value::<ComputeUnit>(json!("1234MiB")).unwrap();
            assert_eq!(str.as_number(), Ok(1234 * 1024 * 1024));
            assert_eq!(str.as_string(), "1234MiB");

            // Test minutes
            let str = serde_json::from_value::<ComputeUnit>(json!("60min")).unwrap();
            assert_eq!(str.as_number(), Ok(60 * 60));
            assert_eq!(str.as_string(), "60min");
        }

        #[test]
        fn test_serialization() {
            // Test string serialization
            let str = serde_json::to_string(&ComputeUnit::String("1234KiB".to_string())).unwrap();
            assert_eq!(str, "\"1234KiB\"");

            // Test number serialization
            let str = serde_json::to_string(&ComputeUnit::Number(1234)).unwrap();
            assert_eq!(str, "1234");
        }

        #[test]
        fn test_string_parsing() {
            // Test valid parse
            let res: Result<ComputeUnit, _> = "123KiB".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(123 * 1024));

            // Test invalid parse
            let res: Result<ComputeUnit, String> = "123 this is wrong".parse();
            assert!(res.is_err());
        }

        #[test]
        fn test_duration_parsing() {
            let res: Result<ComputeUnit, String> = "1hr".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(60 * 60));

            let res: Result<ComputeUnit, String> = "1hr 30min 10sec".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(60 * 60 + 30 * 60 + 10));
        }

        #[test]
        fn test_bytesize_parsing() {
            let res: Result<ComputeUnit, String> = "1byte".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1));

            let res: Result<ComputeUnit, String> = "1kb".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1000));

            let res: Result<ComputeUnit, String> = "1mb".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1000 * 1000));

            let res: Result<ComputeUnit, String> = "1gb".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1000 * 1000 * 1000));

            let res: Result<ComputeUnit, String> = "1kiB".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1024));

            let res: Result<ComputeUnit, String> = "1MiB".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1024 * 1024));

            let res: Result<ComputeUnit, String> = "1GiB".parse();
            assert!(res.is_ok());
            assert_eq!(res.unwrap().as_number(), Ok(1024 * 1024 * 1024));
        }
    }

    #[test]
    fn test_parse_task_with_units() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "metadata": {
                "name": "Test Task",
                "creator": "test",
                "description": "Test Task",
                "tags": [],
                "labels": []
            },
            "spec": {
                "image": "test",
                "command": ["test"],
                "args": ["test"],
                "env": [],
                "inputContexts": [],
                "outputContexts": [],
                "resources": {
                    "cpus": "1000mcpu",
                    "gpus": "1000mgpu",
                    "memory": "1024mb",
                    "time": "1hr"
                }
            }
        }))
        .unwrap();

        assert_eq!(
            task.spec.resources.cpus,
            ComputeUnit::String("1000mcpu".to_string())
        );
        assert_eq!(
            task.spec.resources.gpus,
            ComputeUnit::String("1000mgpu".to_string())
        );
        assert_eq!(
            task.spec.resources.memory,
            ComputeUnit::String("1024mb".to_string())
        );
        assert_eq!(
            task.spec.resources.time,
            ComputeUnit::String("1hr".to_string())
        );
    }

    #[test]
    fn test_parse_task_without_units() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "metadata": {
                "name": "Test Task",
                "creator": "test",
                "description": "Test Task",
                "tags": [],
                "labels": []
            },
            "spec": {
                "image": "test",
                "command": ["test"],
                "args": ["test"],
                "env": [],
                "inputContexts": [],
                "outputContexts": [],
                "resources": {
                    "cpus": 1000,
                    "gpus": 1000,
                    "memory": 1024,
                    "time": 1
                }
            }
        }))
        .unwrap();

        assert_eq!(task.spec.resources.cpus.as_number(), Ok(1000));
        assert_eq!(task.spec.resources.gpus.as_number(), Ok(1000));
        assert_eq!(task.spec.resources.memory.as_number(), Ok(1024));
        assert_eq!(task.spec.resources.time.as_number(), Ok(1));
    }

    #[test]
    fn test_parse_task_without_much() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "test",
                "resources": {
                    "cpus": "1000 MCpu",
                    "gpus": "1000 MGpu",
                    "memory": "1024 MiB",
                    "time": "1 hr"
                }
            }
        }))
        .expect("Failed to parse task");

        assert_eq!(task.spec.resources.cpus.as_number(), Ok(1000));
        assert_eq!(task.spec.resources.gpus.as_number(), Ok(1000));
        assert_eq!(
            task.spec.resources.memory.as_number(),
            Ok(1024 * 1024 * 1024)
        );
        assert_eq!(task.spec.resources.time.as_number(), Ok(60 * 60));
    }

    #[test]
    fn test_parse_task_yaml() {
        let task = serde_yaml::from_str::<Task>(
            r#"
            kind: Task
            version: v0
            spec:
                image: test
                resources:
                    cpus: 1000 MCpu
                    gpus: 1000 MGpu
                    memory: 1024 MiB
                    time: 1 hr
        "#,
        )
        .expect("Failed to parse task");

        assert_eq!(task.spec.resources.cpus.as_number(), Ok(1000));
        assert_eq!(task.spec.resources.gpus.as_number(), Ok(1000));
        assert_eq!(
            task.spec.resources.memory.as_number(),
            Ok(1024 * 1024 * 1024)
        );
        assert_eq!(task.spec.resources.time.as_number(), Ok(60 * 60));
    }

    #[test]
    fn test_parse_complete_task() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "metadata": {
                "id": "test-id",
                "name": "Test Task",
                "creator": "test-creator",
                "description": "Test Task Description",
                "tags": ["tag1", "tag2"],
                "labels": [
                    {"key": "label1", "value": "value1"},
                    {"key": "label2", "value": "value2"}
                ],
                "workflowRef": "test-workflow"
            },
            "spec": {
                "image": "test-image",
                "command": ["cmd1", "cmd2"],
                "args": ["arg1", "arg2"],
                "env": [
                    {"name": "ENV1", "value": "value1"},
                    {"name": "ENV2", "value": "value2"}
                ],
                "inputContexts": [
                    {"source": "src1", "target": "tgt1"},
                    {"source": "src2", "target": "tgt2"}
                ],
                "outputContexts": [
                    {"source": "out1", "retentionPeriod": 100},
                    {"source": "out2", "retentionPeriod": 200}
                ],
                "resources": {
                    "cpus": "2cores",
                    "gpus": "1gpu",
                    "memory": "2GiB",
                    "time": "1hr"
                },
                "storeStdout": true,
                "storeStderr": true
            },
            "status": {
                "state": "Running",
                "createdAt": 1000,
                "startedAt": 1001,
                "completedAt": 1002,
                "assignedWorkers": ["worker1", "worker2"],
                "activeWorker": "worker1",
                "outputContexts": ["out1-cid", "out2-cid"],
            }
        }))
        .unwrap();

        // Verify metadata
        assert_eq!(task.metadata.id, Some("test-id".to_string()));
        assert_eq!(task.metadata.name, "Test Task");
        assert_eq!(task.metadata.creator, Some("test-creator".to_string()));
        assert_eq!(task.metadata.description, "Test Task Description");
        assert_eq!(task.metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(task.metadata.labels.len(), 2);
        assert_eq!(task.metadata.labels[0].key, "label1");
        assert_eq!(task.metadata.labels[0].value, "value1");
        assert_eq!(
            task.metadata.workflow_ref,
            Some("test-workflow".to_string())
        );

        // Verify spec
        assert_eq!(task.spec.image, "test-image");
        assert_eq!(task.spec.command, vec!["cmd1", "cmd2"]);
        assert_eq!(task.spec.args, vec!["arg1", "arg2"]);

        assert_eq!(task.spec.env.len(), 2);
        assert_eq!(task.spec.env[0].name, "ENV1");
        assert_eq!(task.spec.env[0].value, "value1");

        assert_eq!(task.spec.input_contexts.len(), 2);
        assert_eq!(task.spec.input_contexts[0].source, "src1");
        assert_eq!(task.spec.input_contexts[0].target, "tgt1");

        assert_eq!(task.spec.output_contexts.len(), 2);
        assert_eq!(task.spec.output_contexts[0].source, "out1");
        assert_eq!(task.spec.output_contexts[0].retention_period, 100);

        assert_eq!(task.spec.resources.cpus.as_number(), Ok(2000));
        assert_eq!(task.spec.resources.gpus.as_number(), Ok(1000));
        assert_eq!(
            task.spec.resources.memory.as_number(),
            Ok(2 * 1024 * 1024 * 1024)
        );
        assert_eq!(task.spec.resources.time.as_number(), Ok(3600));

        assert!(task.spec.store_stdout);
        assert!(task.spec.store_stderr);

        // Verify status
        let status = task.status.unwrap();
        assert_eq!(status.state, "Running");
        assert_eq!(status.created_at, 1000);
        assert_eq!(status.started_at, 1001);
        assert_eq!(status.completed_at, 1002);
        assert_eq!(status.assigned_workers, vec!["worker1", "worker2"]);
        assert_eq!(status.active_worker, "worker1");
        assert_eq!(status.exit_code, None);
        assert_eq!(status.output_contexts, vec!["out1-cid", "out2-cid"]);
    }

    #[test]
    fn test_parse_pin() {
        let pin = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "metadata": {
                "name": "Test Pin",
                "creator": "test",
                "description": "Test Pin Description",
                "tags": ["tag1", "tag2"],
                "labels": [
                    {
                        "key": "label1",
                        "value": "value1"
                    },
                    {
                        "key": "label2",
                        "value": "value2"
                    }
                ],
                "workflowRef": "test-workflow"
            },
            "spec": {
                "cid": "test-cid",
                "bytes": "1234KiB",
                "time": "24h",
                "redundancy": 3,
                "fallbackUrls": ["url1", "url2"]
            },
            "status": {
                "assignedWorkers": ["worker1", "worker2"],
                "workerAcks": [
                    {
                        "worker": "worker1",
                        "blockHeight": 1000,
                        "success": true,
                        "error": null
                    },
                    {
                        "worker": "worker2",
                        "blockHeight": 1001,
                        "success": false,
                        "error": "Failed to pin"
                    }
                ],
                "cid": "test-cid"
            }
        }))
        .unwrap();

        // Verify metadata
        assert_eq!(pin.kind, "Pin");
        assert_eq!(pin.version, "v0");
        assert_eq!(pin.metadata.name, "Test Pin");
        assert_eq!(pin.metadata.creator, Some("test".to_string()));
        assert_eq!(pin.metadata.description, "Test Pin Description");
        assert_eq!(pin.metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(pin.metadata.labels.len(), 2);
        assert_eq!(pin.metadata.labels[0].key, "label1");
        assert_eq!(pin.metadata.labels[0].value, "value1");
        assert_eq!(pin.metadata.workflow_ref, Some("test-workflow".to_string()));

        // Verify spec
        assert_eq!(pin.spec.cid, Some("test-cid".to_string()));
        assert_eq!(pin.spec.bytes.as_number(), Ok(1234 * 1024));
        assert_eq!(pin.spec.time.as_number(), Ok(24 * 60 * 60));
        assert_eq!(pin.spec.redundancy, 3);
        assert_eq!(
            pin.spec.fallback_urls,
            Some(vec!["url1".to_string(), "url2".to_string()])
        );

        // Verify status
        let status = pin.status.unwrap();
        assert_eq!(status.assigned_workers, vec!["worker1", "worker2"]);
        assert_eq!(status.worker_acks.len(), 2);
        assert_eq!(status.worker_acks[0].worker, "worker1");
        assert_eq!(status.worker_acks[0].block_height, 1000);
        assert_eq!(status.worker_acks[0].success, true);
        assert_eq!(status.worker_acks[0].error, None);
        assert_eq!(status.cid, Some("test-cid".to_string()));
    }

    #[test]
    fn test_parse_pin_with_the_bare_minimum() {
        let pin = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "cid": "test-cid",
                "bytes": "1234KiB",
                "time": "24h",
            }
        }))
        .unwrap();

        assert_eq!(pin.spec.cid, Some("test-cid".to_string()));
        assert_eq!(pin.spec.bytes.as_number(), Ok(1234 * 1024));
        assert_eq!(pin.spec.time.as_number(), Ok(24 * 60 * 60));
        assert_eq!(pin.spec.redundancy, 1);
        assert_eq!(pin.spec.fallback_urls, None);
    }

    #[test]
    fn test_pin_requires_cid_or_fallback_urls() {
        // Should fail without either cid or fallback_urls
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_err());

        // Should succeed with just cid
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "cid": "test-cid",
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_ok());

        // Should succeed with just fallback_urls
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "fallbackUrls": ["url1", "url2"],
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_ok());

        // Should fail with empty fallback_urls array
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "fallbackUrls": [],
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_err());
    }
}
