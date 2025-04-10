/*! Events emitted by the Gevulot blockchain.

This module provides types and functionality for parsing and handling blockchain events
emitted by the Gevulot network. These events represent state changes in the blockchain,
such as worker registration, task creation, workflow updates, and data pinning operations.

Events are parsed from Cosmos SDK events emitted by the Ember blockchain component 
and converted into strongly-typed Rust structs for easier consumption by client applications.

# Main Components

- [`GevulotEvent`] - The primary enum representing all possible events from the Gevulot chain.
- [`WorkerEvent`] - Events related to worker nodes (registration, updates, etc.)
- [`TaskEvent`] - Events related to computation tasks (creation, acceptance, completion)
- [`WorkflowEvent`] - Events related to multi-task workflows (creation, progress updates)
- [`PinEvent`] - Events related to data pinning operations (creating, deleting, acknowledging)
- [`ProofEvent`] - Events related to zero-knowledge proof operations

# Usage Example

```rust,no_run
use cosmrs::tendermint::abci::Event;
use cosmrs::tendermint::block::Height;
use gevulot_rs::events::GevulotEvent;

// Parse an event from a Cosmos SDK event
fn process_event(event: &Event, height: Height) {
    match GevulotEvent::from_cosmos(event, height) {
        Ok(GevulotEvent::Worker(worker_event)) => {
            // Handle worker event
            println!("Received worker event: {:?}", worker_event);
        }
        Ok(GevulotEvent::Task(task_event)) => {
            // Handle task event
            println!("Received task event: {:?}", task_event);
        }
        Ok(_) => {
            // Handle other event types...
        }
        Err(e) => {
            // Handle error
            eprintln!("Failed to parse event: {:?}", e);
        }
    }
}
```
*/

use cosmrs::tendermint::block::Height;

use crate::error::Error;

/// Represents all possible events emitted by the Gevulot blockchain.
///
/// This enum is the primary entry point for working with Gevulot events.
/// Each variant corresponds to a major subsystem in the Gevulot network:
/// - `Worker` events relate to worker node lifecycle
/// - `Task` events relate to computation task lifecycle
/// - `Workflow` events relate to workflow management
/// - `Pin` events relate to data pinning operations
/// - `Proof` events relate to zero-knowledge proof operations
#[derive(Clone, Debug)]
pub enum GevulotEvent {
    /// Events related to pinning and unpinning data
    Pin(PinEvent),
    /// Events related to task lifecycle (create, accept, finish)
    Task(TaskEvent),
    /// Events related to worker node lifecycle (register, update, delete)
    Worker(WorkerEvent),
    /// Events related to workflow management
    Workflow(WorkflowEvent),
    /// Events related to zero-knowledge proof operations
    Proof(ProofEvent),
}

