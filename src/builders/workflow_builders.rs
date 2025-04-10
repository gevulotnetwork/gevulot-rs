/*!
 * # Workflow Builder Types
 *
 * This module provides builders for creating workflow-related messages in the Gevulot network.
 * These include messages for creating and deleting computational workflows.
 */
use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self, WorkflowSpec},
};

/// Builder for constructing workflow creation messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to create a computational workflow
/// in the Gevulot network. A workflow consists of one or more stages of tasks that
/// are executed sequentially.
///
/// # Fields
///
/// * `creator` - Identity of the account creating the workflow
/// * `spec` - The workflow specification defining stages and tasks
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgCreateWorkflowBuilder;
/// use gevulot_rs::proto::gevulot::gevulot::{WorkflowSpec, TaskSpec};
///
/// // Create a simple workflow spec
/// let workflow_spec = WorkflowSpec::default();
///
/// let msg = MsgCreateWorkflowBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .spec(workflow_spec)
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgCreateWorkflow {
    /// Identity of the account creating the workflow
    /// This must be a valid Gevulot account address
    pub creator: String,
    
    /// The workflow specification defining stages and tasks
    /// This defines the execution plan for the workflow
    pub spec: WorkflowSpec,
}

impl MsgCreateWorkflowBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgCreateWorkflow>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgCreateWorkflowBuilder;
    /// use gevulot_rs::proto::gevulot::gevulot::WorkflowSpec;
    ///
    /// let workflow_spec = WorkflowSpec::default(); // A simple empty workflow
    ///
    /// let proto_msg = MsgCreateWorkflowBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .spec(workflow_spec)
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgCreateWorkflow> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgCreateWorkflow {
            creator: msg.creator,
            spec: Some(msg.spec),
        })
    }
}

/// Builder for deleting a workflow from the Gevulot blockchain.
///
/// This struct represents the parameters needed to delete a previously created
/// workflow in the Gevulot network. Only the original creator can delete a workflow.
///
/// # Fields
///
/// * `creator` - Identity of the account requesting deletion
/// * `id` - Unique identifier of the workflow to delete
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgDeleteWorkflowBuilder;
///
/// let msg = MsgDeleteWorkflowBuilder::default()
///     .creator("gevulot1abcdef".to_string())
///     .id("workflow-123456".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgDeleteWorkflow {
    /// Identity of the account requesting workflow deletion
    /// This must match the original creator of the workflow
    pub creator: String,
    
    /// Unique identifier of the workflow to delete
    /// This is the blockchain-assigned ID for the workflow
    pub id: String,
}

impl MsgDeleteWorkflowBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgDeleteWorkflow>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgDeleteWorkflowBuilder;
    ///
    /// let proto_msg = MsgDeleteWorkflowBuilder::default()
    ///     .creator("gevulot1abcdef".to_string())
    ///     .id("workflow-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgDeleteWorkflow> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgDeleteWorkflow {
            creator: msg.creator,
            id: msg.id,
        })
    }
} 