use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::Result,
    proto::gevulot::gevulot::{
        MsgSudoDeletePin, MsgSudoDeletePinResponse, MsgSudoDeleteTask, MsgSudoDeleteTaskResponse,
        MsgSudoDeleteWorker, MsgSudoDeleteWorkerResponse, MsgSudoFreezeAccount,
        MsgSudoFreezeAccountResponse,
    },
};

/// Client for managing sudo operations in the Gevulot system.
#[derive(Debug, Clone)]
pub struct SudoClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl SudoClient {
    /// Creates a new instance of SudoClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient.
    ///
    /// # Returns
    ///
    /// A new instance of SudoClient.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Deletes a pin.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the pin to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response of the delete pin operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete_pin(&mut self, msg: MsgSudoDeletePin) -> Result<MsgSudoDeletePinResponse> {
        let resp: MsgSudoDeletePinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a pin asynchronously.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the pin to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the hash of the delete pin operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete_pin_async(&mut self, msg: MsgSudoDeletePin) -> Result<String> {
        let resp = self
            .base_client
            .write()
            .await
            .send_msg(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the worker to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response of the delete worker operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete_worker(
        &mut self,
        msg: MsgSudoDeleteWorker,
    ) -> Result<MsgSudoDeleteWorkerResponse> {
        let resp: MsgSudoDeleteWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a worker asynchronously.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the worker to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the hash of the delete worker operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete_worker_async(&mut self, msg: MsgSudoDeleteWorker) -> Result<String> {
        let resp = self
            .base_client
            .write()
            .await
            .send_msg(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a task.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the task to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response of the delete task operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete_task(
        &mut self,
        msg: MsgSudoDeleteTask,
    ) -> Result<MsgSudoDeleteTaskResponse> {
        let resp: MsgSudoDeleteTaskResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a task asynchronously.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the task to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the hash of the delete task operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete_task_async(&mut self, msg: MsgSudoDeleteTask) -> Result<String> {
        let resp = self
            .base_client
            .write()
            .await
            .send_msg(msg, "")
            .await?;
        Ok(resp)
    }


    /// Freezes an account.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the account to freeze.
    ///
    /// # Returns
    ///
    /// A Result containing the response of the freeze account operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn freeze_account(
        &mut self,
        msg: MsgSudoFreezeAccount,
    ) -> Result<MsgSudoFreezeAccountResponse> {
        let resp: MsgSudoFreezeAccountResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Freezes an account asynchronously.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the account to freeze.
    ///
    /// # Returns
    ///
    /// A Result containing the hash of the freeze account operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn freeze_account_async(&mut self, msg: MsgSudoFreezeAccount) -> Result<String> {
        let resp = self
            .base_client
            .write()
            .await
            .send_msg(msg, "")
            .await?;
        Ok(resp)
    }
    
}
