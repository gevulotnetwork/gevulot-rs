use serde::{Deserialize, Serialize};
use crate::proto::gevulot::gevulot;

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
