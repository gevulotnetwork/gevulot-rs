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
///
/// WorkflowClient provides a high-level interface for interacting with the workflow management
/// functionality of the Gevulot blockchain. It allows clients to create, query, and
/// manage computational workflows across the network.
///
/// # Workflow Lifecycle
///
/// 1. Workflows are created with defined stages and task specifications
/// 2. The system processes stages sequentially, with tasks in each stage potentially executing in parallel
/// 3. As tasks in a stage complete, the workflow advances to the next stage
/// 4. Workflow creators can monitor progress or delete workflows as needed
///
/// A workflow in Gevulot represents a computational pipeline consisting of multiple stages,
/// where each stage contains one or more tasks. The stages are executed sequentially,
/// while tasks within a stage can be executed in parallel. This structure enables
/// complex computational workflows with dependencies between stages.
#[derive(Debug, Clone)]
pub struct WorkflowClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl WorkflowClient {
    /// Creates a new instance of WorkflowClient.
    ///
    /// Initializes a new WorkflowClient with the provided BaseClient, which handles
    /// the underlying communication with the Gevulot blockchain. The BaseClient 
    /// should be configured with appropriate connection details and fuel policy
    /// before being passed to this constructor.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient, which handles the
    ///   underlying communication with the Gevulot blockchain. The BaseClient must be
    ///   properly initialized with a valid node URL and fuel policy.
    ///
    /// # Returns
    ///
    /// A new instance of WorkflowClient ready to interact with the Gevulot workflow system.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all workflows in the Gevulot network.
    ///
    /// Retrieves a complete list of all workflows currently in the system,
    /// regardless of their status or ownership. This function provides a comprehensive
    /// view of all computational workflows registered on the Gevulot blockchain.
    ///
    /// The returned workflows include their full specifications, metadata, and current status,
    /// allowing clients to monitor and analyze the state of workflows across the network.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Workflow>>` - A vector containing all workflows in the system, or an error.
    ///   Each workflow contains detailed information about its structure, stages, tasks, and
    ///   current execution state.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The connection to the Gevulot blockchain fails
    /// - The request times out
    /// - The response cannot be parsed
    /// - Authentication or authorization fails
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
    /// Retrieves detailed information about a specific workflow, including its
    /// current status, stages, and execution details. This function allows clients
    /// to monitor the progress and state of individual workflows.
    ///
    /// The returned workflow contains comprehensive information about its structure,
    /// including all stages, tasks, and their specifications. It also includes the
    /// current execution state, indicating which stage is active and the status of
    /// completed tasks.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the workflow to retrieve. This ID is assigned
    ///   by the system when the workflow is created and remains constant throughout
    ///   the workflow's lifecycle.
    ///
    /// # Returns
    ///
    /// * `Result<Workflow>` - The requested workflow details on success, or an error.
    ///   The workflow contains its complete specification, metadata, and current status.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified workflow does not exist (Error::NotFound)
    /// - The connection to the Gevulot blockchain fails
    /// - The request times out
    /// - The response cannot be parsed
    /// - Authentication or authorization fails
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

    /// Creates a new workflow in the Gevulot network.
    ///
    /// Submits a new computational workflow to be executed in the network. The workflow
    /// defines multiple stages that are executed sequentially, with each stage containing
    /// one or more tasks that can run in parallel.
    ///
    /// When creating a workflow, clients must provide:
    /// 1. Creator information (account address)
    /// 2. Workflow specification with stages and tasks
    /// 3. Resource requirements for each task
    /// 4. Input/output context definitions for data handling between stages
    ///
    /// The system assigns a unique ID to the workflow upon creation, which can be
    /// used for subsequent operations and monitoring. The workflow is initially in
    /// a "pending" state and transitions to "running" once execution begins.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing all the workflow creation parameters, including
    ///   the creator's address, workflow specification with stages and tasks, and
    ///   any additional metadata. This should be built using the MsgCreateWorkflowBuilder.
    ///
    /// # Returns
    ///
    /// * `Result<MsgCreateWorkflowResponse>` - Response containing the created workflow's ID
    ///   on success, or an error. The ID can be used for tracking and managing the workflow.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required fields are missing or invalid in the message
    /// - The creator account doesn't exist or lacks permissions
    /// - The workflow specification is malformed or contains invalid parameters
    /// - Resource requirements exceed system limits
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    /// - Transaction signing or broadcasting fails
    pub async fn create(&mut self, msg: MsgCreateWorkflow) -> Result<MsgCreateWorkflowResponse> {
        let resp: MsgCreateWorkflowResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a workflow from the Gevulot network.
    ///
    /// Removes a previously created workflow from the system. Only the workflow's creator
    /// can delete it. Workflows can be deleted in any state, but deleting a running
    /// workflow will attempt to terminate all its active tasks.
    ///
    /// When a workflow is deleted:
    /// 1. All active tasks are signaled for termination
    /// 2. Resources allocated to the workflow are released
    /// 3. Temporary data contexts may be cleaned up (depending on retention policies)
    /// 4. The workflow record is marked as deleted in the blockchain state
    ///
    /// Note that while the workflow is removed from active management, historical records
    /// of its execution may still be available in the blockchain history for auditing
    /// and tracking purposes.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message containing the workflow deletion parameters, including the
    ///   workflow ID and the creator's address. This should be built using the
    ///   MsgDeleteWorkflowBuilder to ensure all required fields are included.
    ///
    /// # Returns
    ///
    /// * `Result<MsgDeleteWorkflowResponse>` - Response confirming deletion on success,
    ///   or an error. The response typically includes confirmation details and
    ///   any cleanup actions that were taken.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified workflow does not exist
    /// - The caller is not the workflow creator (authorization error)
    /// - The workflow is in a state that prevents deletion
    /// - The connection to the Gevulot blockchain fails
    /// - The response cannot be parsed
    /// - Transaction signing or broadcasting fails
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
