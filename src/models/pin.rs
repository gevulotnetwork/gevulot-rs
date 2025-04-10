//! Pin model and related types for managing pinned data in the Gevulot network.
//!
//! This module provides the core data pinning model used throughout the system, including:
//! - Pin specifications for data storage requirements
//! - Status tracking of pinned data
//! - Worker acknowledgment handling
//! - Content ID (CID) and fallback URL management
//!
//! Pins are a critical part of the Gevulot network's data availability system. They
//! represent data that should be persistently stored by worker nodes for a specified
//! duration, ensuring data remains accessible for computational tasks.
//!
//! # Key Components
//!
//! - [`Pin`] - Complete pin definition including metadata, specification, and status
//! - [`PinSpec`] - Defines data storage requirements (size, time, redundancy)
//! - [`PinStatus`] - Tracks worker assignments and acknowledgments
//! - [`PinAck`] - Records individual worker acknowledgments of pinned data
//!
//! # Pin Lifecycle
//!
//! A typical pin follows this lifecycle:
//! 1. **Created** - A pin request is submitted with CID or fallback URLs
//! 2. **Assigned** - Workers are assigned to store the data
//! 3. **Acknowledged** - Workers confirm they have stored the data
//! 4. **Maintained** - Data is stored for the specified duration
//! 5. **Expired** - After the time period elapses, data may be removed

use super::{
    metadata::{Label, Metadata},
    serialization_helpers::{ByteUnit, DefaultFactorOne, TimeUnit},
};
use crate::proto::gevulot::gevulot;
use serde::{Deserialize, Serialize};

/// Represents a Pin resource for storing data in the network
///
/// A Pin defines what data should be stored, for how long, and with what redundancy level.
/// The data can be referenced either by CID or fallback URLs.
///
/// # Fields
///
/// * `kind` - Type identifier, always "Pin" for pin entities
/// * `version` - Schema version, typically "v0"
/// * `metadata` - Pin identification, description, and classification information
/// * `spec` - Data storage specifications including size, duration, and redundancy
/// * `status` - Optional current status including worker assignments and acknowledgments
///
/// # Examples
///
/// Creating a Pin with CID:
/// ```
/// use gevulot_rs::models::{Pin, PinSpec, Metadata};
///
/// let pin = Pin {
///     kind: "Pin".to_string(),
///     version: "v0".to_string(),
///     metadata: Metadata {
///         name: "my-data".to_string(),
///         ..Default::default()
///     },
///     spec: PinSpec {
///         cid: Some("QmExample123".to_string()),
///         bytes: "1GB".parse().unwrap(),
///         time: "24h".parse().unwrap(),
///         redundancy: 3,
///         fallback_urls: None,
///     },
///     status: None,
/// };
/// ```
///
/// Creating a Pin with fallback URLs:
/// ```
/// use gevulot_rs::models::{Pin, PinSpec, Metadata};
///
/// let pin = Pin {
///     kind: "Pin".to_string(),
///     version: "v0".to_string(),
///     metadata: Metadata {
///         name: "my-backup".to_string(),
///         ..Default::default()
///     },
///     spec: PinSpec {
///         cid: None,
///         bytes: "500MB".parse().unwrap(),
///         time: "7d".parse().unwrap(),
///         redundancy: 2,
///         fallback_urls: Some(vec![
///             "https://example.com/backup1".to_string(),
///             "https://backup.example.com/data".to_string()
///         ]),
///     },
///     status: None,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    /// Type identifier, always "Pin" for this struct
    /// Used for type identification in serialized form
    pub kind: String,
    
    /// API version for the pin format, currently "v0"
    /// This allows for future schema evolution
    pub version: String,
    
    /// Pin metadata like name, description, tags, and identifying information
    /// Used for filtering, searching, and referencing pins
    #[serde(default)]
    pub metadata: Metadata,
    
    /// Core pin specification containing data storage parameters
    /// Defines what data to store and the required resources
    pub spec: PinSpec,
    
    /// Runtime status of the pin, populated during data storage
    /// Contains worker assignments and acknowledgment information
    pub status: Option<PinStatus>,
}

/// Converts a protobuf Pin message to the internal Pin model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
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

/// Specification for a Pin resource
///
/// Defines the key parameters for pinning data including size, duration and redundancy.
/// Either a CID or fallback URLs must be specified.
///
/// # Fields
///
/// * `cid` - Optional Content Identifier for the data to pin
/// * `bytes` - Size of the data in bytes with human-readable formatting
/// * `time` - Duration to keep the data pinned for availability
/// * `redundancy` - Number of worker nodes that should store copies of the data
/// * `fallback_urls` - Optional list of URLs where the data can be retrieved from
///
/// # Data Identification
///
/// A pin must identify data either through:
/// - A content identifier (CID) - Preferred if data is already in the network
/// - Fallback URLs - Alternative sources to retrieve the data from
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::PinSpec;
///
/// let spec = PinSpec {
///     cid: Some("QmExample123".to_string()),
///     bytes: "1GB".parse().unwrap(),
///     time: "24h".parse().unwrap(),
///     redundancy: 3,
///     fallback_urls: None,
/// };
/// ```
#[derive(Serialize, Debug)]
pub struct PinSpec {
    /// Content identifier for the data to pin
    /// If not present, fallback_urls must be provided
    #[serde(default)]
    pub cid: Option<String>,
    
