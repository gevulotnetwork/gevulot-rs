
use serde::{Deserialize, Serialize};

mod serialization_helpers;
use serialization_helpers::*;

mod metadata;
pub use metadata::{Label, Metadata};

mod task;
pub use task::{Task, TaskSpec, TaskStatus, TaskEnv, InputContext, OutputContext, TaskResources};

mod worker;
pub use worker::{Worker, WorkerSpec, WorkerStatus};

mod pin;
pub use pin::{Pin, PinSpec, PinStatus, PinAck};

mod workflow;
pub use workflow::{Workflow, WorkflowSpec, WorkflowStatus, WorkflowStage, WorkflowStageStatus};

#[derive(Serialize, Deserialize, Debug)]
pub struct Generic {
    pub kind: String,
    pub version: String,
    pub metadata: Metadata,
    pub spec: serde_json::Value,
    pub status: Option<serde_json::Value>,
}
