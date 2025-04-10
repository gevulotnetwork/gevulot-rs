/*!
 * # Task Builder Types
 *
 * This module provides builders for creating task-related messages in the Gevulot network.
 * These include messages for creating, managing, and reporting on computational tasks.
 */
use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self, InputContext, Label, OutputContext, TaskEnv},
};

use super::common::ByteSize;

/// Builder for constructing task creation messages for the Gevulot blockchain.
///
/// This struct represents all the parameters needed to create a new computational task
/// in the Gevulot network. It provides sensible defaults for optional fields while
/// requiring essential information like the creator identity and container image.
///
/// # Fields
///
/// * `creator` - Identity of the account creating the task
/// * `image` - Container image to execute
/// * `command` - Optional command to override the container entrypoint
/// * `args` - Optional arguments to pass to the command
/// * `env` - Environment variables to set in the container
/// * `input_contexts` - Data sources to mount into the container
/// * `output_contexts` - Output paths to capture from the container
/// * `cpus` - CPU cores required (default: 1000 millicores)
/// * `gpus` - GPU units required (default: 0)
/// * `memory` - Memory required (default: 1024 MB)
/// * `time` - Time limit in seconds (default: 3600)
/// * `store_stdout` - Whether to capture standard output (default: true)
/// * `store_stderr` - Whether to capture standard error (default: true)
/// * `labels` - Key-value pairs for metadata and filtering
/// * `tags` - Simple string tags for categorization
///
/// # Examples
///
/// ## Creating a basic task
///
/// ```
/// use gevulot_rs::builders::{MsgCreateTaskBuilder, ByteSize, ByteUnit};
///
/// let msg = MsgCreateTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .image("ubuntu:latest".to_string())
///     .command(vec!["echo".to_string(), "hello".to_string()])
///     .build()
///     .unwrap();
/// ```
///
/// ## Task with custom resource requirements
///
/// ```
/// use gevulot_rs::builders::{MsgCreateTaskBuilder, ByteSize, ByteUnit};
///
/// let msg = MsgCreateTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .image("ml-training:v1".to_string())
///     .cpus(4000) // 4 cores
///     .gpus(1000) // 1 GPU
///     .memory(ByteSize::new(8, ByteUnit::Gigabyte))
///     .time(7200) // 2 hours
///     .build()
///     .unwrap();
/// ```
///
/// ## Task with input and output contexts
///
/// ```
/// use gevulot_rs::builders::MsgCreateTaskBuilder;
/// use std::collections::HashMap;
///
/// let mut input_contexts = HashMap::new();
/// input_contexts.insert("bafyabcdef123".to_string(), "/data".to_string());
///
/// let msg = MsgCreateTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .image("data-processor:v1".to_string())
///     .input_contexts(input_contexts)
///     .output_contexts(vec![("/results".to_string(), 86400)])
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgCreateTask {
    /// Identity of the account creating the task
    /// This must be a valid Gevulot account address
    pub creator: String,
    
    /// Container image to execute (Docker-compatible format)
    /// This defines the execution environment for the task
    pub image: String,
    
    /// Optional command to override the container entrypoint
    /// If not specified, the default entrypoint of the image is used
    #[builder(default = "Vec::new()")]
    pub command: Vec<String>,
    
    /// Optional arguments for the command
    /// These are passed to either the command or the image entrypoint
    #[builder(default = "Vec::new()")]
    pub args: Vec<String>,
    
    /// Environment variables to set in container
    /// Used to configure the application's behavior
    #[builder(default = "std::collections::HashMap::new()")]
    pub env: std::collections::HashMap<String, String>,
    
    /// Input data contexts to mount into the container
    /// Maps source CIDs or pins to target paths in the container
    #[builder(default = "std::collections::HashMap::new()")]
    pub input_contexts: std::collections::HashMap<String, String>,
    
    /// Output data contexts to capture from the container
    /// Each entry specifies a source path and retention period in seconds
    #[builder(default = "Vec::new()")]
    pub output_contexts: Vec<(String, u64)>,
    
    /// CPU cores required in millicores (1000 = 1 CPU)
    /// Default is 1 CPU core
    #[builder(default = "1000")]
    pub cpus: u64,
    
    /// GPU units required in milli-GPU units (1000 = 1 GPU)
    /// Default is 0 (no GPU)
    #[builder(default = "0")]
    pub gpus: u64,
    
    /// Memory required for the task
    /// Default is 1024 MB
    #[builder(default = "ByteSize::new(1024, super::common::ByteUnit::Megabyte)")]
    pub memory: ByteSize,
    
    /// Time limit in seconds
    /// Default is 3600 seconds (1 hour)
    #[builder(default = "3600")]
    pub time: u64,
    
    /// Whether to capture and store standard output
    /// When true, stdout is saved in the task status
    #[builder(default = "true")]
    pub store_stdout: bool,
    
    /// Whether to capture and store standard error
    /// When true, stderr is saved in the task status
    #[builder(default = "true")]
    pub store_stderr: bool,
    
    /// Key-value pairs for task metadata and filtering
    /// These can be used to categorize and search for tasks
    #[builder(default = "std::collections::HashMap::new()")]
    pub labels: std::collections::HashMap<String, String>,
    
    /// Simple string tags for task categorization
    /// These provide a simpler alternative to labels for basic filtering
    #[builder(default = "Vec::new()")]
    pub tags: Vec<String>,
}

