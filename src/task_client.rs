use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::{Error, Result},
    proto::gevulot::gevulot::{
        MsgAcceptTask, MsgAcceptTaskResponse, MsgCreateTask, MsgCreateTaskResponse, MsgDeclineTask,
        MsgDeclineTaskResponse, MsgDeleteTask, MsgDeleteTaskResponse, MsgFinishTask,
        MsgFinishTaskResponse, MsgRescheduleTask, MsgRescheduleTaskResponse,
    },
};

/// Client for managing tasks in the Gevulot system.
///
/// TaskClient provides a high-level interface for interacting with the task management
/// functionality of the Gevulot blockchain. It allows clients to create, query, and
/// manage computational tasks across the network.
///
/// # Task Lifecycle
///
/// 1. Tasks are created with specific requirements and parameters
/// 2. The system assigns tasks to suitable workers
/// 3. Workers accept, decline, or complete tasks
/// 4. Task creators can manage, reschedule, or delete their tasks
///
/// # Examples
///
/// ## Creating a client
///
/// ```no_run
/// use std::sync::Arc;
/// use tokio::sync::RwLock;
/// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, task_client::TaskClient};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a connection to the Gevulot node
///     let base_client = Arc::new(RwLock::new(
///         BaseClient::new(
///             "http://localhost:9090",
///             FuelPolicy::Dynamic { 
///                 gas_price: 0.025, 
///                 gas_multiplier: 1.2 
///             }
///         ).await?
///     ));
///     
///     // Initialize the task client
///     let mut task_client = TaskClient::new(base_client);
///     
///     // Now ready to use task_client
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TaskClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl TaskClient {
    /// Creates a new instance of TaskClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient, which handles the
    ///   underlying communication with the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// A new instance of TaskClient.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, task_client::TaskClient};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Create a connection to the Gevulot node
    /// let base_client = Arc::new(RwLock::new(
    ///     BaseClient::new(
    ///         "http://localhost:9090",
    ///         FuelPolicy::Dynamic { 
    ///             gas_price: 0.025, 
    ///             gas_multiplier: 1.2 
    ///         }
    ///     ).await?
    /// ));
    /// 
    /// // Initialize the task client
    /// let task_client = TaskClient::new(base_client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all tasks in the Gevulot network.
    ///
    /// Retrieves a complete list of all tasks currently in the system, 
    /// regardless of their status or ownership.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Task>>` - A vector containing all tasks in the system, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The connection to the Gevulot blockchain fails
    /// - The request times out
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, task_client::TaskClient};
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Get all tasks in the system
    ///     let tasks = task_client.list().await?;
    ///     
    ///     // Display task information
    ///     for task in tasks {
    ///         if let Some(metadata) = &task.metadata {
    ///             println!("Task ID: {}, Status: {:?}", 
    ///                 &metadata.id, 
    ///                 &task.status);
    ///         }
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(&mut self) -> Result<Vec<crate::proto::gevulot::gevulot::Task>> {
        let request = crate::proto::gevulot::gevulot::QueryAllTaskRequest { pagination: None };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .task_all(request)
            .await?;
        Ok(response.into_inner().task)
    }

    /// Gets a task by its ID.
    ///
    /// Retrieves detailed information about a specific task, including its
    /// current status, resource requirements, and execution details.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to retrieve
    ///
    /// # Returns
    ///
    /// * `Result<Task>` - The requested task details on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified task does not exist
    /// - The connection to the Gevulot blockchain fails
    /// - The request times out
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{base_client::{BaseClient, FuelPolicy}, task_client::TaskClient};
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Retrieve a specific task by its ID
    ///     let task_id = "task-123456";
    ///     match task_client.get(task_id).await {
    ///         Ok(task) => {
    ///             println!("Task found:");
    ///             if let Some(metadata) = &task.metadata {
    ///                 let id = &metadata.id;
    ///                 let creator = &metadata.creator;
    ///                 println!("  ID: {}", id);
    ///                 println!("  Creator: {}", creator);
    ///             }
    ///             if let Some(spec) = &task.spec {
    ///                 println!("  Image: {}", spec.image);
    ///             }
    ///             println!("  Status: {:?}", task.status);
    ///         },
    ///         Err(e) => println!("Error retrieving task: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&mut self, id: &str) -> Result<crate::proto::gevulot::gevulot::Task> {
        let request = crate::proto::gevulot::gevulot::QueryGetTaskRequest { id: id.to_owned() };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .task(request)
            .await?;
        response.into_inner().task.ok_or(Error::NotFound)
    }

    /// Creates a new task in the Gevulot network.
    ///
    /// Submits a new computational task to be executed in the network. The task
    /// specifies its resource requirements, execution environment, and data contexts.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing all the task creation parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgCreateTaskResponse>` - Response containing the created task's ID on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required fields are missing or invalid
    /// - The creator account doesn't exist or lacks permissions
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ## Creating a basic task
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgCreateTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a task creation message
    ///     let msg = MsgCreateTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .image("ubuntu:latest".to_string())
    ///         .command(vec!["echo".to_string(), "hello".to_string()])
    ///         .into_message()?;
    ///     
    ///     // Submit the task
    ///     let response = task_client.create(msg).await?;
    ///     println!("Task created with ID: {}", response.id);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Creating a task with custom requirements
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::{MsgCreateTaskBuilder, ByteSize, ByteUnit},
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Create input contexts (data sources)
    ///     let mut input_contexts = HashMap::new();
    ///     input_contexts.insert("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string(), "/data".to_string());
    ///     
    ///     // Build a more complex task message
    ///     let msg = MsgCreateTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .image("ml-training:v1".to_string())
    ///         .command(vec!["python".to_string(), "train.py".to_string()])
    ///         .cpus(4000) // 4 cores
    ///         .gpus(1000) // 1 GPU
    ///         .memory(ByteSize::new(8, ByteUnit::Gigabyte))
    ///         .time(7200) // 2 hours
    ///         .input_contexts(input_contexts)
    ///         .output_contexts(vec![("/results".to_string(), 86400)]) // 1 day retention
    ///         .into_message()?;
    ///     
    ///     // Submit the task
    ///     let response = task_client.create(msg).await?;
    ///     println!("Task created with ID: {}", response.id);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&mut self, msg: MsgCreateTask) -> Result<MsgCreateTaskResponse> {
        let resp: MsgCreateTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a task from the Gevulot network.
    ///
    /// Removes a previously created task from the system. Only the task's creator
    /// can delete it. Tasks can be deleted in any state, but deleting a running
    /// task will attempt to terminate it.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the task deletion parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgDeleteTaskResponse>` - Response confirming deletion on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified task does not exist
    /// - The caller is not the task creator
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgDeleteTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a task deletion message
    ///     let msg = MsgDeleteTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .id("task-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Delete the task
    ///     match task_client.delete(msg).await {
    ///         Ok(_) => println!("Task successfully deleted"),
    ///         Err(e) => println!("Failed to delete task: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&mut self, msg: MsgDeleteTask) -> Result<MsgDeleteTaskResponse> {
        let resp: MsgDeleteTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Accepts a task assignment in the Gevulot network.
    ///
    /// Used by workers to signal acceptance of a task assignment. Accepting a task
    /// indicates that the worker has verified it can execute the task and has begun
    /// the execution process.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the task acceptance parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgAcceptTaskResponse>` - Response confirming acceptance on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified task does not exist
    /// - The task is not in a state where it can be accepted
    /// - The worker is not assigned to the task
    /// - The caller is not the worker's registered owner
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgAcceptTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a task acceptance message
    ///     let msg = MsgAcceptTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .task_id("task-123456".to_string())
    ///         .worker_id("worker-789012".to_string())
    ///         .into_message()?;
    ///     
    ///     // Accept the task
    ///     match task_client.accept(msg).await {
    ///         Ok(_) => println!("Task successfully accepted"),
    ///         Err(e) => println!("Failed to accept task: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn accept(&mut self, msg: MsgAcceptTask) -> Result<MsgAcceptTaskResponse> {
        let resp: MsgAcceptTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Declines a task assignment in the Gevulot network.
    ///
    /// Used by workers to signal that they cannot execute an assigned task. Declining a task
    /// allows the system to reassign it to another compatible worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the task decline parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgDeclineTaskResponse>` - Response confirming decline on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified task does not exist
    /// - The task is not in a state where it can be declined
    /// - The worker is not assigned to the task
    /// - The caller is not the worker's registered owner
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgDeclineTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a task decline message
    ///     let msg = MsgDeclineTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .task_id("task-123456".to_string())
    ///         .worker_id("worker-789012".to_string())
    ///         .error(Some("Required input context is not available".to_string()))
    ///         .into_message()?;
    ///     
    ///     // Decline the task
    ///     match task_client.decline(msg).await {
    ///         Ok(_) => println!("Task successfully declined"),
    ///         Err(e) => println!("Failed to decline task: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn decline(&mut self, msg: MsgDeclineTask) -> Result<MsgDeclineTaskResponse> {
        let resp: MsgDeclineTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Reports completion of a task in the Gevulot network.
    ///
    /// Used by workers to signal that a task has completed execution, providing
    /// the execution results and any output data produced.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the task completion details
    ///
    /// # Returns
    ///
    /// * `Result<MsgFinishTaskResponse>` - Response confirming completion on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified task does not exist
    /// - The task is not in a running state
    /// - The worker is not assigned to the task
    /// - The caller is not the worker's registered owner
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ## Reporting successful task completion
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgFinishTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a successful task completion message
    ///     let msg = MsgFinishTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .task_id("task-123456".to_string())
    ///         .exit_code(0)
    ///         .stdout(Some("Computation completed successfully".to_string()))
    ///         .stderr(None)
    ///         .output_contexts(Some(vec!["bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string()]))
    ///         .error(None)
    ///         .into_message()?;
    ///     
    ///     // Report task completion
    ///     match task_client.finish(msg).await {
    ///         Ok(_) => println!("Task completion reported successfully"),
    ///         Err(e) => println!("Failed to report task completion: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Reporting a task failure
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgFinishTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a task failure message
    ///     let msg = MsgFinishTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .task_id("task-123456".to_string())
    ///         .exit_code(1)
    ///         .stdout(Some("".to_string()))
    ///         .stderr(Some("Error: Out of memory during computation".to_string()))
    ///         .output_contexts(None)
    ///         .error(Some("Task failed due to resource constraints".to_string()))
    ///         .into_message()?;
    ///     
    ///     // Report task failure
    ///     match task_client.finish(msg).await {
    ///         Ok(_) => println!("Task failure reported successfully"),
    ///         Err(e) => println!("Failed to report task failure: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn finish(&mut self, msg: MsgFinishTask) -> Result<MsgFinishTaskResponse> {
        let resp: MsgFinishTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Requests rescheduling of a task in the Gevulot network.
    ///
    /// Used to request that a task be reassigned and executed again, typically
    /// after it has failed or been declined by previous workers.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the task rescheduling parameters
    ///
    /// # Returns
    ///
    /// * `Result<MsgRescheduleTaskResponse>` - Response confirming rescheduling on success, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified task does not exist
    /// - The task is not in a state where it can be rescheduled
    /// - The caller is not authorized to reschedule the task
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gevulot_rs::{
    ///     base_client::{BaseClient, FuelPolicy},
    ///     task_client::TaskClient,
    ///     builders::MsgRescheduleTaskBuilder,
    /// };
    /// use std::sync::Arc;
    /// use tokio::sync::RwLock;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Create a connection to the Gevulot node
    ///     let base_client = Arc::new(RwLock::new(
    ///         BaseClient::new(
    ///             "http://localhost:9090",
    ///             FuelPolicy::Dynamic { 
    ///                 gas_price: 0.025, 
    ///                 gas_multiplier: 1.2 
    ///             }
    ///         ).await?
    ///     ));
    ///     
    ///     let mut task_client = TaskClient::new(base_client);
    ///     
    ///     // Build a task rescheduling message
    ///     let msg = MsgRescheduleTaskBuilder::default()
    ///         .creator("gevulot1abcdef".to_string())
    ///         .task_id("task-123456".to_string())
    ///         .into_message()?;
    ///     
    ///     // Request task rescheduling
    ///     match task_client.reschedule(msg).await {
    ///         Ok(_) => println!("Task successfully rescheduled"),
    ///         Err(e) => println!("Failed to reschedule task: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn reschedule(
        &mut self,
        msg: MsgRescheduleTask,
    ) -> Result<MsgRescheduleTaskResponse> {
        let resp: MsgRescheduleTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
