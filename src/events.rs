use cosmrs::tendermint::block::Height;

use crate::error::Error;

#[derive(Clone, Debug)]
pub enum GevulotEvent {
    Pin(PinEvent),
    Task(TaskEvent),
    Worker(WorkerEvent),
    Workflow(WorkflowEvent),
}

impl GevulotEvent {
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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

                let assigned_workers = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key_bytes() == b"assigned-workers")
                    .map(|attr| attr.value_str().map(|x| x.to_string()))
                    .collect::<Result<Vec<String>, _>>()?;

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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();

                Ok(GevulotEvent::Task(TaskEvent::Finish(TaskFinishEvent {
                    block_height,
                    task_id,
                    creator,
                    worker_id,
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

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();
                Ok(GevulotEvent::Task(TaskEvent::Decline(TaskDeclineEvent {
                    block_height,
                    task_id,
                    creator,
                    worker_id,
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

                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

                let worker_id = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"worker-id")
                    .ok_or(Error::MissingEventAttribute("worker-id"))?
                    .value_str()?
                    .to_string();
                Ok(GevulotEvent::Task(TaskEvent::Accept(TaskAcceptEvent {
                    block_height,
                    task_id,
                    creator,
                    worker_id,
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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

                Ok(GevulotEvent::Workflow(WorkflowEvent::Delete(
                    WorkflowDeleteEvent {
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
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();

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
                    .map(|attr| attr.value_str().map(|x| x.to_string()))
                    .collect::<Result<Vec<String>, _>>()?;
                let retention_period = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"retention-period")
                    .ok_or(Error::MissingEventAttribute("retention-period"))?
                    .value_str()?
                    .parse()
                    .map_err(|_| Error::InvalidEventAttribute("retention-period"))?;
                Ok(GevulotEvent::Pin(PinEvent::Create(PinCreateEvent {
                    block_height,
                    cid,
                    creator,
                    assigned_workers,
                    retention_period,
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

                Ok(GevulotEvent::Pin(PinEvent::Delete(PinDeleteEvent {
                    block_height,
                    cid,
                    creator,
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
                let creator = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key_bytes() == b"creator")
                    .ok_or(Error::MissingEventAttribute("creator"))?
                    .value_str()?
                    .to_string();
                Ok(GevulotEvent::Pin(PinEvent::Ack(PinAckEvent {
                    block_height,
                    cid,
                    worker_id,
                    creator,
                })))
            }
            _ => Err(Error::UnknownEventKind(event.kind.clone())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PinCreateEvent {
    pub block_height: Height,
    pub cid: String,
    pub creator: String,
    pub assigned_workers: Vec<String>,
    pub retention_period: u64,
}

#[derive(Clone, Debug)]
pub struct PinDeleteEvent {
    pub block_height: Height,
    pub cid: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct PinAckEvent {
    pub block_height: Height,
    pub cid: String,
    pub creator: String,
    pub worker_id: String,
}

#[derive(Clone, Debug)]
pub enum PinEvent {
    Create(PinCreateEvent),
    Delete(PinDeleteEvent),
    Ack(PinAckEvent),
}

#[derive(Clone, Debug)]
pub struct TaskCreateEvent {
    pub block_height: Height,
    pub task_id: String,
    pub creator: String,
    pub assigned_workers: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct TaskDeleteEvent {
    pub block_height: Height,
    pub task_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct TaskAcceptEvent {
    pub block_height: Height,
    pub task_id: String,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct TaskDeclineEvent {
    pub block_height: Height,
    pub task_id: String,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct TaskFinishEvent {
    pub block_height: Height,
    pub task_id: String,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub enum TaskEvent {
    Create(TaskCreateEvent),
    Delete(TaskDeleteEvent),
    Accept(TaskAcceptEvent),
    Decline(TaskDeclineEvent),
    Finish(TaskFinishEvent),
}

#[derive(Clone, Debug)]
pub struct WorkerCreateEvent {
    pub block_height: Height,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct WorkerUpdateEvent {
    pub block_height: Height,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct WorkerDeleteEvent {
    pub block_height: Height,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct WorkerAnnounceExitEvent {
    pub block_height: Height,
    pub worker_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub enum WorkerEvent {
    Create(WorkerCreateEvent),
    Update(WorkerUpdateEvent),
    Delete(WorkerDeleteEvent),
    AnnounceExit(WorkerAnnounceExitEvent),
}

#[derive(Clone, Debug)]
pub struct WorkflowCreateEvent {
    pub block_height: Height,
    pub workflow_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct WorkflowDeleteEvent {
    pub block_height: Height,
    pub workflow_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct WorkflowProgressEvent {
    pub block_height: Height,
    pub workflow_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub struct WorkflowFinishEvent {
    pub block_height: Height,
    pub workflow_id: String,
    pub creator: String,
}

#[derive(Clone, Debug)]
pub enum WorkflowEvent {
    Create(WorkflowCreateEvent),
    Delete(WorkflowDeleteEvent),
    Progress(WorkflowProgressEvent),
    Finish(WorkflowFinishEvent),
}
