use super::{
    metadata::{Label, Metadata},
    serialization_helpers::{ByteUnit, DefaultFactorOne, TimeUnit},
};
use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pub kind: String,
    pub version: String,
    #[serde(default)]
    pub metadata: Metadata,
    pub spec: PinSpec,
    pub status: Option<PinStatus>,
}

impl From<gevulot::Pin> for Pin {
    fn from(proto: gevulot::Pin) -> Self {
        let mut spec: PinSpec = proto.spec.unwrap().into();
        spec.cid = proto
            .status
            .as_ref()
            .map(|s| s.cid.clone())
            .or_else(|| proto.metadata.as_ref().map(|m| m.id.clone()));
        Pin {
            kind: "Pin".to_string(),
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
            status: proto.status.map(|s| s.into()),
            spec,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PinSpec {
    #[serde(default)]
    pub cid: Option<String>,
    pub bytes: ByteUnit<DefaultFactorOne>,
    pub time: TimeUnit,
    pub redundancy: i64,
    #[serde(rename = "fallbackUrls", default)]
    pub fallback_urls: Option<Vec<String>>,
}

impl<'de> Deserialize<'de> for PinSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Create an intermediate struct for initial deserialization
        #[derive(Deserialize)]
        struct PinSpecHelper {
            #[serde(default)]
            cid: Option<String>,
            bytes: ByteUnit,
            time: TimeUnit,
            redundancy: Option<i64>,
            #[serde(rename = "fallbackUrls", default)]
            fallback_urls: Option<Vec<String>>,
        }

        // Deserialize to the helper struct
        let helper = PinSpecHelper::deserialize(deserializer)?;

        // Validate the fields
        if helper.cid.is_none() {
            // If no CID, must have non-empty fallback URLs
            match &helper.fallback_urls {
                None => {
                    return Err(serde::de::Error::custom(
                        "Either cid or fallbackUrls must be specified",
                    ))
                }
                Some(urls) if urls.is_empty() => {
                    return Err(serde::de::Error::custom(
                        "fallbackUrls must contain at least one URL when no cid is specified",
                    ))
                }
                _ => {}
            }
        }

        let redundancy = helper.redundancy.unwrap_or(1);
        // Convert to final struct
        Ok(PinSpec {
            cid: helper.cid,
            bytes: helper.bytes,
            time: helper.time,
            redundancy,
            fallback_urls: helper.fallback_urls,
        })
    }
}

impl From<gevulot::PinSpec> for PinSpec {
    fn from(proto: gevulot::PinSpec) -> Self {
        PinSpec {
            cid: None,
            bytes: (proto.bytes as i64).into(),
            time: (proto.time as i64).into(),
            redundancy: proto.redundancy as i64,
            fallback_urls: Some(proto.fallback_urls),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinStatus {
    #[serde(rename = "assignedWorkers", default)]
    pub assigned_workers: Vec<String>,
    #[serde(rename = "workerAcks", default)]
    pub worker_acks: Vec<PinAck>,
    pub cid: Option<String>,
}

impl From<gevulot::PinStatus> for PinStatus {
    fn from(proto: gevulot::PinStatus) -> Self {
        PinStatus {
            assigned_workers: proto.assigned_workers,
            worker_acks: proto
                .worker_acks
                .into_iter()
                .map(|a| PinAck {
                    worker: a.worker,
                    block_height: a.block_height as i64,
                    success: a.success,
                    error: if a.error.is_empty() {
                        None
                    } else {
                        Some(a.error)
                    },
                })
                .collect(),
            cid: Some(proto.cid),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinAck {
    pub worker: String,
    #[serde(rename = "blockHeight")]
    pub block_height: i64,
    pub success: bool,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_pin() {
        let pin = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "metadata": {
                "name": "Test Pin",
                "creator": "test",
                "description": "Test Pin Description",
                "tags": ["tag1", "tag2"],
                "labels": [
                    {
                        "key": "label1",
                        "value": "value1"
                    },
                    {
                        "key": "label2",
                        "value": "value2"
                    }
                ],
                "workflowRef": "test-workflow"
            },
            "spec": {
                "cid": "test-cid",
                "bytes": "1234KiB",
                "time": "24h",
                "redundancy": 3,
                "fallbackUrls": ["url1", "url2"]
            },
            "status": {
                "assignedWorkers": ["worker1", "worker2"],
                "workerAcks": [
                    {
                        "worker": "worker1",
                        "blockHeight": 1000,
                        "success": true,
                        "error": null
                    },
                    {
                        "worker": "worker2",
                        "blockHeight": 1001,
                        "success": false,
                        "error": "Failed to pin"
                    }
                ],
                "cid": "test-cid"
            }
        }))
        .unwrap();

        // Verify metadata
        assert_eq!(pin.kind, "Pin");
        assert_eq!(pin.version, "v0");
        assert_eq!(pin.metadata.name, "Test Pin");
        assert_eq!(pin.metadata.creator, Some("test".to_string()));
        assert_eq!(pin.metadata.description, "Test Pin Description");
        assert_eq!(pin.metadata.tags, vec!["tag1", "tag2"]);
        assert_eq!(pin.metadata.labels.len(), 2);
        assert_eq!(pin.metadata.labels[0].key, "label1");
        assert_eq!(pin.metadata.labels[0].value, "value1");
        assert_eq!(pin.metadata.workflow_ref, Some("test-workflow".to_string()));

        // Verify spec
        assert_eq!(pin.spec.cid, Some("test-cid".to_string()));
        assert_eq!(pin.spec.bytes.bytes(), Ok(1234 * 1024));
        assert_eq!(pin.spec.time.seconds(), Ok(24 * 60 * 60));
        assert_eq!(pin.spec.redundancy, 3);
        assert_eq!(
            pin.spec.fallback_urls,
            Some(vec!["url1".to_string(), "url2".to_string()])
        );

        // Verify status
        let status = pin.status.unwrap();
        assert_eq!(status.assigned_workers, vec!["worker1", "worker2"]);
        assert_eq!(status.worker_acks.len(), 2);
        assert_eq!(status.worker_acks[0].worker, "worker1");
        assert_eq!(status.worker_acks[0].block_height, 1000);
        assert_eq!(status.worker_acks[0].success, true);
        assert_eq!(status.worker_acks[0].error, None);
        assert_eq!(status.cid, Some("test-cid".to_string()));
    }

    #[test]
    fn test_parse_pin_with_the_bare_minimum() {
        let pin = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "cid": "test-cid",
                "bytes": "1234KiB",
                "time": "24h",
            }
        }))
        .unwrap();