    /// Size of the data in human-readable format (e.g., "1GB")
    /// Used to estimate storage requirements and costs
    pub bytes: ByteUnit<DefaultFactorOne>,
    
    /// Duration to keep the data pinned using human-readable format (e.g., "7d")
    /// Data may be garbage collected after this period expires
    pub time: TimeUnit,
    
    /// Number of worker nodes that should store copies of the data
    /// Higher values increase data availability and fault tolerance
    pub redundancy: i64,
    
    /// Alternative URLs where the data can be retrieved from
    /// Required if no CID is specified
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

/// Converts a protobuf PinSpec message to the internal PinSpec model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
impl From<gevulot::PinSpec> for PinSpec {
    fn from(proto: gevulot::PinSpec) -> Self {
        PinSpec {
            cid: None,
            bytes: proto.bytes.into(),
            time: proto.time.into(),
            redundancy: proto.redundancy as i64,
            fallback_urls: Some(proto.fallback_urls),
        }
    }
}

/// Status information for a Pin
///
/// Tracks which workers are assigned to store the data and their acknowledgments.
/// This provides visibility into the current state of data availability.
///
/// # Fields
///
/// * `assigned_workers` - List of worker IDs assigned to store the data
/// * `worker_acks` - List of acknowledgments from workers confirming storage
/// * `cid` - Content identifier for the pinned data (may be updated after pinning)
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::{PinStatus, PinAck};
///
/// let status = PinStatus {
///     assigned_workers: vec!["worker1".to_string(), "worker2".to_string()],
///     worker_acks: vec![
///         PinAck {
///             worker: "worker1".to_string(),
///             block_height: 1000,
///             success: true,
///             error: None,
///         }
///     ],
///     cid: Some("QmExample123".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct PinStatus {
    /// List of worker IDs assigned to store this data
    /// The number of workers should typically match the redundancy level
    #[serde(rename = "assignedWorkers", default)]
    pub assigned_workers: Vec<String>,
    
    /// List of acknowledgments from workers confirming data storage
    /// Each acknowledgment includes the block height when storage was confirmed
    #[serde(rename = "workerAcks", default)]
    pub worker_acks: Vec<PinAck>,
    
    /// Content identifier for the pinned data
    /// May be updated after pinning if data was retrieved from fallback URLs
    pub cid: Option<String>,
}

/// Converts a protobuf PinStatus message to the internal PinStatus model.
///
/// This implementation handles the conversion from the low-level protobuf
/// representation to the higher-level domain model, ensuring proper
/// field mapping and type conversions.
impl From<gevulot::PinStatus> for PinStatus {
    fn from(proto: gevulot::PinStatus) -> Self {
        PinStatus {
            assigned_workers: proto.assigned_workers,
            worker_acks: proto
                .worker_acks
                .into_iter()
                .map(|ack| PinAck {
                    worker: ack.worker,
                    block_height: ack.block_height as i64,
                    success: ack.success,
                    error: if ack.error.is_empty() {
                        None
                    } else {
                        Some(ack.error)
                    },
                })
                .collect(),
            cid: Some(proto.cid),
        }
    }
}

/// Acknowledgment from a worker that it has processed a pin request
///
/// This records whether a worker has successfully stored the pinned data,
/// along with block height for verification and any error information.
///
/// # Fields
///
/// * `worker` - ID of the worker that provided this acknowledgment
/// * `block_height` - Blockchain height when acknowledgment was recorded
/// * `success` - Whether the worker successfully stored the data
/// * `error` - Optional error message if storage failed
///
/// # Examples
///
/// ```
/// use gevulot_rs::models::PinAck;
///
/// let ack = PinAck {
///     worker: "worker1".to_string(),
///     block_height: 1000,
///     success: true,
///     error: None,
/// };
///
/// let error_ack = PinAck {
///     worker: "worker2".to_string(),
///     block_height: 1001,
///     success: false,
///     error: Some("Failed to retrieve data from fallback URLs".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct PinAck {
    /// ID of the worker that provided this acknowledgment
    pub worker: String,
    
    /// Blockchain height when acknowledgment was recorded
    /// Useful for verification and auditing purposes
    #[serde(rename = "blockHeight")]
    pub block_height: i64,
    
    /// Whether the worker successfully stored the data
    /// False indicates the worker encountered an error
    pub success: bool,
    
    /// Optional error message if the worker failed to store the data
    /// Provides context about why storage failed
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
        assert!(status.worker_acks[0].success);
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