impl GevulotEvent {
    /// Parses a Cosmos SDK event into a strongly-typed `GevulotEvent`.
    ///
    /// This function examines the event type and attributes to determine the
    /// appropriate event variant and constructs the corresponding event struct.
    ///
    /// # Arguments
    ///
    /// * `event` - The Cosmos SDK event to parse
    /// * `block_height` - The height of the block containing the event
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed event or an error if parsing fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The event kind is unknown
    /// - Required attributes are missing
    /// - Attribute values cannot be parsed
    pub fn from_cosmos(
        event: &cosmrs::tendermint::abci::Event,
        block_height: Height,
    ) -> crate::error::Result<Self> {
        match event.kind.as_str() {
            "create-worker" => {
                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Worker(WorkerEvent::Create(
                    WorkerCreateEvent {
                        block_height,
                        worker_id,
                        creator,
                    },
                )))
            }
            "update-worker" => {
                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Worker(WorkerEvent::Update(
                    WorkerUpdateEvent {
                        block_height,
                        worker_id,
                        creator,
                    },
                )))
            }
            "delete-worker" => {
                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Worker(WorkerEvent::Delete(
                    WorkerDeleteEvent {
                        block_height,
                        worker_id,
                        creator,
                    },
                )))
            }
            "announce-worker-exit" => {
                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Worker(WorkerEvent::AnnounceExit(
                    WorkerAnnounceExitEvent {
                        block_height,
                        worker_id,
                        creator,
                    },
                )))
            }
            "create-task" => {
                let task_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"task-id")
                    .ok_or(Error::MissingEventAttribute("task-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                let assigned_workers = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key_bytes() == b"worker-id")
                    .flat_map(|attr| {
                        attr.value_str()
                            .map(|s| {
                                s.split(',')
                                    .map(|x| x.trim().to_string())
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    })
                    .collect::<Vec<String>>();

                Ok(GevulotEvent::Task(TaskEvent::Create(TaskCreateEvent {
                    block_height,
                    task_id,
                    creator,
                    assigned_workers,
                })))
            }
            "delete-task" => {
                let task_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"task-id")
                    .ok_or(Error::MissingEventAttribute("task-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Task(TaskEvent::Delete(TaskDeleteEvent {
                    block_height,
                    task_id,
                    creator,
                })))
            }
            "finish-task" => {
                let task_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"task-id")
                    .ok_or(Error::MissingEventAttribute("task-id"))?
                    .value_str()?
                    .to_string();

                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Task(TaskEvent::Finish(TaskFinishEvent {
                    block_height,
                    task_id,
                    worker_id,
                    creator,
                })))
            }
            "decline-task" => {
                let task_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"task-id")
                    .ok_or(Error::MissingEventAttribute("task-id"))?
                    .value_str()?
                    .to_string();

                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Task(TaskEvent::Decline(TaskDeclineEvent {
                    block_height,
                    task_id,
                    worker_id,
                    creator,
                })))
            }
            "accept-task" => {
                let task_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"task-id")
                    .ok_or(Error::MissingEventAttribute("task-id"))?
                    .value_str()?
                    .to_string();

                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Task(TaskEvent::Accept(TaskAcceptEvent {
                    block_height,
                    task_id,
                    worker_id,
                    creator,
                })))
            }
            "create-workflow" => {
                let workflow_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"workflow-id")
                    .ok_or(Error::MissingEventAttribute("workflow-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Workflow(WorkflowEvent::Create(
                    WorkflowCreateEvent {
                        block_height,
                        workflow_id,
                        creator,
                    },
                )))
            }
            "delete-workflow" => {
                let workflow_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"workflow-id")
                    .ok_or(Error::MissingEventAttribute("workflow-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Workflow(WorkflowEvent::Delete(
                    WorkflowDeleteEvent {
                        block_height,
                        workflow_id,
                        creator,
                    },
                )))
            }
            "update-workflow" => {
                let workflow_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"workflow-id")
                    .ok_or(Error::MissingEventAttribute("workflow-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Workflow(WorkflowEvent::Update(
                    WorkflowUpdateEvent {
                        block_height,
                        workflow_id,
                        creator,
                    },
                )))
            }
            "finish-workflow" => {
                let workflow_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"workflow-id")
                    .ok_or(Error::MissingEventAttribute("workflow-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Workflow(WorkflowEvent::Finish(
                    WorkflowFinishEvent {
                        block_height,
                        workflow_id,
                        creator,
                    },
                )))
            }
            "progress-workflow" => {
                let workflow_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"workflow-id")
                    .ok_or(Error::MissingEventAttribute("workflow-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

                Ok(GevulotEvent::Workflow(WorkflowEvent::Progress(
                    WorkflowProgressEvent {
                        block_height,
                        workflow_id,
                        creator,
                    },
                )))
            }
            "create-pin" => {
                let cid = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"cid")
                    .ok_or(Error::MissingEventAttribute("cid"))?
                    .value_str()?
                    .to_string();
                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();
                let assigned_workers = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key_bytes() == b"assigned-workers")
                    .flat_map(|attr| {
                        attr.value_str()
                            .map(|s| {
                                s.split(',')
                                    .map(|x| x.trim().to_string())
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    })
                    .collect::<Vec<String>>();
                let retention_period = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"retention-period")
                    .ok_or(Error::MissingEventAttribute("retention-period"))?
                    .value_str()?
                    .parse()
                    .map_err(|_| Error::InvalidEventAttribute("retention-period"))?;
                let fallback_urls = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key_bytes() == b"fallback-urls")
                    .flat_map(|attr| {
                        attr.value_str()
                            .map(|s| {
                                s.split(',')
                                    .map(|x| x.trim().to_string())
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    })
                    .filter(|url| !url.is_empty())
                    .collect::<Vec<String>>();
                let id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"id")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_else(|| cid.clone());

                Ok(GevulotEvent::Pin(PinEvent::Create(PinCreateEvent {
                    block_height,
                    cid,
                    creator,
                    assigned_workers,
                    retention_period,
                    fallback_urls,
                    id,
                })))
            }
            "delete-pin" => {
                let cid = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"cid")
                    .ok_or(Error::MissingEventAttribute("cid"))?
                    .value_str()?
                    .to_string();
                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();
                let id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"id")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_else(|| cid.clone());

                Ok(GevulotEvent::Pin(PinEvent::Delete(PinDeleteEvent {
                    block_height,
                    cid,
                    creator,
                    id,
                })))
            }
            "ack-pin" => {
                let cid = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"cid")
                    .ok_or(Error::MissingEventAttribute("cid"))?
                    .value_str()?
                    .to_string();
                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();
                let success = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"success")
                    .map(|attr| attr.value_str().unwrap_or("true").parse().unwrap_or(true))
                    .unwrap_or(true);
                let id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"id")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_else(|| cid.clone());
                Ok(GevulotEvent::Pin(PinEvent::Ack(PinAckEvent {
                    block_height,
                    cid,
                    worker_id,
                    success,
                    id,
                })))
            }
            "create-proof" => {
                let proof_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"proof-id")
                    .ok_or(Error::MissingEventAttribute("proof-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Proof(ProofEvent::Create(
                    ProofCreateEvent {
                        block_height,
                        proof_id,
                        creator,
                    },
                )))
            }
            "update-proof" => {
                let proof_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"proof-id")
                    .ok_or(Error::MissingEventAttribute("proof-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Proof(ProofEvent::Update(
                    ProofUpdateEvent {
                        block_height,
                        proof_id,
                        creator,
                    },
                )))
            }
            "delete-proof" => {
                let proof_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"proof-id")
                    .ok_or(Error::MissingEventAttribute("proof-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Proof(ProofEvent::Delete(
                    ProofDeleteEvent {
                        block_height,
                        proof_id,
                        creator,
                    },
                )))
            }
            "finish-proof" => {
                let proof_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"proof-id")
                    .ok_or(Error::MissingEventAttribute("proof-id"))?
                    .value_str()?
                    .to_string();

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .map(|attr| attr.value_str().unwrap_or_default().to_string())
                    .unwrap_or_default();

                Ok(GevulotEvent::Proof(ProofEvent::Finish(
                    ProofFinishEvent {
                        block_height,
                        proof_id,
                        creator,
                    },
                )))
            }
            _ => Err(Error::UnknownEventKind(event.kind.clone())),
        }
    }
}