impl MsgCreateTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgCreateTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgCreateTaskBuilder;
    ///
    /// let proto_msg = MsgCreateTaskBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .image("ubuntu:latest".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
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

/// Builder for creating task acceptance messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed when a worker accepts a task
/// assignment. Acceptance indicates that the worker has verified it can execute
/// the task and has begun the execution process.
///
/// # Fields
///
/// * `creator` - Identity of the account sending the acceptance
/// * `task_id` - Unique identifier of the task being accepted
/// * `worker_id` - Identifier of the worker accepting the task
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgAcceptTaskBuilder;
///
/// let msg = MsgAcceptTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .task_id("task-123456".to_string())
///     .worker_id("worker-789012".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgAcceptTask {
    /// Identity of the account sending the acceptance
    /// This should match the worker's registered owner
    pub creator: String,
    
    /// Unique identifier of the task being accepted
    /// This is the blockchain-assigned ID for the task
    pub task_id: String,
    
    /// Identifier of the worker accepting the task
    /// This is the blockchain-assigned ID for the worker
    pub worker_id: String,
}

impl MsgAcceptTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgAcceptTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgAcceptTaskBuilder;
    ///
    /// let proto_msg = MsgAcceptTaskBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .task_id("task-123456".to_string())
    ///     .worker_id("worker-789012".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
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

/// Builder for creating task decline messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed when a worker declines a task
/// assignment. Declining indicates that the worker is unable to execute the task
/// for some reason, and the task should be reassigned to another worker.
///
/// # Fields
///
/// * `creator` - Identity of the account sending the decline message
/// * `task_id` - Unique identifier of the task being declined
/// * `worker_id` - Identifier of the worker declining the task
/// * `error` - Optional reason why the task was declined
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgDeclineTaskBuilder;
///
/// let proto_msg = MsgDeclineTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .task_id("task-123456".to_string())
///     .worker_id("worker-789012".to_string())
///     .error(Some("Required input context is not available".to_string()))
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
#[derive(Builder)]
pub struct MsgDeclineTask {
    /// Identity of the account sending the decline message
    /// This should match the worker's registered owner
    pub creator: String,
    
    /// Unique identifier of the task being declined
    /// This is the blockchain-assigned ID for the task
    pub task_id: String,
    
    /// Identifier of the worker declining the task
    /// This is the blockchain-assigned ID for the worker
    pub worker_id: String,
    
    /// Optional reason why the task was declined
    /// Provides context about why the worker cannot execute the task
    pub error: Option<String>,
}

impl MsgDeclineTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgDeclineTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgDeclineTaskBuilder;
    ///
    /// let proto_msg = MsgDeclineTaskBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .task_id("task-123456".to_string())
    ///     .worker_id("worker-789012".to_string())
    ///     .error(Some("Required input context is not available".to_string()))
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgDeclineTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeclineTask {
            creator: msg.creator,
            task_id: msg.task_id,
            worker_id: msg.worker_id,
            error: msg.error.unwrap_or_default(),
        })
    }
}

