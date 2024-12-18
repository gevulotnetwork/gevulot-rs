use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};
use super::{Label, Metadata, TaskSpec};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workflow {
    pub kind: String,
    pub version: String,
    #[serde(default)]
    pub metadata: Metadata,
    pub spec: WorkflowSpec,
    pub status: Option<WorkflowStatus>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStage {
    pub tasks: Vec<TaskSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowSpec {
    pub stages: Vec<WorkflowStage>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStageStatus {
    #[serde(rename = "taskIds")]
    pub task_ids: Vec<String>,
    #[serde(rename = "finishedTasks")]
    pub finished_tasks: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStatus {
    pub state: String,
    #[serde(rename = "currentStage")]
    pub current_stage: u64,
    pub stages: Vec<WorkflowStageStatus>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_workflow() {
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
