//! Task model and related types for managing computational task execution in Gevulot.
//!
//! This module provides the core task model used throughout the Gevulot system, including:
//! - Task specification and status tracking
//! - Computational resource requirements (CPU, GPU, memory, time)
//! - Input/output data context handling
//! - Environment variable configuration
//! - Metadata management for task identification and filtering
//!
//! Tasks are the fundamental execution unit in Gevulot and represent a containerized
//! computation job that runs on worker nodes. Tasks can be submitted individually or
//! as part of a workflow.
//!
//! # Task Lifecycle
//!
//! A typical task follows this lifecycle:
//! 1. **Created** - Task is submitted to the blockchain and assigned an ID
//! 2. **Pending** - Task is waiting for worker assignment
//! 3. **Running** - Task is executing on a worker node
//! 4. **Completed** - Task has finished execution (with success or failure)
//!
//! # Task Resource Specification
//!
//! Resources for tasks are specified using human-readable units:
//! - CPU: "1cpu", "500mcpu" (millicores)
//! - GPU: "1gpu", "500mgpu" (milli-GPU units)
//! - Memory: "1gb", "512mb"
//! - Time: "1h", "30m", "90s"

use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Represents a complete task definition with metadata, specification and status.
///
/// A `Task` is the fundamental execution unit in Gevulot, representing a containerized
/// computation that runs on worker nodes. It contains all the information needed to 
/// schedule, execute, and track a computational job.
///
/// # Fields
///
/// * `kind` - Type identifier, always "Task" for serialization format identification
/// * `version` - API version of the task format, currently "v0"
/// * `metadata` - Additional descriptive information about the task
/// * `spec` - Core specification of what to execute and required resources
/// * `status` - Runtime information tracking the task's execution state
///
/// # Examples
///
/// ## Creating a basic task
///
/// ```
/// use gevulot_rs::models::Task;
///
/// let task = serde_json::from_str::<Task>(r#"{
///     "kind": "Task",
///     "version": "v0",
///     "spec": {
///         "image": "ubuntu:latest",
///         "command": ["echo", "hello"],
///         "resources": {
///             "cpus": "1cpu",
///             "gpus": "0gpu",
///             "memory": "512mb",
///             "time": "1h"
///         }
///     }
/// }"#).unwrap();
/// ```
///
/// ## Task with input/output contexts
///
/// Input contexts allow mounting data into the container, while output contexts
/// capture results:
///
/// ```
/// use gevulot_rs::models::Task;
///
/// let task = serde_json::from_str::<Task>(r#"{
///     "kind": "Task",
///     "version": "v0",
///     "spec": {
///         "image": "processor:v1",
///         "command": ["python", "process.py"],
///         "inputContexts": [{
///             "source": "input-data",
///             "target": "/data"
///         }],
///         "outputContexts": [{
///             "source": "/results",
///             "retentionPeriod": 86400
///         }],
///         "resources": {
///             "cpus": "2cpu",
///             "gpus": "1gpu",
///             "memory": "4gb",
///             "time": "2h"
///         }
///     }
/// }"#).unwrap();
/// ```
///
/// ## Task with environment variables and output capture
///
/// ```
/// use gevulot_rs::models::Task;
///
/// let task = serde_json::from_str::<Task>(r#"{
///     "kind": "Task",
///     "version": "v0",
///     "metadata": {
///         "name": "Data Analysis Job",
///         "description": "Processes input data with configurable parameters",
///         "tags": ["analysis", "data-processing"],
///         "labels": [
///             {"key": "priority", "value": "high"},
///             {"key": "department", "value": "research"}
///         ]
///     },
///     "spec": {
///         "image": "data-analyzer:v2",
///         "command": ["analyze.sh"],
///         "env": [
///             {"name": "LOG_LEVEL", "value": "debug"},
///             {"name": "BATCH_SIZE", "value": "1000"}
///         ],
///         "storeStdout": true,
///         "storeStderr": true,
///         "resources": {
///             "cpus": "4cpu",
///             "gpus": "0gpu",
///             "memory": "8gb",
///             "time": "4h"
///         }
///     }
/// }"#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// Type identifier, always "Task" for this struct
    /// Used for type identification in serialized form
    pub kind: String,
    
    /// API version for the task format, currently "v0"
    /// This allows for future schema evolution
    pub version: String,
    
    /// Task metadata like name, description, tags, and identifying information
    /// Used for filtering, searching, and referencing tasks
    #[serde(default)]
    pub metadata: crate::models::Metadata,
    
    /// Core task specification containing execution parameters
    /// Defines what to run and the required resources
    pub spec: TaskSpec,
    
    /// Runtime status of the task, populated during execution
    /// Contains state, timestamps, and execution results
    pub status: Option<TaskStatus>,
}

