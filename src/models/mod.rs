/*!
 * # Gevulot Data Models
 *
 * This module contains the core data models used throughout the Gevulot system.
 * These models represent the fundamental entities in the Gevulot network such as
 * tasks, workers, pins, and workflows.
 *
 * ## Model Categories
 *
 * - **Task Models**: Represent computational jobs that run on worker nodes
 * - **Worker Models**: Represent compute providers in the network
 * - **Pin Models**: Represent data pinned for availability in the network
 * - **Workflow Models**: Represent sequences of interdependent tasks
 * - **Serialization Helpers**: Types for resource units and parsing
 *
 * Each model typically follows a pattern with a main struct, a specification struct,
 * and often a status struct to track runtime state.
 */
use serde::{Deserialize, Serialize};

/// Serialization and unit conversion helpers for resource specifications.
mod serialization_helpers;
pub use serialization_helpers::{
    ByteUnit, CoreUnit, DefaultFactor, DefaultFactorOne, DefaultFactorOneGigabyte,
    DefaultFactorOneKilobyte, DefaultFactorOneMegabyte, TimeUnit,
};

/// Common metadata models used across different entity types.
mod metadata;
pub use metadata::{Label, Metadata};

/// Task models for computational jobs.
mod task;
pub use task::{InputContext, OutputContext, Task, TaskEnv, TaskResources, TaskSpec, TaskStatus};

/// Worker models for compute providers.
mod worker;
pub use worker::{Worker, WorkerSpec, WorkerStatus};

/// Pin models for data availability.
mod pin;
pub use pin::{Pin, PinAck, PinSpec, PinStatus};

/// Workflow models for coordinating sequences of tasks.
mod workflow;
pub use workflow::{Workflow, WorkflowSpec, WorkflowStage, WorkflowStageStatus, WorkflowStatus};

/// Generic representation of any Gevulot entity.
///
/// This struct provides a way to handle any Gevulot entity in a generic way,
/// particularly useful for cases where the specific entity type is not known at compile time.
/// It includes common fields (kind, version, metadata) and uses serde_json::Value
/// for the spec and status fields to allow flexible deserialization.
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{Generic, Metadata};
/// use serde_json::json;
///
/// let generic_entity = Generic {
///     kind: "Task".to_string(),
///     version: "v0".to_string(),
///     metadata: Metadata::default(),
///     spec: json!({
///         "image": "ubuntu:latest",
///         "resources": {
///             "cpus": "1cpu",
///             "memory": "512mb",
///             "time": "1h"
///         }
///     }),
///     status: None,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Generic {
    /// Type identifier indicating the entity kind (e.g., "Task", "Worker", "Pin")
    pub kind: String,
    
    /// API version of the entity format, typically "v0"
    pub version: String,
    
    /// Metadata common to all entity types
    pub metadata: Metadata,
    
    /// Entity specification in a generic JSON format
    pub spec: serde_json::Value,
    
    /// Optional entity status in a generic JSON format
    pub status: Option<serde_json::Value>,
}
