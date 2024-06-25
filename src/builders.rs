use std::collections::HashMap;

use crate::proto::gevulot::gevulot::{
    MsgCreatePin, MsgCreateTask, MsgCreateWorker,
    MsgDeletePin, MsgDeleteTask, MsgDeleteWorker,
    MsgAckPin, MsgAnnounceWorkerExit,
    MsgAcceptTask, MsgDeclineTask, MsgFinishTask,
};

pub enum ByteUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl ByteUnit {
    fn to_bytes(&self, value: u64) -> u64 {
        match self {
            ByteUnit::Byte => value,
            ByteUnit::Kilobyte => value * 1024,
            ByteUnit::Megabyte => value * 1024 * 1024,
            ByteUnit::Gigabyte => value * 1024 * 1024 * 1024,
        }
    }
}

impl From<(u64, ByteUnit)> for ByteSize {
    fn from(value: (u64, ByteUnit)) -> Self {
        Self {
            value: value.0,
            unit: value.1,
        }
    }
}

pub struct ByteSize {
    value: u64,
    unit: ByteUnit,
}

impl ByteSize {
    pub fn new(value: u64, unit: ByteUnit) -> Self {
        Self { value, unit }
    }

    pub fn to_bytes(&self) -> u64 {
        self.unit.to_bytes(self.value)
    }
}

impl std::fmt::Display for ByteSize {
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

pub struct MsgCreateTaskBuilder {
    creator: String,
    image: String,
    command: String,
    args: Vec<String>,
    env: std::collections::HashMap<String, String>,
    input_contexts: std::collections::HashMap<String, String>,
    output_contexts: Vec<String>,
    cpus: u64,
    gpus: u64,
    memory: u64,
    time: u64,
    storage: u64,
    store_stdout: bool,
    store_stderr: bool,
}

impl Default for MsgCreateTaskBuilder {
    fn default() -> Self {
        Self::new()
            .image("ubuntu:latest")
            .command("bash")
            .args(vec!["-c", "echo 'Hello, World!'"])
            .cpus(1000)
            .memory(ByteSize::new(1, ByteUnit::Gigabyte))
            .storage(ByteSize::new(1, ByteUnit::Gigabyte))
            .store_stdout(true)
            .time(10)
    }
}

impl MsgCreateTaskBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            image: String::new(),
            command: String::new(),
            args: Vec::new(),
            env: std::collections::HashMap::new(),
            input_contexts: std::collections::HashMap::new(),
            output_contexts: Vec::new(),
            cpus: 0,
            gpus: 0,
            memory: 0,
            time: 0,
            storage: 0,
            store_stdout: false,
            store_stderr: false,
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn image(mut self, image: &str) -> Self {
        self.image = image.to_string();
        self
    }

    pub fn command(mut self, command: &str) -> Self {
        self.command = command.to_string();
        self
    }

    pub fn args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|arg| arg.to_string()).collect();
        self
    }

    pub fn env(mut self, env: std::collections::HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub fn input_contexts(
        mut self,
        input_contexts: std::collections::HashMap<String, String>,
    ) -> Self {
        self.input_contexts = input_contexts;
        self
    }

    pub fn output_contexts(mut self, output_contexts: Vec<&str>) -> Self {
        self.output_contexts = output_contexts.iter().map(|arg| arg.to_string()).collect();
        self
    }

    pub fn cpus(mut self, cpus: u64) -> Self {
        self.cpus = cpus;
        self
    }

    pub fn gpus(mut self, gpus: u64) -> Self {
        self.gpus = gpus;
        self
    }

    pub fn memory(mut self, memory: ByteSize) -> Self {
        self.memory = memory.to_bytes();
        self
    }

    pub fn time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }

    pub fn storage(mut self, storage: ByteSize) -> Self {
        self.storage = storage.to_bytes();
        self
    }

    pub fn store_stdout(mut self, store_stdout: bool) -> Self {
        self.store_stdout = store_stdout;
        self
    }

    pub fn store_stderr(mut self, store_stderr: bool) -> Self {
        self.store_stderr = store_stderr;
        self
    }

    pub fn build(self) -> MsgCreateTask {
        MsgCreateTask {
            creator: self.creator,
            image: self.image,
            command: self.command,
            args: self.args,
            env: self.env,
            input_contexts: self.input_contexts,
            output_contexts: self.output_contexts,
            cpus: self.cpus,
            gpus: self.gpus,
            memory: self.memory,
            time: self.time,
            storage: self.storage,
            store_stdout: self.store_stdout,
            store_stderr: self.store_stderr,
        }
    }
}

