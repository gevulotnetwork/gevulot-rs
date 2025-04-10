//! Workflow model and related types for managing workflow execution in Gevulot.
//!
//! This module provides the core workflow model used throughout the Gevulot system, including:
//! - Workflow specifications with sequential stages and parallel tasks
//! - Status tracking for multi-stage workflow execution
//! - Stage transition and completion monitoring
//! - Metadata management for workflow identification and filtering
//!
//! Workflows are a higher-level abstraction in Gevulot that allow coordinating multiple
//! tasks in a structured, dependency-aware manner. They enable complex computational
//! pipelines where outputs from one stage can be used as inputs to the next.
//!
//! # Key Components
//!
//! - [`Workflow`] - Complete workflow definition including metadata, specification, and status
//! - [`WorkflowSpec`] - Defines the stages and tasks that make up the workflow
//! - [`WorkflowStage`] - Represents a collection of tasks that can run in parallel
//! - [`WorkflowStatus`] - Tracks execution progress across stages and tasks
//! - [`WorkflowStageStatus`] - Monitors completion of tasks within a specific stage
//!
//! # Workflow Lifecycle
//!
//! A typical workflow follows this lifecycle:
//! 1. **Created** - Workflow is submitted to the blockchain with all stages defined
//! 2. **Running** - Tasks from the first stage are created and executed
//! 3. **Transitioning** - As each stage completes, the next stage's tasks are created
//! 4. **Completed** - All stages have finished execution (success or failure)

use super::{Label, Metadata, TaskSpec};
use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Represents a complete workflow definition with metadata, specification and status
///
/// A workflow consists of one or more stages that are executed sequentially. Each stage
/// contains one or more tasks that can be executed in parallel. Workflows provide a
/// structured way to coordinate complex computational pipelines.
///
/// # Fields
///
/// * `kind` - Type identifier, always "Workflow" for workflow entities
/// * `version` - Schema version, typically "v0"
/// * `metadata` - Workflow identification, description, tags, and identifying information
/// * `spec` - Stages and tasks that define the workflow's structure
/// * `status` - Optional runtime status tracking execution progress
///
/// # Examples
///
/// Creating a basic workflow:
/// ```
/// use gevulot_rs::models::Workflow;
///
/// let workflow = serde_json::from_str::<Workflow>(r#"{
///     "kind": "Workflow",
///     "version": "v0",
///     "metadata": {
///         "name": "my-workflow",
///         "tags": ["compute"],
///         "description": "Simple data analysis pipeline",
///         "labels": [
///             {
///                 "key": "my-key",
///                 "value": "my-label"
///             }
///         ]
///     },
///     "spec": {
///         "stages": [{
///             "tasks": [{
///                 "image": "alpine",
///                 "resources": {
///                     "cpus": "1cpu",
///                     "memory": "1GiB",
///                     "gpus": 0,
///                     "time": "120s"
///                 }
///             }]
///         }]
///     }
/// }"#).unwrap();
/// ```
///
/// Multi-stage workflow with data dependencies:
/// ```
/// use gevulot_rs::models::Workflow;
///
/// let workflow = serde_json::from_str::<Workflow>(r#"{
///     "kind": "Workflow",
///     "version": "v0",
///     "metadata": {
///         "name": "data-processing-pipeline",
///         "description": "Multi-stage data processing workflow",
///         "tags": ["data", "processing", "pipeline"],
///         "labels": [
///             {"key": "priority", "value": "high"},
///             {"key": "department", "value": "research"}
///         ]
///     },
///     "spec": {
///         "stages": [
///             {
///                 "tasks": [{
///                     "image": "data-collector:v1",
///                     "command": ["collect.sh"],
///                     "outputContexts": [{
///                         "source": "/data",
///                         "retentionPeriod": 3600
///                     }],
///                     "resources": {
///                         "cpus": "2cpu",
///                         "gpus": 0,
///                         "memory": "4GiB",
///                         "time": "1h"
///                     }
///                 }]
///             },
///             {
///                 "tasks": [{
///                     "image": "data-processor:v2",
///                     "command": ["process.py"],
///                     "inputContexts": [{
///                         "source": "stage-0-task-0-output-0",
///                         "target": "/input"
///                     }],
///                     "outputContexts": [{
///                         "source": "/results",
///                         "retentionPeriod": 86400
///                     }],
///                     "resources": {
///                         "cpus": "4cpu",
///                         "gpus": "1gpu",
///                         "memory": "8GiB",
///                         "time": "2h"
///                     }
///                 }]
///             }
///         ]
///     }
/// }"#).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Workflow {
    /// Type identifier, always "Workflow" for this struct
    /// Used for type identification in serialized form
    pub kind: String,
    
    /// API version for the workflow format, currently "v0"
    /// This allows for future schema evolution
    pub version: String,
    
    /// Workflow metadata like name, description, tags, and identifying information
    /// Used for filtering, searching, and referencing workflows
    #[serde(default)]
    pub metadata: Metadata,
    
    /// Core workflow specification containing stages and tasks
    /// Defines the structure and execution order of the workflow
    pub spec: WorkflowSpec,
    
    /// Runtime status of the workflow, populated during execution
    /// Contains state, current stage, and completion status
    pub status: Option<WorkflowStatus>,
}