/// Represents an event for creating a new pin (data storage request).
#[derive(Clone, Debug)]
pub struct PinCreateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The content identifier (CID) of the data to pin
    pub cid: String,
    /// The unique identifier for this pin request 
    pub id: String,
    /// The address of the account that created the pin
    pub creator: String,
    /// The list of worker IDs assigned to store this data
    pub assigned_workers: Vec<String>,
    /// The period (in blocks) for which the data should be retained
    pub retention_period: u64,
    /// Optional URLs where the data can also be found
    pub fallback_urls: Vec<String>,
}

/// Represents an event for deleting a pin.
#[derive(Clone, Debug)]
pub struct PinDeleteEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The content identifier (CID) of the data that was pinned
    pub cid: String,
    /// The unique identifier for this pin 
    pub id: String,
    /// The address of the account that created the pin
    pub creator: String,
}

/// Represents an acknowledgment event from a worker about a pin request.
#[derive(Clone, Debug)]
pub struct PinAckEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The content identifier (CID) of the data
    pub cid: String,
    /// The unique identifier for this pin
    pub id: String,
    /// The ID of the worker acknowledging the pin
    pub worker_id: String,
    /// Whether the worker successfully pinned the data
    pub success: bool,
}

/// Represents events related to data pinning operations.
#[derive(Clone, Debug)]
pub enum PinEvent {
    /// A new pin request was created
    Create(PinCreateEvent),
    /// A pin was deleted
    Delete(PinDeleteEvent),
    /// A worker acknowledged a pin request
    Ack(PinAckEvent),
}

