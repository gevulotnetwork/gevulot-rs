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
#[derive(Debug, Clone)]
pub struct TaskClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl TaskClient {
    /// Creates a new instance of TaskClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient.
    ///
    /// # Returns
    ///
    /// A new instance of TaskClient.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all tasks.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of tasks or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
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
    /// # Arguments
    ///
    /// * `id` - The ID of the task to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the task or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the task is not found or if the request to the Gevulot client fails.
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

    /// Creates a new task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the task details.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn create(&mut self, msg: MsgCreateTask) -> Result<MsgCreateTaskResponse> {
        let resp: MsgCreateTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the task ID to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete(&mut self, msg: MsgDeleteTask) -> Result<MsgDeleteTaskResponse> {
        let resp: MsgDeleteTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Accepts a task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the task ID to accept.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn accept(&mut self, msg: MsgAcceptTask) -> Result<MsgAcceptTaskResponse> {
        let resp: MsgAcceptTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Declines a task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the task ID to decline.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn decline(&mut self, msg: MsgDeclineTask) -> Result<MsgDeclineTaskResponse> {
        let resp: MsgDeclineTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Finishes a task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the task ID to finish.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn finish(&mut self, msg: MsgFinishTask) -> Result<MsgFinishTaskResponse> {
        let resp: MsgFinishTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Reschedules a task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the task ID to reschedule.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    pub async fn reschedule(&mut self, msg: MsgRescheduleTask) -> Result<MsgRescheduleTaskResponse> {
        let resp: MsgRescheduleTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