/// Builder for creating task completion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed when a worker reports the completion
/// of a task execution. It includes the execution results and any output data produced.
///
/// # Fields
///
/// * `creator` - Identity of the account reporting task completion
/// * `task_id` - Unique identifier of the completed task
/// * `exit_code` - Exit code from the task execution (0 typically indicates success)
/// * `stdout` - Optional captured standard output from the task
/// * `stderr` - Optional captured standard error from the task
/// * `output_contexts` - Optional CIDs of any output data generated by the task
/// * `error` - Optional error message if the task failed abnormally
///
/// # Examples
///
/// ## Reporting successful task completion
///
/// ```
/// use gevulot_rs::builders::MsgFinishTaskBuilder;
///
/// let proto_msg = MsgFinishTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .task_id("task-123456".to_string())
///     .exit_code(0)
///     .stdout(Some("Computation completed successfully".to_string()))
///     .stderr(None)
///     .output_contexts(Some(vec!["bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()]))
///     .error(None)
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
///
/// ## Reporting a task failure
///
/// ```
/// use gevulot_rs::builders::MsgFinishTaskBuilder;
///
/// let proto_msg = MsgFinishTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .task_id("task-123456".to_string())
///     .exit_code(1)
///     .stdout(Some("".to_string()))
///     .stderr(Some("Error: Out of memory during computation".to_string()))
///     .output_contexts(None)
///     .error(Some("Task failed due to resource constraints".to_string()))
///     .into_message()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgFinishTask {
    /// Identity of the account reporting task completion
    /// This should match the worker's registered owner
    pub creator: String,
    
    /// Unique identifier of the completed task
    /// This is the blockchain-assigned ID for the task
    pub task_id: String,
    
    /// Exit code from the task execution
    /// 0 typically indicates success, non-zero values indicate various error conditions
    pub exit_code: i32,
    
    /// Optional captured standard output from the task
    /// Contains the text output written to stdout during execution
    pub stdout: Option<String>,
    
    /// Optional captured standard error from the task
    /// Contains the text output written to stderr during execution
    pub stderr: Option<String>,
    
    /// Optional CIDs of any output data generated by the task
    /// These are the content identifiers for files produced by the task
    pub output_contexts: Option<Vec<String>>,
    
    /// Optional error message if the task failed abnormally
    /// Provides additional context about the failure beyond the exit code
    pub error: Option<String>,
}

impl MsgFinishTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgFinishTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgFinishTaskBuilder;
    ///
    /// let proto_msg = MsgFinishTaskBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .task_id("task-123456".to_string())
    ///     .exit_code(0)
    ///     .stdout(Some("Task completed successfully".to_string()))
    ///     .stderr(None)
    ///     .output_contexts(Some(vec!["bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()]))
    ///     .error(None)
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgFinishTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgFinishTask {
            creator: msg.creator,
            task_id: msg.task_id,
            exit_code: msg.exit_code,
            stdout: msg.stdout.unwrap_or_default(),
            stderr: msg.stderr.unwrap_or_default(),
            output_contexts: msg.output_contexts.unwrap_or_default(),
            error: msg.error.unwrap_or_default(),
        })
    }
}

/// Builder for requesting task rescheduling in the Gevulot blockchain.
///
/// This struct represents the parameters needed to request that a task be rescheduled
/// for execution, typically after it has failed or been declined by previous workers.
///
/// # Fields
///
/// * `creator` - Identity of the account requesting rescheduling
/// * `task_id` - Unique identifier of the task to reschedule
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgRescheduleTaskBuilder;
///
/// let proto_msg = MsgRescheduleTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .task_id("task-123456".to_string())
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
#[derive(Builder)]
pub struct MsgRescheduleTask {
    /// Identity of the account requesting rescheduling
    /// This should typically be the original task creator or an admin
    pub creator: String,
    
    /// Unique identifier of the task to reschedule
    /// This is the blockchain-assigned ID for the task
    pub task_id: String,
}

impl MsgRescheduleTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgRescheduleTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgRescheduleTaskBuilder;
    ///
    /// let proto_msg = MsgRescheduleTaskBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .task_id("task-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgRescheduleTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgRescheduleTask {
            creator: msg.creator,
            id: msg.task_id,
        })
    }
}

/// Builder for creating task deletion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to request deletion of a previously
/// created task in the Gevulot network. Only the original creator can delete a task
/// before its completion, or after it has completed or failed.
///
/// # Fields
///
/// * `creator` - Identity of the account requesting deletion
/// * `id` - Unique identifier of the task to delete
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgDeleteTaskBuilder;
///
/// let proto_msg = MsgDeleteTaskBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .id("task-123456".to_string())
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
#[derive(Builder)]
pub struct MsgDeleteTask {
    /// Identity of the account requesting task deletion
    /// This must match the original task creator or be an admin account
    pub creator: String,
    
    /// Unique identifier of the task to delete
    /// This is the blockchain-assigned ID for the task
    pub id: String,
}

impl MsgDeleteTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgDeleteTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgDeleteTaskBuilder;
    ///
    /// let proto_msg = MsgDeleteTaskBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .id("task-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgDeleteTask> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeleteTask {
            creator: msg.creator,
            id: msg.id,
        })
    }
} 