/// Represents an event for creating a new computation task.
#[derive(Clone, Debug)]
pub struct TaskCreateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the task
    pub task_id: String,
    /// The address of the account that created the task
    pub creator: String,
    /// The list of worker IDs assigned to this task
    pub assigned_workers: Vec<String>,
}

/// Represents an event for deleting a task.
#[derive(Clone, Debug)]
pub struct TaskDeleteEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the task
    pub task_id: String,
    /// The address of the account that created the task
    pub creator: String,
}

/// Represents an event for a worker accepting a task.
#[derive(Clone, Debug)]
pub struct TaskAcceptEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the task
    pub task_id: String,
    /// The ID of the worker that accepted the task
    pub worker_id: String,
    /// The address of the account that created the task
    pub creator: String,
}

/// Represents an event for a worker declining a task.
#[derive(Clone, Debug)]
pub struct TaskDeclineEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the task
    pub task_id: String,
    /// The ID of the worker that declined the task
    pub worker_id: String,
    /// The address of the account that created the task
    pub creator: String,
}

/// Represents an event for a worker finishing a task.
#[derive(Clone, Debug)]
pub struct TaskFinishEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the task
    pub task_id: String,
    /// The ID of the worker that finished the task
    pub worker_id: String,
    /// The address of the account that created the task
    pub creator: String,
}

/// Represents events related to computation tasks.
#[derive(Clone, Debug)]
pub enum TaskEvent {
    /// A new task was created
    Create(TaskCreateEvent),
    /// A task was deleted
    Delete(TaskDeleteEvent),
    /// A worker accepted a task
    Accept(TaskAcceptEvent),
    /// A worker declined a task
    Decline(TaskDeclineEvent),
    /// A worker finished a task
    Finish(TaskFinishEvent),
}

/// Represents an event for registering a new worker node.
#[derive(Clone, Debug)]
pub struct WorkerCreateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the worker
    pub worker_id: String,
    /// The address of the account that registered the worker
    pub creator: String,
}

/// Represents an event for updating a worker node's information.
#[derive(Clone, Debug)]
pub struct WorkerUpdateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the worker
    pub worker_id: String,
    /// The address of the account that owns the worker
    pub creator: String,
}

/// Represents an event for deregistering a worker node.
#[derive(Clone, Debug)]
pub struct WorkerDeleteEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the worker
    pub worker_id: String,
    /// The address of the account that owns the worker
    pub creator: String,
}

/// Represents an event for a worker announcing its intention to exit the network.
#[derive(Clone, Debug)]
pub struct WorkerAnnounceExitEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the worker
    pub worker_id: String,
    /// The address of the account that owns the worker
    pub creator: String,
}

/// Represents events related to worker node lifecycle.
#[derive(Clone, Debug)]
pub enum WorkerEvent {
    /// A new worker was registered
    Create(WorkerCreateEvent),
    /// A worker's information was updated
    Update(WorkerUpdateEvent),
    /// A worker was deregistered
    Delete(WorkerDeleteEvent),
    /// A worker announced its intention to exit the network
    AnnounceExit(WorkerAnnounceExitEvent),
}

/// Represents an event for creating a new workflow.
///
/// Workflows in Gevulot are sequences of tasks that form a complete computation pipeline.
#[derive(Clone, Debug)]
pub struct WorkflowCreateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the workflow
    pub workflow_id: String,
    /// The address of the account that created the workflow
    pub creator: String,
}

/// Represents an event for deleting a workflow.
#[derive(Clone, Debug)]
pub struct WorkflowDeleteEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the workflow
    pub workflow_id: String,
    /// The address of the account that created the workflow
    pub creator: String,
}

/// Represents an event for updating the progress of a workflow.
#[derive(Clone, Debug)]
pub struct WorkflowProgressEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the workflow
    pub workflow_id: String,
    /// The address of the account that created the workflow
    pub creator: String,
}

/// Represents an event for updating a workflow's definition.
#[derive(Clone, Debug)]
pub struct WorkflowUpdateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the workflow
    pub workflow_id: String,
    /// The address of the account that created the workflow
    pub creator: String,
}

/// Represents an event for completing a workflow.
#[derive(Clone, Debug)]
pub struct WorkflowFinishEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the workflow
    pub workflow_id: String,
    /// The address of the account that created the workflow
    pub creator: String,
}

