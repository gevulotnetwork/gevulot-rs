//! Task model and related types for managing task execution.
//!
//! This module provides the core task model used throughout the system, including:
//! - Task specification and status
//! - Resource requirements (CPU, GPU, memory, time)
//! - Input/output context handling
//! - Environment variables
//! - Metadata like tags and labels

use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Represents a complete task definition with metadata, specification and status
///
/// # Examples
///
/// Creating a basic task:
/// ```
/// use crate::models::Task;
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
/// Task with input/output contexts:
/// ```
/// let task = serde_json::from_str::<Task>(r#"{
///     "kind": "Task",
///     "version": "v0",
///     "spec": {
///         "image": "processor:v1",
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    // The kind is always "Task" - used for type identification in serialized form
    pub kind: String,
    // API version, currently "v0"
    pub version: String,
    // Task metadata like name, description, tags etc
    #[serde(default)]
    pub metadata: crate::models::Metadata,
    // Core task specification
    pub spec: TaskSpec,
    // Optional runtime status
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

/// Task specification containing all execution parameters
///
/// # Examples
///
/// Basic spec with just image and resources:
/// ```
/// use crate::models::TaskSpec;
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
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSpec {
    // Container image to run
    pub image: String,
    // Optional command to override image entrypoint
    #[serde(default)]
    pub command: Vec<String>,
    // Optional arguments for the command
    #[serde(default)]
    pub args: Vec<String>,
    // Environment variables to set in container
    #[serde(default)]
    pub env: Vec<TaskEnv>,
    // Input data contexts to mount
    #[serde(rename = "inputContexts", default)]
    pub input_contexts: Vec<InputContext>,
    // Output data contexts to capture
    #[serde(rename = "outputContexts", default)]
    pub output_contexts: Vec<OutputContext>,
    // Resource requirements
    pub resources: TaskResources,
    // Whether to store stdout stream
    #[serde(rename = "storeStdout", default)]
    pub store_stdout: bool,
    // Whether to store stderr stream
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

/// Environment variable definition for task container
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskEnv {
    pub name: String,
    pub value: String,
}

/// Input context for mounting data into task container
#[derive(Serialize, Deserialize, Debug)]
pub struct InputContext {
    // Source data identifier
    pub source: String,
    // Target mount path in container
    pub target: String,
}

/// Output context for capturing data from task container
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputContext {
    // Source path in container to capture
    pub source: String,
    // How long to retain the output data
    #[serde(rename = "retentionPeriod")]
    pub retention_period: i64,
}

/// Resource requirements for task execution
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResources {
    // CPU cores required (supports units like "2cpu", "500mcpu")
    pub cpus: crate::models::CoreUnit,
    // GPU cores required (supports units like "1gpu", "500mgpu")
    pub gpus: crate::models::CoreUnit,
    // Memory required (supports units like "1gb", "512mb")
    pub memory: crate::models::ByteUnit,
    // Time limit (supports units like "1h", "30m")
    pub time: crate::models::TimeUnit,
}

/// Runtime status of a task
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatus {
    // Current state (Pending, Running, Done, Failed etc)
    pub state: String,
    // Timestamps for task lifecycle
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    #[serde(rename = "completedAt")]
    pub completed_at: i64,
    // Workers assigned/active for this task
    #[serde(rename = "assignedWorkers")]
    pub assigned_workers: Vec<String>,
    #[serde(rename = "activeWorker")]
    pub active_worker: String,
    // Exit code if task completed
    #[serde(rename = "exitCode")]
    pub exit_code: Option<i64>,
    // Output context identifiers
    #[serde(rename = "outputContexts")]
    pub output_contexts: Vec<String>,
    // Captured output streams if enabled
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    // Error message if failed
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
                "image": "test",
                "resources": {
                    "cpus": "1cpu",
                    "gpus": "1gpu",
                    "memory": "1024mb",
                    "time": "1hr"
                }
            }
        }))
        .unwrap();

        assert_eq!(
            task.spec.resources.cpus,
            crate::models::CoreUnit::String("1cpu".to_string())
        );
        assert_eq!(
            task.spec.resources.gpus,
            crate::models::CoreUnit::String("1gpu".to_string())
        );
        assert_eq!(
            task.spec.resources.memory,
            crate::models::ByteUnit::String("1024mb".to_string())
        );
        assert_eq!(
            task.spec.resources.time,
            crate::models::TimeUnit::String("1hr".to_string())
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
                    "cpus": 1,
                    "gpus": 1,
                    "memory": 1024,
                    "time": 1
                }
            }
        }))
        .unwrap();

        assert_eq!(task.spec.resources.cpus.millicores(), Ok(1000));
        assert_eq!(task.spec.resources.gpus.millicores(), Ok(1000));
        assert_eq!(task.spec.resources.memory.bytes(), Ok(1024));
        assert_eq!(task.spec.resources.time.seconds(), Ok(1));
    }

    #[test]
    fn test_parse_resources_without_units_as_strings() {
        let resources = serde_json::from_value::<TaskResources>(json!({
            "cpus": "1",
            "gpus": "1",
            "memory": "1024",
            "time": 1
        }))
        .unwrap();
        // To parse time we use humantime, which does require an explicit suffix,
        // so specifying time as string doesn't work
        // However for other resource types string with bare number is equal to that number.

        assert_eq!(resources.cpus.millicores(), Ok(1000));
        assert_eq!(resources.gpus.millicores(), Ok(1000));
        assert_eq!(resources.memory.bytes(), Ok(1024));
        assert_eq!(resources.time.seconds(), Ok(1));
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

        assert_eq!(task.spec.resources.cpus.millicores(), Ok(1000));
        assert_eq!(task.spec.resources.gpus.millicores(), Ok(1000));
        assert_eq!(task.spec.resources.memory.bytes(), Ok(1024 * 1024 * 1024));
        assert_eq!(task.spec.resources.time.seconds(), Ok(60 * 60));
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

        assert_eq!(task.spec.resources.cpus.millicores(), Ok(1000));
        assert_eq!(task.spec.resources.gpus.millicores(), Ok(1000));
        assert_eq!(task.spec.resources.memory.bytes(), Ok(1024 * 1024 * 1024));
        assert_eq!(task.spec.resources.time.seconds(), Ok(60 * 60));
    }

    #[test]
    fn test_parse_task_with_env() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "test",
                "env": [
                    {
                        "name": "FOO",
                        "value": "bar"
                    },
                    {
                        "name": "DEBUG",
                        "value": "1"
                    }
                ],
                "resources": {
                    "cpus": "1000 MCpu",
                    "gpus": "1000 MGpu",
                    "memory": "1024 MiB",
                    "time": "1 hr"
                }
            }
        }))
        .expect("Failed to parse task");

        assert_eq!(
            task.spec
                .env
                .iter()
                .find(|e| e.name == "FOO")
                .unwrap()
                .value,
            "bar"
        );
        assert_eq!(
            task.spec
                .env
                .iter()
                .find(|e| e.name == "DEBUG")
                .unwrap()
                .value,
            "1"
        );
    }

    #[test]
    fn test_parse_task_with_input_context() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "test",
                "inputContexts": [
                    {
                        "source": "pin1",
                        "target": "/input/data1"
                    },
                    {
                        "source": "pin2",
                        "target": "/input/data2"
                    }
                ],
                "resources": {
                    "cpus": "1000 MCpu",
                    "gpus": "1000 MGpu",
                    "memory": "1024 MiB",
                    "time": "1 hr"
                }
            }
        }))
        .expect("Failed to parse task");

        let input = &task.spec.input_contexts[0];
        assert_eq!(input.source, "pin1");
        assert_eq!(input.target, "/input/data1");
        let input = &task.spec.input_contexts[1];
        assert_eq!(input.source, "pin2");
        assert_eq!(input.target, "/input/data2");
    }

    #[test]
    fn test_parse_task_with_output_context() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "spec": {
                "image": "test",
                "outputContexts": [
                    {
                        "source": "/output/result1",
                        "retentionPeriod": 1000
                    },
                    {
                        "source": "/output/result2",
                        "retentionPeriod": 1000
                    }
                ],
                "resources": {
                    "cpus": "1000 MCpu",
                    "gpus": "1000 MGpu",
                    "memory": "1024 MiB",
                    "time": "1 hr"
                }
            }
        }))
        .expect("Failed to parse task");

        let output = &task.spec.output_contexts[0];
        assert_eq!(output.source, "/output/result1");
        assert_eq!(output.retention_period, 1000);
        let output = &task.spec.output_contexts[1];
        assert_eq!(output.source, "/output/result2");
        assert_eq!(output.retention_period, 1000);
    }

    #[test]
    fn test_parse_task_with_metadata() {
        let task = serde_json::from_value::<Task>(json!({
            "kind": "Task",
            "version": "v0",
            "metadata": {
                "name": "test-task",
                "description": "A test task",
                "tags": ["test", "example"],
                "labels": [
                    {"key": "env", "value": "test"},
                    {"key": "priority", "value": "high"}
                ]
            },
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

        assert_eq!(task.metadata.name, "test-task");
        assert_eq!(task.metadata.description, "A test task");
        assert_eq!(task.metadata.tags, vec!["test", "example"]);
        assert_eq!(task.metadata.labels[0].key, "env");
        assert_eq!(task.metadata.labels[0].value, "test");
        assert_eq!(task.metadata.labels[1].key, "priority");
        assert_eq!(task.metadata.labels[1].value, "high");
    }
}
