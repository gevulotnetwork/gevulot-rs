use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::{Error, Result},
    proto::gevulot::gevulot::{
        MsgCreateWorkflow, MsgCreateWorkflowResponse, MsgDeleteWorkflow, MsgDeleteWorkflowResponse,
    },
};

/// Client for managing workflows in the Gevulot system.
pub struct WorkflowClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl WorkflowClient {
    /// Creates a new instance of WorkflowClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient.
    ///
    /// # Returns
    ///
    /// A new instance of WorkflowClient.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all workflows.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of workflows or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn list(&mut self) -> Result<Vec<crate::proto::gevulot::gevulot::Workflow>> {
        let request = crate::proto::gevulot::gevulot::QueryAllWorkflowRequest { pagination: None };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .workflow_all(request)
            .await?;
        Ok(response.into_inner().workflow)
    }

    /// Gets a workflow by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the workflow to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the workflow or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the workflow is not found or if the request to the Gevulot client fails.
    pub async fn get(&mut self, id: &str) -> Result<crate::proto::gevulot::gevulot::Workflow> {
        let request = crate::proto::gevulot::gevulot::QueryGetWorkflowRequest { id: id.to_owned() };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .workflow(request)
            .await?;
        response.into_inner().workflow.ok_or(Error::NotFound)
    }

    /// Creates a new workflow.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the workflow details.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn create(&mut self, msg: MsgCreateWorkflow) -> Result<MsgCreateWorkflowResponse> {
        let resp: MsgCreateWorkflowResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a workflow.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the workflow ID to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete(&mut self, msg: MsgDeleteWorkflow) -> Result<MsgDeleteWorkflowResponse> {
        let resp: MsgDeleteWorkflowResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