/// Represents events related to workflow management.
#[derive(Clone, Debug)]
pub enum WorkflowEvent {
    /// A new workflow was created
    Create(WorkflowCreateEvent),
    /// A workflow was deleted
    Delete(WorkflowDeleteEvent),
    /// A workflow's progress was updated
    Progress(WorkflowProgressEvent),
    /// A workflow was completed
    Finish(WorkflowFinishEvent),
    /// A workflow's definition was updated
    Update(WorkflowUpdateEvent),
}

/// Represents an event for creating a new proof operation.
#[derive(Clone, Debug)]
pub struct ProofCreateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the proof
    pub proof_id: String,
    /// The address of the account that initiated the proof operation
    pub creator: String,
}

/// Represents an event for updating a proof operation.
#[derive(Clone, Debug)]
pub struct ProofUpdateEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the proof
    pub proof_id: String,
    /// The address of the account that owns the proof operation
    pub creator: String,
}

/// Represents an event for deleting a proof operation.
#[derive(Clone, Debug)]
pub struct ProofDeleteEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the proof
    pub proof_id: String,
    /// The address of the account that owns the proof operation
    pub creator: String,
}

/// Represents an event for completing a proof operation.
#[derive(Clone, Debug)]
pub struct ProofFinishEvent {
    /// The block height at which the event was emitted
    pub block_height: Height,
    /// The unique identifier for the proof
    pub proof_id: String,
    /// The address of the account that owns the proof operation
    pub creator: String,
}

/// Represents events related to proof operations.
///
/// Proofs in Gevulot refer to zero-knowledge proofs that can be used
/// to verify computations without revealing the underlying data.
#[derive(Clone, Debug)]
pub enum ProofEvent {
    /// A new proof operation was created
    Create(ProofCreateEvent),
    /// A proof was updated
    Update(ProofUpdateEvent),
    /// A proof was deleted
    Delete(ProofDeleteEvent),
    /// A proof operation was completed
    Finish(ProofFinishEvent),
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmrs::{rpc::dialect::v0_34::EventAttribute, tendermint::abci::Event};