/// Converts a protobuf Workflow message to the internal Workflow model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
impl From<gevulot::Workflow> for Workflow {
    fn from(proto: gevulot::Workflow) -> Self {
        // Create a new workflow, carefully mapping all protobuf fields to our model
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

/// Represents a single stage in a workflow containing one or more tasks
///
/// Tasks within a stage can be executed in parallel. The workflow will only
/// proceed to the next stage once all tasks in the current stage are complete.
///
/// # Fields
///
/// * `tasks` - Vector of task specifications to execute in this stage
///
/// # Execution Model
///
/// - All tasks in a stage are eligible for concurrent execution
/// - The stage is considered complete only when all tasks have finished
/// - If any task fails, the entire workflow may be marked as failed
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{WorkflowStage, TaskSpec, TaskResources};
///
/// let stage = WorkflowStage {
///     tasks: vec![
///         TaskSpec {
///             image: "processor:v1".to_string(),
///             command: vec!["python".to_string(), "process.py".to_string()],
///             args: vec![],
///             env: vec![],
///             input_contexts: vec![],
///             output_contexts: vec![],
///             resources: TaskResources::default(),
///             store_stdout: false,
///             store_stderr: false,
///         }
///     ]
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStage {
    /// List of task specifications to execute in this stage
    /// All tasks in a stage are eligible for concurrent execution
    pub tasks: Vec<TaskSpec>,
}

/// Specification for a workflow defining its stages and tasks
///
/// The stages are executed sequentially, with tasks in each stage potentially
/// running in parallel depending on available resources.
///
/// # Fields
///
/// * `stages` - Vector of workflow stages to execute in sequence
///
/// # Stage Dependencies
///
/// Stages are executed in order from first to last. The workflow system
/// automatically creates tasks for each stage when the previous stage completes.
/// Tasks can reference outputs from previous stages as inputs.
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{WorkflowSpec, WorkflowStage, TaskSpec, TaskResources};
///
/// let spec = WorkflowSpec {
///     stages: vec![
///         WorkflowStage {
///             tasks: vec![
///                 TaskSpec {
///                     image: "data-collector:v1".to_string(),
///                     command: vec![],
///                     args: vec![],
///                     env: vec![],
///                     input_contexts: vec![],
///                     output_contexts: vec![],
///                     resources: TaskResources::default(),
///                     store_stdout: false,
///                     store_stderr: false,
///                 }
///             ]
///         },
///         WorkflowStage {
///             tasks: vec![
///                 TaskSpec {
///                     image: "data-processor:v2".to_string(),
///                     command: vec![],
///                     args: vec![],
///                     env: vec![],
///                     input_contexts: vec![],
///                     output_contexts: vec![],
///                     resources: TaskResources::default(),
///                     store_stdout: false,
///                     store_stderr: false,
///                 }
///             ]
///         }
///     ]
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowSpec {
    /// Vector of workflow stages to execute in sequence
    /// Each stage contains tasks that can run in parallel
    pub stages: Vec<WorkflowStage>,
}

/// Converts a protobuf WorkflowSpec message to the internal WorkflowSpec model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
impl From<gevulot::WorkflowSpec> for WorkflowSpec {
    fn from(proto: gevulot::WorkflowSpec) -> Self {
        // Map each protobuf stage to our stage model, converting tasks as well
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

/// Status information for a single stage in a workflow
///
/// Tracks which tasks have been created and how many have completed.
///
/// # Fields
///
/// * `task_ids` - IDs of tasks created for this stage
/// * `finished_tasks` - Count of tasks that have completed (success or failure)
///
/// # Completion Criteria
///
/// A stage is considered complete when the number of finished tasks equals the 
/// total number of tasks in the stage. The workflow can then advance to the next stage.
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::WorkflowStageStatus;
///
/// let stage_status = WorkflowStageStatus {
///     task_ids: vec!["task-1".to_string(), "task-2".to_string(), "task-3".to_string()],
///     finished_tasks: 2,
/// };
///
/// // Two out of three tasks are finished
/// let progress = stage_status.finished_tasks as f64 / stage_status.task_ids.len() as f64;
/// assert_eq!(progress, 2.0 / 3.0);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStageStatus {
    /// IDs of tasks created for this stage
    /// These can be used to look up individual task status
    #[serde(rename = "taskIds")]
    pub task_ids: Vec<String>,
    
    /// Count of tasks that have completed execution
    /// Used to determine when to advance to the next stage
    #[serde(rename = "finishedTasks")]
    pub finished_tasks: u64,
}

/// Current status of a workflow's execution
///
/// Tracks the overall workflow state, current execution stage, and
/// status of each stage including task completion.
///
/// # Fields
///
/// * `state` - Current workflow state (Pending, Running, Done, Failed)
/// * `current_stage` - Index of the stage currently being executed
/// * `stages` - Vector of stage statuses with task information
///
/// # Workflow States
///
/// - **Pending**: Workflow is created but execution hasn't started
/// - **Running**: One or more stages are currently executing
/// - **Done**: All stages have completed successfully
/// - **Failed**: A task failure has occurred that prevented completion
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{WorkflowStatus, WorkflowStageStatus};
///
/// let status = WorkflowStatus {
///     state: "Running".to_string(),
///     current_stage: 1,
///     stages: vec![
///         WorkflowStageStatus {
///             task_ids: vec!["task-1".to_string(), "task-2".to_string()],
///             finished_tasks: 2,
///         },
///         WorkflowStageStatus {
///             task_ids: vec!["task-3".to_string(), "task-4".to_string()],
///             finished_tasks: 1,
///         },
///     ],
/// };
///
/// // First stage is complete, second stage is 50% complete
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStatus {
    /// Current workflow state
    /// Common values: "Pending", "Running", "Done", "Failed"
    pub state: String,
    
    /// Index of the stage currently being executed
    /// Zero-based index into the stages array
    #[serde(rename = "currentStage")]
    pub current_stage: u64,
    
    /// Status information for each stage in the workflow
    /// Tracks task creation and completion
    pub stages: Vec<WorkflowStageStatus>,
}

/// Converts a protobuf WorkflowStatus message to the internal WorkflowStatus model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
impl From<gevulot::WorkflowStatus> for WorkflowStatus {
    fn from(proto: gevulot::WorkflowStatus) -> Self {
        WorkflowStatus {
            // Map numeric states to human readable strings
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

// Unit tests to verify workflow serialization/deserialization and field mapping
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_workflow() {
        // Test parsing a complete workflow JSON with all fields populated
        let workflow = serde_json::from_value::<Workflow>(json!({
            "kind": "Workflow",
            "version": "v0",
            "metadata": {
                "id": "test-id",
                "name": "Test Workflow",
                "creator": "test-creator",
                "description": "Test Workflow Description",
                "tags": ["tag1", "tag2"],
                "labels": [
                    {"key": "label1", "value": "value1"},
                    {"key": "label2", "value": "value2"}
                ]
            },
            "spec": {
                "stages": [
                    {
                        "tasks": [
                            {
                                "image": "test-image-1",
                                "resources": {
                                    "cpus": "1cpu",
                                    "gpus": "1gpu",
                                    "memory": "1GiB",
                                    "time": "1hr"
                                }
                            },
                            {
                                "image": "test-image-2",
                                "resources": {
                                    "cpus": "2cpu",
                                    "gpus": "2gpu",
                                    "memory": "2GiB",
                                    "time": "2hr"
                                }
                            }
                        ]
                    }
                ]
            },
            "status": {
                "state": "Running",
                "currentStage": 0,
                "stages": [
                    {
                        "taskIds": ["task-1", "task-2"],
                        "finishedTasks": 1
                    }
                ]
            }
        }))
        .unwrap();

        // Verify metadata
        assert_eq!(workflow.metadata.id, Some("test-id".to_string()));
        assert_eq!(workflow.metadata.name, "Test Workflow");
        assert_eq!(workflow.metadata.creator, Some("test-creator".to_string()));
        assert_eq!(workflow.metadata.description, "Test Workflow Description");
        assert_eq!(workflow.metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(workflow.metadata.labels.len(), 2);
        assert_eq!(workflow.metadata.labels[0].key, "label1");
        assert_eq!(workflow.metadata.labels[0].value, "value1");

        // Verify spec
        assert_eq!(workflow.spec.stages.len(), 1);
        assert_eq!(workflow.spec.stages[0].tasks.len(), 2);
        assert_eq!(workflow.spec.stages[0].tasks[0].image, "test-image-1");
        assert_eq!(workflow.spec.stages[0].tasks[1].image, "test-image-2");

        // Verify status
        let status = workflow.status.unwrap();
        assert_eq!(status.state, "Running");
        assert_eq!(status.current_stage, 0);
        assert_eq!(status.stages.len(), 1);
        assert_eq!(status.stages[0].task_ids, vec!["task-1", "task-2"]);
        assert_eq!(status.stages[0].finished_tasks, 1);
    }

    #[test]
    fn test_parse_workflow_with_minimum() {
        // Test parsing a minimal workflow JSON with only required fields
        let workflow = serde_json::from_value::<Workflow>(json!({
            "kind": "Workflow",
            "version": "v0",
            "spec": {
                "stages": []
            }
        }))
        .unwrap();

        assert_eq!(workflow.kind, "Workflow");
        assert_eq!(workflow.version, "v0");
        assert_eq!(workflow.spec.stages.len(), 0);
        assert!(workflow.status.is_none());
    }
}
