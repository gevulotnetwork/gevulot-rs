use serde::{Deserialize, Serialize};

mod serialization_helpers;
use serialization_helpers::*;

mod metadata;
pub use metadata::{Label, Metadata};

mod task;
pub use task::{InputContext, OutputContext, Task, TaskEnv, TaskResources, TaskSpec, TaskStatus};

mod worker;
pub use worker::{Worker, WorkerSpec, WorkerStatus};

mod pin;
pub use pin::{Pin, PinAck, PinSpec, PinStatus};

mod workflow;
pub use workflow::{Workflow, WorkflowSpec, WorkflowStage, WorkflowStageStatus, WorkflowStatus};

#[derive(Serialize, Deserialize, Debug)]
pub struct Generic {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: serde_json::Value,
    pub status: Option<serde_json::Value>,
}
