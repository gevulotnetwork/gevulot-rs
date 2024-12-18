use serde::{Deserialize, Serialize};
use crate::proto::gevulot::gevulot;

/// Metadata represents common metadata fields used across different resource types.
///
/// # Examples
///
/// ```
/// use crate::models::Metadata;
/// use crate::models::Label;
///
/// let metadata = Metadata {
///     id: Some("task-123".to_string()),
///     name: "my-task".to_string(), 
///     creator: Some("alice".to_string()),
///     description: "An example task".to_string(),
///     tags: vec!["tag1".to_string(), "tag2".to_string()],
///     labels: vec![
///         Label {
///             key: "env".to_string(),
///             value: "prod".to_string()
///         }
///     ],
///     workflow_ref: None
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Metadata {
    /// Unique identifier for the resource
    pub id: Option<String>,
    /// Name of the resource
    pub name: String,
    /// Creator/owner of the resource
    pub creator: Option<String>,
    /// Detailed description of the resource
    pub description: String,
    /// List of searchable tags
    pub tags: Vec<String>,
    /// List of key-value labels
    pub labels: Vec<Label>,
    /// Reference to a parent workflow (only used in TaskMetadata)
    #[serde(rename = "workflowRef")]
    pub workflow_ref: Option<String>,
}

/// Label represents a key-value pair used for resource classification and filtering.
///
/// # Examples
///
/// ```
/// use crate::models::Label;
///
/// let label = Label {
///     key: "environment".to_string(),
///     value: "production".to_string()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    /// The label key
    pub key: String,
    /// The label value
    pub value: String,
}

impl From<gevulot::Label> for Label {
    /// Converts a protobuf Label into our domain Label
    fn from(proto: gevulot::Label) -> Self {
        Label {
            key: proto.key,
            value: proto.value,
        }
    }
}

impl From<Label> for gevulot::Label {
    /// Converts our domain Label into a protobuf Label
    fn from(val: Label) -> Self {
        gevulot::Label {
            key: val.key,
            value: val.value,
        }
    }
}
