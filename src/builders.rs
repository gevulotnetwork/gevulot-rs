use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self, InputContext, Label, OutputContext, TaskEnv},
};

/// Enum representing different units of bytes.
#[derive(Clone)]
pub enum ByteUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl ByteUnit {
    /// Converts a value in the given ByteUnit to bytes.
    fn to_bytes(&self, value: u64) -> u64 {
        match self {
            ByteUnit::Byte => value,
            ByteUnit::Kilobyte => value * 1024,
            ByteUnit::Megabyte => value * 1024 * 1024,
            ByteUnit::Gigabyte => value * 1024 * 1024 * 1024,
        }
    }
}

/// Struct representing a size in bytes with a specific unit.
#[derive(Clone)]
pub struct ByteSize {
    value: u64,
    unit: ByteUnit,
}

impl ByteSize {
    /// Creates a new ByteSize instance.
    pub fn new(value: u64, unit: ByteUnit) -> Self {
        Self { value, unit }
    }

    /// Converts the ByteSize to bytes.
    pub fn to_bytes(&self) -> u64 {
        self.unit.to_bytes(self.value)
    }
}

impl std::fmt::Display for ByteSize {
    /// Formats the ByteSize for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit_str = match self.unit {
            ByteUnit::Byte => "B",
            ByteUnit::Kilobyte => "KB",
            ByteUnit::Megabyte => "MB",
            ByteUnit::Gigabyte => "GB",
        };
        write!(f, "{} {}", self.value, unit_str)
    }
}

impl From<(u64, ByteUnit)> for ByteSize {
    /// Converts a tuple of (u64, ByteUnit) to a ByteSize.
    fn from(value: (u64, ByteUnit)) -> Self {
        Self {
            value: value.0,
            unit: value.1,
        }
    }
}

#[derive(Builder)]
pub struct MsgCreateTask {
    pub creator: String,
    pub image: String,
    #[builder(default = "Vec::new()")]
    pub command: Vec<String>,
    #[builder(default = "Vec::new()")]
    pub args: Vec<String>,
    #[builder(default = "std::collections::HashMap::new()")]
    pub env: std::collections::HashMap<String, String>,
    #[builder(default = "std::collections::HashMap::new()")]
    pub input_contexts: std::collections::HashMap<String, String>,
    #[builder(default = "Vec::new()")]
    pub output_contexts: Vec<(String, u64)>,
    #[builder(default = "1000")]
    pub cpus: u64,
    #[builder(default = "0")]
    pub gpus: u64,
    #[builder(default = "ByteSize::new(1024, ByteUnit::Megabyte)")]
    pub memory: ByteSize,
    #[builder(default = "3600")]
    pub time: u64,
    #[builder(default = "true")]
    pub store_stdout: bool,
    #[builder(default = "true")]
    pub store_stderr: bool,
    #[builder(default = "std::collections::HashMap::new()")]
    pub labels: std::collections::HashMap<String, String>,
    #[builder(default = "Vec::new()")]
    pub tags: Vec<String>,
}

impl MsgCreateTaskBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgCreateTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgCreateTask {
            creator: msg.creator,
            image: msg.image,
            command: msg.command,
            args: msg.args,
            env: msg
                .env
                .into_iter()
                .map(|(k, v)| TaskEnv { name: k, value: v })
                .collect(),
            input_contexts: msg
                .input_contexts
                .into_iter()
                .map(|(k, v)| InputContext {
                    source: k,
                    target: v,
                })
                .collect(),
            output_contexts: msg
                .output_contexts
                .into_iter()
                .map(|(source, retention_period)| OutputContext {
                    source,
                    retention_period,
                })
                .collect(),
            cpus: msg.cpus,
            gpus: msg.gpus,
            memory: msg.memory.to_bytes(),
            time: msg.time,
            store_stdout: msg.store_stdout,
            store_stderr: msg.store_stderr,
            tags: msg.tags,
            labels: msg
                .labels
                .into_iter()
                .map(|(k, v)| Label { key: k, value: v })
                .collect(),
        })
    }
}

#[derive(Builder)]
pub struct MsgCreatePin {
    pub creator: String,
    pub cid: String,
    pub bytes: ByteSize,
    pub name: String,
    pub redundancy: u64,
    pub time: u64,
    pub description: String,
    pub fallback_urls: Vec<String>,
    pub tags: Vec<String>,
    pub labels: Vec<Label>,
}

impl MsgCreatePinBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgCreatePin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgCreatePin {
            creator: msg.creator,
            cid: msg.cid,
            bytes: msg.bytes.to_bytes(),
            name: msg.name,
            redundancy: msg.redundancy,
            time: msg.time,
            description: msg.description,
            fallback_urls: msg.fallback_urls,
            tags: msg.tags,
            labels: msg.labels,
        })
    }
}