pub struct MsgCreatePinBuilder {
    creator: String,
    cid: String,
    bytes: u64,
    name: String,
    redundancy: u64,
    time: u64,
    description: String,
    fallback_urls: Vec<String>,
    tags: Vec<String>,
    labels: HashMap<String, String>,
}

impl MsgCreatePinBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            cid: String::new(),
            bytes: 0,
            name: String::new(),
            redundancy: 0,
            time: 0,
            description: String::new(),
            fallback_urls: Vec::new(),
            tags: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn cid(mut self, cid: &str) -> Self {
        self.cid = cid.to_string();
        self
    }

    pub fn bytes(mut self, bytes: ByteSize) -> Self {
        self.bytes = bytes.to_bytes();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn redundancy(mut self, redundancy: u64) -> Self {
        self.redundancy = redundancy;
        self
    }

    pub fn time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn fallback_urls(mut self, fallback_urls: Vec<&str>) -> Self {
        self.fallback_urls = fallback_urls.iter().map(|arg| arg.to_string()).collect();
        self
    }

    pub fn tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|arg| arg.to_string()).collect();
        self
    }

    pub fn labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn build(self) -> MsgCreatePin {
        MsgCreatePin {
            creator: self.creator,
            cid: self.cid,
            bytes: self.bytes,
            name: self.name,
            redundancy: self.redundancy,
            time: self.time,
            description: self.description,
            fallback_urls: self.fallback_urls,
            tags: self.tags,
            labels: self.labels,
        }
    }
}

pub struct MsgCreateWorkerBuilder {
    creator: String,
    name: String,
    description: String,
    cpus: u64,
    gpus: u64,
    memory: u64,
    disk: u64,
}

impl MsgCreateWorkerBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            name: String::new(),
            description: String::new(),
            cpus: 0,
            gpus: 0,
            memory: 0,
            disk: 0,
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn cpus(mut self, cpus: u64) -> Self {
        self.cpus = cpus;
        self
    }

    pub fn gpus(mut self, gpus: u64) -> Self {
        self.gpus = gpus;
        self
    }

    pub fn memory(mut self, memory: ByteSize) -> Self {
        self.memory = memory.to_bytes();
        self
    }

    pub fn disk(mut self, disk: ByteSize) -> Self {
        self.disk = disk.to_bytes();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn build(self) -> MsgCreateWorker {
        MsgCreateWorker {
            creator: self.creator,
            name: self.name,
            description: self.description,
            cpus: self.cpus,
            gpus: self.gpus,
            memory: self.memory,
            disk: self.disk,
        }
    }
}

pub struct MsgDeletePinBuilder {
    creator: String,
    cid: String,
}

impl MsgDeletePinBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            cid: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn cid(mut self, cid: &str) -> Self {
        self.cid = cid.to_string();
        self
    }

    pub fn build(self) -> MsgDeletePin {
        MsgDeletePin {
            creator: self.creator,
            cid: self.cid,
        }
    }
}

pub struct MsgDeleteTaskBuilder {
    creator: String,
    id: String,
}

impl MsgDeleteTaskBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            id: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn build(self) -> MsgDeleteTask {
        MsgDeleteTask {
            creator: self.creator,
            id: self.id,
        }
    }
}

pub struct MsgDeleteWorkerBuilder {
    creator: String,
    id: String,
}

impl MsgDeleteWorkerBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            id: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn build(self) -> MsgDeleteWorker {
        MsgDeleteWorker {
            creator: self.creator,
            id: self.id,
        }
    }
}