    #[test]
    fn test_from_cosmos_create_pin() {
        let event = Event::new(
            "create-pin",
            vec![
                EventAttribute {
                    index: true,
                    key: b"cid".to_vec(),
                    value: b"QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"assigned-workers".to_vec(),
                    value: b"1,2,3".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"retention-period".to_vec(),
                    value: b"86400".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"fallback-urls".to_vec(),
                    value: b"https://example1.com,https://example2.org".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"fallback-urls".to_vec(),
                    value: b"https://example3.com".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Pin(PinEvent::Create(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.cid, "QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
            assert_eq!(event.assigned_workers, vec!["1", "2", "3"]);
            assert_eq!(event.retention_period, 86400);
            assert_eq!(
                event.fallback_urls,
                vec![
                    "https://example1.com",
                    "https://example2.org",
                    "https://example3.com"
                ]
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_delete_pin() {
        let event = Event::new(
            "delete-pin",
            vec![
                EventAttribute {
                    index: true,
                    key: b"cid".to_vec(),
                    value: b"QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Pin(PinEvent::Delete(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.cid, "QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_ack_pin() {
        let event = Event::new(
            "ack-pin",
            vec![
                EventAttribute {
                    index: true,
                    key: b"cid".to_vec(),
                    value: b"QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"success".to_vec(),
                    value: b"true".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"id".to_vec(),
                    value: b"123".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Pin(PinEvent::Ack(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.cid, "QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y");
            assert_eq!(event.worker_id, "worker1");
            assert!(event.success);
            assert_eq!(event.id, "123");
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_create_worker() {
        let event = Event::new(
            "create-worker",
            vec![
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Worker(WorkerEvent::Create(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.worker_id, "worker1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_update_worker() {
        let event = Event::new(
            "update-worker",
            vec![
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Worker(WorkerEvent::Update(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.worker_id, "worker1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_delete_worker() {
        let event = Event::new(
            "delete-worker",
            vec![
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Worker(WorkerEvent::Delete(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.worker_id, "worker1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_announce_worker_exit() {
        let event = Event::new(
            "announce-worker-exit",
            vec![
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Worker(WorkerEvent::AnnounceExit(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.worker_id, "worker1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_create_task() {
        let event = Event::new(
            "create-task",
            vec![
                EventAttribute {
                    index: true,
                    key: b"task-id".to_vec(),
                    value: b"task1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1,worker2".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker3".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Task(TaskEvent::Create(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.task_id, "task1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
            assert_eq!(
                event.assigned_workers,
                vec!["worker1", "worker2", "worker3"]
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_delete_task() {
        let event = Event::new(
            "delete-task",
            vec![
                EventAttribute {
                    index: true,
                    key: b"task-id".to_vec(),
                    value: b"task1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Task(TaskEvent::Delete(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.task_id, "task1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_finish_task() {
        let event = Event::new(
            "finish-task",
            vec![
                EventAttribute {
                    index: true,
                    key: b"task-id".to_vec(),
                    value: b"task1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Task(TaskEvent::Finish(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.task_id, "task1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
            assert_eq!(event.worker_id, "worker1");
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_create_workflow() {
        let event = Event::new(
            "create-workflow",
            vec![
                EventAttribute {
                    index: true,
                    key: b"workflow-id".to_vec(),
                    value: b"workflow1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Workflow(WorkflowEvent::Create(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.workflow_id, "workflow1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_delete_workflow() {
        let event = Event::new(
            "delete-workflow",
            vec![
                EventAttribute {
                    index: true,
                    key: b"workflow-id".to_vec(),
                    value: b"workflow1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Workflow(WorkflowEvent::Delete(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.workflow_id, "workflow1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_progress_workflow() {
        let event = Event::new(
            "progress-workflow",
            vec![
                EventAttribute {
                    index: true,
                    key: b"workflow-id".to_vec(),
                    value: b"workflow1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Workflow(WorkflowEvent::Progress(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.workflow_id, "workflow1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_finish_workflow() {
        let event = Event::new(
            "finish-workflow",
            vec![
                EventAttribute {
                    index: true,
                    key: b"workflow-id".to_vec(),
                    value: b"workflow1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Workflow(WorkflowEvent::Finish(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.workflow_id, "workflow1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_accept_task() {
        let event = Event::new(
            "accept-task",
            vec![
                EventAttribute {
                    index: true,
                    key: b"task-id".to_vec(),
                    value: b"task1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Task(TaskEvent::Accept(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.task_id, "task1");
            assert_eq!(event.worker_id, "worker1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_decline_task() {
        let event = Event::new(
            "decline-task",
            vec![
                EventAttribute {
                    index: true,
                    key: b"task-id".to_vec(),
                    value: b"task1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"worker-id".to_vec(),
                    value: b"worker1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Task(TaskEvent::Decline(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.task_id, "task1");
            assert_eq!(event.worker_id, "worker1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_create_proof() {
        let event = Event::new(
            "create-proof",
            vec![
                EventAttribute {
                    index: true,
                    key: b"proof-id".to_vec(),
                    value: b"proof1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Proof(ProofEvent::Create(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.proof_id, "proof1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_update_proof() {
        let event = Event::new(
            "update-proof",
            vec![
                EventAttribute {
                    index: true,
                    key: b"proof-id".to_vec(),
                    value: b"proof1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Proof(ProofEvent::Update(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.proof_id, "proof1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_delete_proof() {
        let event = Event::new(
            "delete-proof",
            vec![
                EventAttribute {
                    index: true,
                    key: b"proof-id".to_vec(),
                    value: b"proof1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Proof(ProofEvent::Delete(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.proof_id, "proof1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_finish_proof() {
        let event = Event::new(
            "finish-proof",
            vec![
                EventAttribute {
                    index: true,
                    key: b"proof-id".to_vec(),
                    value: b"proof1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Proof(ProofEvent::Finish(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.proof_id, "proof1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_from_cosmos_update_workflow() {
        let event = Event::new(
            "update-workflow",
            vec![
                EventAttribute {
                    index: true,
                    key: b"workflow-id".to_vec(),
                    value: b"workflow1".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Workflow(WorkflowEvent::Update(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.workflow_id, "workflow1");
            assert_eq!(
                event.creator,
                "cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh"
            );
        } else {
            panic!("Unexpected event type");
        }
    }
}