// Conversion from protobuf Task message
impl From<gevulot::Task> for Task {
    fn from(proto: gevulot::Task) -> Self {
        // Extract workflow reference if present in spec
        let workflow_ref = match proto.spec.as_ref() {
            Some(spec) if !spec.workflow_ref.is_empty() => Some(spec.workflow_ref.clone()),
            _ => None,
        };

        Task {
            kind: "Task".to_string(),
            version: "v0".to_string(),
            metadata: crate::models::Metadata {
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
                    .map(|l| crate::models::Label {
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

/// Task specification containing all execution parameters for a computational job.
///
/// The `TaskSpec` defines exactly what should be executed, including container image,
/// command, arguments, environment configuration, data context mounts, and resource
/// requirements.
///
/// # Fields
///
/// * `image` - Container image to run (e.g., Docker image)
/// * `command` - Optional command to override the image's default entrypoint
/// * `args` - Optional arguments to pass to the command
/// * `env` - Environment variables to set in the container
/// * `input_contexts` - Data sources to mount into the container
/// * `output_contexts` - Output data to capture from the container
/// * `resources` - CPU, GPU, memory, and time requirements
/// * `store_stdout` - Whether to capture and store standard output
/// * `store_stderr` - Whether to capture and store standard error
///
/// # Examples
///
/// ## Basic specification with just image and resources
///
/// ```
/// use gevulot_rs::models::TaskSpec;
///
/// let spec = serde_json::from_str::<TaskSpec>(r#"{
///     "image": "ubuntu:latest",
///     "resources": {
///         "cpus": "1cpu",
///         "gpus": "0gpu",
///         "memory": "512mb",
///         "time": "1h"
///     }
/// }"#).unwrap();
/// ```
///
/// ## Advanced specification with environment variables and input/output contexts
///
/// ```
/// use gevulot_rs::models::TaskSpec;
///
/// let spec = serde_json::from_str::<TaskSpec>(r#"{
///     "image": "prover:latest",
///     "command": ["/bin/prover"],
///     "args": ["--circuit", "/input/circuit.json", "--output", "/output/proof.json"],
///     "env": [
///         {"name": "RUST_LOG", "value": "debug"},
///         {"name": "NUM_THREADS", "value": "4"}
///     ],
///     "inputContexts": [
///         {"source": "circuit-data", "target": "/input"}
///     ],
///     "outputContexts": [
///         {"source": "/output", "retentionPeriod": 604800}
///     ],
///     "storeStdout": true,
///     "storeStderr": true,
///     "resources": {
///         "cpus": "4cpu",
///         "gpus": "1gpu", 
///         "memory": "16gb",
///         "time": "12h"
///     }
/// }"#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpec {
    /// Container image to run (Docker-compatible format)
    /// This defines the execution environment for the task
    pub image: String,
    
    /// Optional command to override image entrypoint
    /// If not specified, the default entrypoint of the image is used
    #[serde(default)]
    pub command: Vec<String>,
    
    /// Optional arguments for the command
    /// These are passed to either the command or the image entrypoint
    #[serde(default)]
    pub args: Vec<String>,
    
    /// Environment variables to set in container
    /// Used to configure the application's behavior
    #[serde(default)]
    pub env: Vec<TaskEnv>,
    
    /// Input data contexts to mount into the container
    /// These provide data access to the computation
    #[serde(rename = "inputContexts", default)]
    pub input_contexts: Vec<InputContext>,
    
    /// Output data contexts to capture from the container
    /// These specify what results to save from the computation
    #[serde(rename = "outputContexts", default)]
    pub output_contexts: Vec<OutputContext>,
    
    /// Resource requirements for scheduling and execution
    /// Specifies CPU, GPU, memory and time limits
    pub resources: TaskResources,
    
    /// Whether to store standard output stream
    /// When true, the stdout is captured and saved in the task status
    #[serde(rename = "storeStdout", default)]
    pub store_stdout: bool,
    
    /// Whether to store standard error stream
    /// When true, the stderr is captured and saved in the task status
    #[serde(rename = "storeStderr", default)]
    pub store_stderr: bool,
}

// Conversion from protobuf TaskSpec message
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
                cpus: proto.cpus.into(),
                gpus: proto.gpus.into(),
                memory: proto.memory.into(),
                time: proto.time.into(),
            },
            store_stdout: proto.store_stdout,
            store_stderr: proto.store_stderr,
        }
    }
}

/// Environment variable definition for task containers.
///
/// This struct represents a name-value pair for an environment variable
/// that will be set in the container environment when the task executes.
///
/// # Fields
///
/// * `name` - The environment variable name
/// * `value` - The environment variable value
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::TaskEnv;
///
/// let env_var = TaskEnv {
///     name: "DEBUG".to_string(), 
///     value: "true".to_string()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEnv {
    /// The environment variable name
    pub name: String,
    /// The environment variable value
    pub value: String,
}

