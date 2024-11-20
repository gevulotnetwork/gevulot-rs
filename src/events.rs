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

                Ok(GevulotEvent::Pin(PinEvent::Create(PinCreateEvent {
                    block_height,
                    cid,
                    creator,
                    assigned_workers,
                    retention_period,
                    fallback_urls,
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
    pub fallback_urls: Vec<String>,
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
                    key: b"creator".to_vec(),
                    value: b"cosmos1fl48vsnmsdzcv85q5d2q4z5ajdha8yu34mf0eh".to_vec(),
                },
            ],
        );

        let parsed = GevulotEvent::from_cosmos(&event, Height::from(1000u32));

        assert!(parsed.is_ok());
        if let Ok(GevulotEvent::Pin(PinEvent::Ack(event))) = parsed {
            assert_eq!(event.block_height, Height::from(1000u32));
            assert_eq!(event.cid, "QmYwMXeEc3Z64vqcPXx8p8Y8Y5tE9Y5sYW42FZ1U87Y");
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
                    key: b"assigned-workers".to_vec(),
                    value: b"worker1,worker2".to_vec(),
                },
                EventAttribute {
                    index: true,
                    key: b"assigned-workers".to_vec(),
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
}