pub struct MsgAckPinBuilder {
    creator: String,
    cid: String,
    worker_id: String,
}

impl MsgAckPinBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            cid: String::new(),
            worker_id: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn cid(mut self, cid: &str) -> Self {
        self.cid = cid.to_string();
        self
    }

    pub fn worker_id(mut self, worker_id: &str) -> Self {
        self.worker_id = worker_id.to_string();
        self
    }

    pub fn build(self) -> MsgAckPin {
        MsgAckPin {
            creator: self.creator,
            cid: self.cid,
            worker_id: self.worker_id,
        }
    }
}

pub struct MsgAnnounceWorkerExitBuilder {
    creator: String,
    worker_id: String,
}

impl MsgAnnounceWorkerExitBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            worker_id: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn worker_id(mut self, worker_id: &str) -> Self {
        self.worker_id = worker_id.to_string();
        self
    }

    pub fn build(self) -> MsgAnnounceWorkerExit {
        MsgAnnounceWorkerExit {
            creator: self.creator,
            worker_id: self.worker_id,
        }
    }
}
pub struct MsgAcceptTaskBuilder {
    creator: String,
    task_id: String,
    worker_id: String,
}

impl MsgAcceptTaskBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            task_id: String::new(),
            worker_id: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn task_id(mut self, task_id: &str) -> Self {
        self.task_id = task_id.to_string();
        self
    }

    pub fn worker_id(mut self, worker_id: &str) -> Self {
        self.worker_id = worker_id.to_string();
        self
    }

    pub fn build(self) -> MsgAcceptTask {
        MsgAcceptTask {
            creator: self.creator,
            task_id: self.task_id,
            worker_id: self.worker_id,
        }
    }
}

pub struct MsgDeclineTaskBuilder {
    creator: String,
    task_id: String,
    worker_id: String,
}

impl MsgDeclineTaskBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            task_id: String::new(),
            worker_id: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn task_id(mut self, task_id: &str) -> Self {
        self.task_id = task_id.to_string();
        self
    }

    pub fn worker_id(mut self, worker_id: &str) -> Self {
        self.worker_id = worker_id.to_string();
        self
    }

    pub fn build(self) -> MsgDeclineTask {
        MsgDeclineTask {
            creator: self.creator,
            task_id: self.task_id,
            worker_id: self.worker_id,
        }
    }
}

pub struct MsgFinishTaskBuilder {
    creator: String,
    task_id: String,
    exit_code: i32,
    stdout: String,
    stderr: String,
    output_contexts: Vec<String>,
    error: String,
}

impl MsgFinishTaskBuilder {
    pub fn new() -> Self {
        Self {
            creator: String::new(),
            task_id: String::new(),
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
            output_contexts: Vec::new(),
            error: String::new(),
        }
    }

    pub fn creator(mut self, creator: &str) -> Self {
        self.creator = creator.to_string();
        self
    }

    pub fn task_id(mut self, task_id: &str) -> Self {
        self.task_id = task_id.to_string();
        self
    }

    pub fn exit_code(mut self, exit_code: i32) -> Self {
        self.exit_code = exit_code;
        self
    }

    pub fn stdout(mut self, stdout: &str) -> Self {
        self.stdout = stdout.to_string();
        self
    }

    pub fn stderr(mut self, stderr: &str) -> Self {
        self.stderr = stderr.to_string();
        self
    }

    pub fn output_contexts(mut self, output_contexts: Vec<String>) -> Self {
        self.output_contexts = output_contexts;
        self
    }

    pub fn error(mut self, error: &str) -> Self {
        self.error = error.to_string();
        self
    }

    pub fn build(self) -> MsgFinishTask {
        MsgFinishTask {
            creator: self.creator,
            task_id: self.task_id,
            exit_code: self.exit_code,
            stdout: self.stdout,
            stderr: self.stderr,
            output_contexts: self.output_contexts,
            error: self.error,
        }
    }
}