/// Input context definition for mounting data into tasks.
///
/// Input contexts define how external data should be mounted into the
/// task container's filesystem. The `source` identifies data that has been
/// previously registered in Gevulot (typically output from another task
/// or pinned data), and `target` specifies where it should be mounted in
/// the container.
///
/// # Fields
///
/// * `source` - The data identifier (CID, pin ID, or output context ID)
/// * `target` - The mount path in the container filesystem
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::InputContext;
///
/// let input = InputContext {
///     source: "QmZ9nBpW6YLmKyhAXnGU9K1xy7be9MJG3fXz9ZwFTL11jc".to_string(),
///     target: "/data".to_string()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct InputContext {
    /// Source data identifier
    /// This can be a content ID (CID), pin ID, or output context ID from another task
    pub source: String,
    
    /// Target mount path in container
    /// This is where the data will be accessible within the container filesystem
    pub target: String,
}

/// Output context definition for capturing data from tasks.
///
/// Output contexts define how data should be captured from the task
/// container's filesystem when the task completes. The captured data
/// becomes available for future tasks to use as input contexts.
///
/// # Fields
///
/// * `source` - The path in the container filesystem to capture
/// * `retention_period` - How long to retain the data (in seconds)
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::OutputContext;
///
/// // Capture /results directory and keep it for 7 days
/// let output = OutputContext {
///     source: "/results".to_string(),
///     retention_period: 7 * 24 * 60 * 60 // 7 days in seconds
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputContext {
    /// Source path in container to capture
    /// This is the directory or file in the container filesystem that will be saved
    pub source: String,
    
    /// How long to retain the output data (in seconds)
    /// After this period expires, the data may be garbage collected
    #[serde(rename = "retentionPeriod")]
    pub retention_period: i64,
}

