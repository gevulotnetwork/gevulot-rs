//! Common metadata models for Gevulot entities.
//!
//! This module provides the metadata structures used across different Gevulot entity types,
//! including tasks, workers, pins, and workflows. Metadata facilitates resource discovery,
//! organization, and filtering.
//!
//! ## Key Features
//!
//! - **Identification**: IDs, names, and creators for resource attribution
//! - **Documentation**: Descriptions for human-readable context
//! - **Classification**: Tags and labels for categorization and filtering
//! - **Relationships**: References to related entities (like workflows)

use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Common metadata fields used across different Gevulot entity types.
///
/// Metadata provides a standard way to describe, identify, and categorize resources
/// within the Gevulot network. It includes fields for basic information like names 
/// and descriptions, as well as structured data like tags and labels for filtering 
/// and organization.
///
/// # Fields
///
/// * `id` - Optional unique identifier, typically assigned by the system
/// * `name` - Human-readable name for the entity
/// * `creator` - Optional identifier of the entity creator/owner
/// * `description` - Detailed description of the entity's purpose
/// * `tags` - List of simple string tags for basic categorization and filtering
/// * `labels` - List of key-value pairs for more structured classification
/// * `workflow_ref` - Optional reference to a parent workflow (used primarily for tasks)
///
/// # Examples
///
/// ## Basic metadata
///
/// ```
/// use gevulot_rs::models::Metadata;
///
/// let metadata = Metadata {
///     id: None,  // System will assign an ID upon creation
///     name: "counting-task".to_string(),
///     creator: Some("alice".to_string()),
///     description: "A simple counting task".to_string(),
///     tags: vec!["example".to_string(), "simple".to_string()],
///     labels: vec![],
///     workflow_ref: None,
/// };
/// ```
///
/// ## Metadata with labels and workflow reference
///
/// ```
/// use gevulot_rs::models::{Metadata, Label};
///
/// let metadata = Metadata {
///     id: Some("task-123".to_string()),
///     name: "preprocessing-task".to_string(),
///     creator: Some("alice".to_string()),
///     description: "Data preprocessing for ML training".to_string(),
///     tags: vec!["data-processing".to_string(), "ml".to_string()],
///     labels: vec![
///         Label {
///             key: "environment".to_string(),
///             value: "production".to_string()
///         },
///         Label {
///             key: "priority".to_string(),
///             value: "high".to_string()
///         }
///     ],
///     workflow_ref: Some("workflow-456".to_string())
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Metadata {
    /// Unique identifier for the entity, typically assigned by the system.
    /// 
    /// This field is typically None when creating a new entity and will be
    /// populated by the system upon successful creation.
    pub id: Option<String>,
    
    /// Human-readable name for the entity.
    /// 
    /// This should be descriptive enough to identify the entity in listings
    /// and should follow a consistent naming convention for easier organization.
    pub name: String,
    
    /// Creator or owner identifier for the entity.
    /// 
    /// This is typically a blockchain address or user identifier that indicates
    /// who created or currently owns the entity. Used for access control and attribution.
    pub creator: Option<String>,
    
    /// Detailed description of the entity's purpose and functionality.
    /// 
    /// This field should provide enough context for other users to understand
    /// what the entity does without having to examine its details.
    pub description: String,
    
    /// List of searchable tags for basic categorization and filtering.
    /// 
    /// Tags are simple string values that can be used for filtering in queries.
    /// They should be concise and follow a consistent taxonomy when possible.
    pub tags: Vec<String>,
    
    /// List of key-value label pairs for more structured classification.
    /// 
    /// Labels provide more structured metadata than tags, allowing for
    /// key-based lookups and hierarchical organization of resources.
    pub labels: Vec<Label>,
    
    /// Reference to a parent workflow, if this entity is part of one.
    /// 
    /// This field is primarily used for tasks that are part of a larger workflow.
    /// It allows for tracking the relationship between tasks and their parent workflows.
    #[serde(rename = "workflowRef")]
    pub workflow_ref: Option<String>,
}

/// Key-value pair used for structured resource classification and filtering.
///
/// Labels provide a more flexible and powerful mechanism for categorizing resources
/// compared to simple tags. They consist of a key and a value, allowing for
/// hierarchical organization and more precise querying.
///
/// # Common Uses
///
/// * Environment categorization (e.g., "environment: production")
/// * Ownership tracking (e.g., "team: infrastructure")
/// * Version tracking (e.g., "version: 1.2.3")
/// * Priority assignment (e.g., "priority: high")
/// * Custom classification (e.g., "data-type: images")
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::Label;
///
/// // Environment label
/// let env_label = Label {
///     key: "environment".to_string(),
///     value: "production".to_string()
/// };
///
/// // Priority label
/// let priority_label = Label {
///     key: "priority".to_string(),
///     value: "high".to_string()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    /// The label's identifier or category.
    /// 
    /// The key should follow a consistent naming pattern, typically
    /// using lowercase letters, numbers, and hyphens.
    pub key: String,
    
    /// The label's assigned value.
    /// 
    /// The value can be any string, but it's recommended to use
    /// consistent values for the same keys across different resources.
    pub value: String,
}

impl From<gevulot::Label> for Label {
    /// Converts a protobuf Label message into the domain Label struct.
    ///
    /// This allows for seamless conversion from the wire format used
    /// in blockchain communication to the application domain model.
    fn from(proto: gevulot::Label) -> Self {
        Label {
            key: proto.key,
            value: proto.value,
        }
    }
}

impl From<Label> for gevulot::Label {
    /// Converts a domain Label struct into a protobuf Label message.
    ///
    /// This allows for seamless conversion from the application domain
    /// model to the wire format used in blockchain communication.
    fn from(val: Label) -> Self {
        gevulot::Label {
            key: val.key,
            value: val.value,
        }
    }
}