#[derive(Builder)]
pub struct MsgDeletePin {
    pub creator: String,
    pub cid: String,
    pub id: String,
}

impl MsgDeletePinBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgDeletePin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeletePin {
            creator: msg.creator,
            cid: msg.cid,
            id: msg.id,
        })
    }
}

#[derive(Builder)]
pub struct MsgCreateWorker {
    pub creator: String,
    pub name: String,
    pub description: String,
    pub cpus: u64,
    pub gpus: u64,
    pub memory: ByteSize,
    pub disk: ByteSize,
    pub labels: Vec<Label>,
    pub tags: Vec<String>,
}

impl MsgCreateWorkerBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgCreateWorker> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgCreateWorker {
            creator: msg.creator,
            name: msg.name,
            description: msg.description,
            cpus: msg.cpus,
            gpus: msg.gpus,
            memory: msg.memory.to_bytes(),
            disk: msg.disk.to_bytes(),
            labels: msg.labels,
            tags: msg.tags,
        })
    }
}

#[derive(Builder)]
pub struct MsgDeleteWorker {
    pub creator: String,
    pub id: String,
}

impl MsgDeleteWorkerBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgDeleteWorker> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeleteWorker {
            creator: msg.creator,
            id: msg.id,
        })
    }
}

#[derive(Builder)]
pub struct MsgAckPin {
    pub creator: String,
    pub cid: String,
    pub id: String,
    pub worker_id: String,
    pub success: bool,
}

impl MsgAckPinBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgAckPin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgAckPin {
            creator: msg.creator,
            cid: msg.cid,
            id: msg.id,
            worker_id: msg.worker_id,
            success: msg.success,
        })
    }
}

#[derive(Builder)]
pub struct MsgAnnounceWorkerExit {
    pub creator: String,
    pub worker_id: String,
}

impl MsgAnnounceWorkerExitBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgAnnounceWorkerExit> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgAnnounceWorkerExit {
            creator: msg.creator,
            worker_id: msg.worker_id,
        })
    }
}

#[derive(Builder)]
pub struct MsgAcceptTask {
    pub creator: String,
    pub task_id: String,
    pub worker_id: String,
}

impl MsgAcceptTaskBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgAcceptTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgAcceptTask {
            creator: msg.creator,
            task_id: msg.task_id,
            worker_id: msg.worker_id,
        })
    }
}

#[derive(Builder)]
pub struct MsgDeclineTask {
    pub creator: String,
    pub task_id: String,
    pub worker_id: String,
}

impl MsgDeclineTaskBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgDeclineTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeclineTask {
            creator: msg.creator,
            task_id: msg.task_id,
            worker_id: msg.worker_id,
        })
    }
}

#[derive(Builder)]
pub struct MsgFinishTask {
    pub creator: String,
    pub task_id: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub output_contexts: Vec<String>,
    pub error: String,
}

impl MsgFinishTaskBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgFinishTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgFinishTask {
            creator: msg.creator,
            task_id: msg.task_id,
            exit_code: msg.exit_code,
            stdout: msg.stdout,
            stderr: msg.stderr,
            output_contexts: msg.output_contexts,
            error: msg.error,
        })
    }
}

#[derive(Builder)]
pub struct MsgSudoDeletePin {
    pub authority: String,
    pub cid: String,
}

impl MsgSudoDeletePinBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgSudoDeletePin> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgSudoDeletePin {
            authority: msg.authority,
            cid: msg.cid,
        })
    }
}

#[derive(Builder)]
pub struct MsgSudoDeleteWorker {
    pub authority: String,
    pub id: String,
}

impl MsgSudoDeleteWorkerBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgSudoDeleteWorker> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgSudoDeleteWorker {
            authority: msg.authority,
            id: msg.id,
        })
    }
}

#[derive(Builder)]
pub struct MsgSudoDeleteTask {
    pub authority: String,
    pub id: String,
}

impl MsgSudoDeleteTaskBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgSudoDeleteTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgSudoDeleteTask {
            authority: msg.authority,
            id: msg.id,
        })
    }
}

#[derive(Builder)]
pub struct MsgSudoFreezeAccount {
    pub authority: String,
    pub account: String,
}

impl MsgSudoFreezeAccountBuilder {
    pub fn into_message(&self) -> Result<gevulot::MsgSudoFreezeAccount> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgSudoFreezeAccount {
            authority: msg.authority,
            account: msg.account,
        })
    }
}