/// Resource requirements for task execution.
///
/// This struct defines the computational resources required for a task,
/// including CPU cores, GPU units, memory, and execution time limit.
/// Each resource can be specified with human-readable units.
///
/// # Fields
///
/// * `cpus` - CPU cores required (e.g., "2cpu", "500mcpu")
/// * `gpus` - GPU units required (e.g., "1gpu", "500mgpu")
/// * `memory` - Memory required (e.g., "4gb", "512mb")
/// * `time` - Maximum execution time (e.g., "1h", "30m")
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{TaskResources, CoreUnit, ByteUnit, TimeUnit};
///
/// let resources = TaskResources {
///     cpus: "2cpu".parse::<CoreUnit>().unwrap(),
///     gpus: "1gpu".parse::<CoreUnit>().unwrap(),
///     memory: "4gb".parse::<ByteUnit>().unwrap(),
///     time: "2h".parse::<TimeUnit>().unwrap()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResources {
    /// CPU cores required (supports units like "2cpu", "500mcpu")
    /// 
    /// This field uses the CoreUnit type which parses and normalizes CPU allocations.
    /// For example, "1cpu" is equivalent to "1000mcpu" (milli-CPU).
    pub cpus: crate::models::CoreUnit,
    
    /// GPU cores required (supports units like "1gpu", "500mgpu")
    /// 
    /// This field uses the CoreUnit type which parses and normalizes GPU allocations.
    /// For example, "1gpu" is equivalent to "1000mgpu" (milli-GPU).
    pub gpus: crate::models::CoreUnit,
    
    /// Memory required (supports units like "1gb", "512mb")
    /// 
    /// This field uses the ByteUnit type which parses and normalizes memory allocations.
    /// Supports various units including KB, MB, GB, etc.
    pub memory: crate::models::ByteUnit,
    
    /// Time limit (supports units like "1h", "30m", "90s")
    /// 
    /// This field uses the TimeUnit type which parses and normalizes time durations.
    /// The task will be terminated if it exceeds this time limit.
    pub time: crate::models::TimeUnit,
}

impl Default for TaskResources {
    fn default() -> Self {
        Self {
            cpus: "1cpu".parse().unwrap(),
            gpus: "0gpu".parse().unwrap(),
            memory: "512mb".parse().unwrap(),
            time: "1h".parse().unwrap(),
        }
    }
}

/// Runtime status of a task.
///
/// This struct contains information about the current state of a task
/// during and after execution, including state, timestamps, assigned workers,
/// and execution results.
///
/// # Fields
///
/// * `state` - Current execution state (e.g., "Pending", "Running", "Completed")
/// * `created_at` - Unix timestamp when the task was created
/// * `started_at` - Unix timestamp when the task started execution
/// * `completed_at` - Unix timestamp when the task completed execution
/// * `assigned_workers` - List of worker IDs that have been assigned to this task
/// * `active_worker` - ID of the worker currently executing the task
/// * `exit_code` - Exit code from the container (if completed)
/// * `output_contexts` - IDs of the output contexts produced by the task
/// * `stdout` - Captured standard output (if store_stdout was enabled)
/// * `stderr` - Captured standard error (if store_stderr was enabled)
/// * `error` - Error message if the task failed
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::TaskStatus;
///
/// let status = TaskStatus {
///     state: "Completed".to_string(),
///     created_at: 1634567890,
///     started_at: 1634567900,
///     completed_at: 1634568000,
///     assigned_workers: vec!["worker-123".to_string()],
///     active_worker: "worker-123".to_string(),
///     exit_code: Some(0),
///     output_contexts: vec!["output-456".to_string()],
///     stdout: Some("Task completed successfully".to_string()),
///     stderr: None,
///     error: None
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatus {
    /// Current state of the task
    /// 
    /// Common values include:
    /// - "Pending": Waiting for worker assignment
    /// - "Running": Currently executing on a worker
    /// - "Completed": Finished execution successfully
    /// - "Failed": Execution failed
    pub state: String,
    
    /// Unix timestamp when the task was created
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    
    /// Unix timestamp when the task started execution
    /// This is set when a worker begins executing the task
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    
    /// Unix timestamp when the task finished execution
    /// This is set when a worker completes or fails the task
    #[serde(rename = "completedAt")]
    pub completed_at: i64,
    
    /// List of worker IDs that have been assigned to this task
    /// A task may be reassigned to different workers if there are failures
    #[serde(rename = "assignedWorkers")]
    pub assigned_workers: Vec<String>,
    
    /// ID of the worker currently executing the task
    /// This is the worker that is actively running the container
    #[serde(rename = "activeWorker")]
    pub active_worker: String,
    
    /// Exit code from the container if task completed
    /// A value of 0 typically indicates successful execution
    #[serde(rename = "exitCode")]
    pub exit_code: Option<i64>,
    
    /// Output context identifiers produced by this task
    /// These can be used as input contexts for other tasks
    #[serde(rename = "outputContexts")]
    pub output_contexts: Vec<String>,
    
    /// Captured standard output if enabled
    /// This is only populated if store_stdout was set to true
    pub stdout: Option<String>,
    
    /// Captured standard error if enabled
    /// This is only populated if store_stderr was set to true
    pub stderr: Option<String>,
    
    /// Error message if the task failed
    /// This provides additional context about the failure reason
    pub error: Option<String>,
}

