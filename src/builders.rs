use std::collections::HashMap;

use crate::proto::gevulot::gevulot::{MsgCreatePin, MsgCreateTask, MsgCreateWorker, MsgDeletePin};

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
        Self { value: value.0, unit: value.1 }
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

    pub fn input_contexts(mut self, input_contexts: std::collections::HashMap<String, String>) -> Self {
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

pub struct DeletePinBuilder {
    creator: String,
    cid: String,
}

impl DeletePinBuilder {
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