        assert_eq!(pin.spec.cid, Some("test-cid".to_string()));
        assert_eq!(pin.spec.bytes.bytes(), Ok(1234 * 1024));
        assert_eq!(pin.spec.time.seconds(), Ok(24 * 60 * 60));
        assert_eq!(pin.spec.redundancy, 1);
        assert_eq!(pin.spec.fallback_urls, None);
    }

    #[test]
    fn test_pin_requires_cid_or_fallback_urls() {
        // Should fail without either cid or fallback_urls
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_err());

        // Should succeed with just cid
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "cid": "test-cid",
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_ok());

        // Should succeed with just fallback_urls
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "fallbackUrls": ["url1", "url2"],
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_ok());

        // Should fail with empty fallback_urls array
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0",
            "spec": {
                "fallbackUrls": [],
                "bytes": "1234KiB",
                "time": "24h"
            }
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_pin_with_raw_bytes() {
        // Should accept raw number for bytes field
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin", 
            "version": "v0",
            "spec": {
                "cid": "test-cid",
                "bytes": 1234,
                "time": "24h"
            }
        }));
        assert!(result.is_ok());
        let pin = result.unwrap();
        assert_eq!(pin.spec.bytes.bytes(), Ok(1234));

        // Should accept string number for bytes field
        let result = serde_json::from_value::<Pin>(json!({
            "kind": "Pin",
            "version": "v0", 
            "spec": {
                "cid": "test-cid",
                "bytes": "1234",
                "time": "24h"
            }
        }));
        assert!(result.is_ok());
        let pin = result.unwrap();
        assert_eq!(pin.spec.bytes.bytes(), Ok(1234));
    }
}