// Conversion from protobuf TaskStatus message
impl From<gevulot::TaskStatus> for TaskStatus {
    fn from(proto: gevulot::TaskStatus) -> Self {
        // Map numeric state to string representation
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

        // Convert empty strings to None
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_task_with_units() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "command": ["echo", "hello"],
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "0gpu",
                    "memory": "512mb",
                    "time": "1h"
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        assert_eq!(task.spec.command, vec!["echo", "hello"]);
        // Using Debug format instead of to_string to avoid Display trait requirements
        assert_eq!(format!("{:?}", task.spec.resources.cpus), "String(\"1cpu\")");
        assert_eq!(format!("{:?}", task.spec.resources.gpus), "String(\"0gpu\")");
        assert_eq!(format!("{:?}", task.spec.resources.memory), "String(\"512mb\")");
        assert_eq!(format!("{:?}", task.spec.resources.time), "String(\"1h\")");
    }

    #[test]
    fn test_parse_task_without_units() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "command": ["echo", "hello"],
                "resources": {
                    "cpus": 1,
                    "gpus": 0,
                    "memory": 512,
                    "time": 3600
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        assert_eq!(task.spec.command, vec!["echo", "hello"]);
        // Using Debug format instead of to_string to avoid Display trait requirements
        assert_eq!(format!("{:?}", task.spec.resources.cpus), "Number(1)");
        assert_eq!(format!("{:?}", task.spec.resources.gpus), "Number(0)");
        assert_eq!(format!("{:?}", task.spec.resources.memory), "Number(512)");
        assert_eq!(format!("{:?}", task.spec.resources.time), "Number(3600)");
    }

    #[test]
    fn test_parse_resources_without_units_as_strings() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "resources": {
                    "cpus": "1",
                    "gpus": "0",
                    "memory": "512",
                    "time": "3600"
                }
            }
        }))
        .unwrap();

        // Using Debug format instead of to_string to avoid Display trait requirements
        assert_eq!(format!("{:?}", task.spec.resources.cpus), "String(\"1\")");
        assert_eq!(format!("{:?}", task.spec.resources.gpus), "String(\"0\")");
        assert_eq!(format!("{:?}", task.spec.resources.memory), "String(\"512\")");
        assert_eq!(format!("{:?}", task.spec.resources.time), "String(\"3600\")");
    }

    #[test]
    fn test_parse_task_without_much() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "0gpu",
                    "memory": "512mb",
                    "time": "1h"
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        assert!(task.spec.command.is_empty());
        assert!(task.spec.args.is_empty());
    }

    #[test]
    fn test_parse_task_yaml() {
        let task_yaml = r#"
            kind: Task
            version: v0
            spec:
                image: ubuntu:latest
                resources:
                    cpus: 1cpu
                    gpus: 0gpu
                    memory: 512mb
                    time: 1h
        "#;

        let task = serde_yaml::from_str::<Task>(task_yaml).unwrap();
        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        // Using Debug format instead of to_string to avoid Display trait requirements
        assert_eq!(format!("{:?}", task.spec.resources.cpus), "String(\"1cpu\")");
        assert_eq!(format!("{:?}", task.spec.resources.gpus), "String(\"0gpu\")");
        assert_eq!(format!("{:?}", task.spec.resources.memory), "String(\"512mb\")");
        assert_eq!(format!("{:?}", task.spec.resources.time), "String(\"1h\")");
    }

    #[test]
    fn test_parse_task_with_env() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "env": [
                    {"name": "FOO", "value": "bar"},
                    {"name": "BAZ", "value": "qux"}
                ],
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "0gpu",
                    "memory": "512mb",
                    "time": "1h"
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        assert_eq!(task.spec.env.len(), 2);
        assert_eq!(task.spec.env[0].name, "FOO");
        assert_eq!(task.spec.env[0].value, "bar");
        assert_eq!(task.spec.env[1].name, "BAZ");
        assert_eq!(task.spec.env[1].value, "qux");
    }

    #[test]
    fn test_parse_task_with_input_context() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "inputContexts": [
                    {"source": "foo", "target": "/foo"},
                    {"source": "bar", "target": "/bar"}
                ],
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "0gpu",
                    "memory": "512mb",
                    "time": "1h"
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        assert_eq!(task.spec.input_contexts.len(), 2);
        assert_eq!(task.spec.input_contexts[0].source, "foo");
        assert_eq!(task.spec.input_contexts[0].target, "/foo");
        assert_eq!(task.spec.input_contexts[1].source, "bar");
        assert_eq!(task.spec.input_contexts[1].target, "/bar");
    }

    #[test]
    fn test_parse_task_with_output_context() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "ubuntu:latest",
                "outputContexts": [
                    {"source": "/foo", "retentionPeriod": 3600},
                    {"source": "/bar", "retentionPeriod": 7200}
                ],
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "0gpu",
                    "memory": "512mb",
                    "time": "1h"
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.spec.image, "ubuntu:latest");
        assert_eq!(task.spec.output_contexts.len(), 2);
        assert_eq!(task.spec.output_contexts[0].source, "/foo");
        assert_eq!(task.spec.output_contexts[0].retention_period, 3600);
        assert_eq!(task.spec.output_contexts[1].source, "/bar");
        assert_eq!(task.spec.output_contexts[1].retention_period, 7200);
    }

    #[test]
    fn test_parse_task_with_metadata() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "metadata": {
                "name": "My Task",
                "description": "A test task",
                "tags": ["test", "example"],
                "labels": [
                    {"key": "foo", "value": "bar"},
                    {"key": "baz", "value": "qux"}
                ]
            },
            "spec": {
                "image": "ubuntu:latest",
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "0gpu",
                    "memory": "512mb",
                    "time": "1h"
                }
            }
        }))
        .unwrap();

        assert_eq!(task.kind, "Task");
        assert_eq!(task.version, "v0");
        assert_eq!(task.metadata.name, "My Task");
        assert_eq!(task.metadata.description, "A test task");
        assert_eq!(task.metadata.tags, vec!["test", "example"]);
        assert_eq!(task.metadata.labels.len(), 2);
        assert_eq!(task.metadata.labels[0].key, "foo");
        assert_eq!(task.metadata.labels[0].value, "bar");
        assert_eq!(task.metadata.labels[1].key, "baz");
        assert_eq!(task.metadata.labels[1].value, "qux");
    }
}
