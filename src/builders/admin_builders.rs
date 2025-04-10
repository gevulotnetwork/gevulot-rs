/*!
 * # Admin Builder Types
 *
 * This module provides builders for creating administrative messages in the Gevulot network.
 * These include messages for sudo operations that can only be performed by accounts with
 * administrative privileges.
 */
use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self},
};

/// Builder for creating administrative pin deletion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed for an admin to forcibly delete a pin
/// from the Gevulot network. This is a privileged operation that can only be performed
/// by accounts with administrative authority.
///
/// # Fields
///
/// * `authority` - Identity of the admin account requesting deletion
/// * `cid` - Content Identifier of the data to remove
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgSudoDeletePinBuilder;
///
/// let msg = MsgSudoDeletePinBuilder::default()
///     .authority("gevulot1admin".to_string())
///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgSudoDeletePin {
    /// Identity of the admin account requesting deletion
    /// This must be an account with administrative privileges
    pub authority: String,
    
    /// Content Identifier (CID) of the data to remove
    /// This is the unique hash identifying the data
    pub cid: String,
}

impl MsgSudoDeletePinBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgSudoDeletePin>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgSudoDeletePinBuilder;
    ///
    /// let proto_msg = MsgSudoDeletePinBuilder::default()
    ///     .authority("gevulot1admin".to_string())
    ///     .cid("bafybeihykld7uyxzogax6vgyvag42y7464eywpf55gxi5qpoisibh2xinu".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
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

/// Builder for creating administrative worker deletion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed for an admin to forcibly delete a worker
/// from the Gevulot network. This is a privileged operation that can only be performed
/// by accounts with administrative authority.
///
/// # Fields
///
/// * `authority` - Identity of the admin account requesting deletion
/// * `id` - Unique identifier of the worker to delete
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgSudoDeleteWorkerBuilder;
///
/// let msg = MsgSudoDeleteWorkerBuilder::default()
///     .authority("gevulot1admin".to_string())
///     .id("worker-123456".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgSudoDeleteWorker {
    /// Identity of the admin account requesting deletion
    /// This must be an account with administrative privileges
    pub authority: String,
    
    /// Unique identifier of the worker to delete
    /// This is the blockchain-assigned ID for the worker
    pub id: String,
}

impl MsgSudoDeleteWorkerBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgSudoDeleteWorker>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgSudoDeleteWorkerBuilder;
    ///
    /// let proto_msg = MsgSudoDeleteWorkerBuilder::default()
    ///     .authority("gevulot1admin".to_string())
    ///     .id("worker-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
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

/// Builder for creating administrative task deletion messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed for an admin to forcibly delete a task
/// from the Gevulot network. This is a privileged operation that can only be performed
/// by accounts with administrative authority.
///
/// # Fields
///
/// * `authority` - Identity of the admin account requesting deletion
/// * `id` - Unique identifier of the task to delete
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgSudoDeleteTaskBuilder;
///
/// let proto_msg = MsgSudoDeleteTaskBuilder::default()
///     .authority("gevulot1admin".to_string())
///     .id("task-123456".to_string())
///     .into_message()
///     .unwrap();
///
/// // proto_msg can now be sent to the blockchain
/// ```
#[derive(Builder)]
pub struct MsgSudoDeleteTask {
    /// Identity of the admin account requesting deletion
    /// This must be an account with administrative privileges
    pub authority: String,
    
    /// Unique identifier of the task to delete
    /// This is the blockchain-assigned ID for the task
    pub id: String,
}

impl MsgSudoDeleteTaskBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgSudoDeleteTask>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgSudoDeleteTaskBuilder;
    ///
    /// let proto_msg = MsgSudoDeleteTaskBuilder::default()
    ///     .authority("gevulot1admin".to_string())
    ///     .id("task-123456".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
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

/// Builder for creating account freezing messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed for an admin to freeze a user account
/// in the Gevulot network. This is a privileged operation that can only be performed
/// by accounts with administrative authority.
///
/// # Fields
///
/// * `authority` - Identity of the admin account requesting the freeze
/// * `account` - Address of the account to freeze
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgSudoFreezeAccountBuilder;
///
/// let msg = MsgSudoFreezeAccountBuilder::default()
///     .authority("gevulot1admin".to_string())
///     .account("gevulot1user123".to_string())
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgSudoFreezeAccount {
    /// Identity of the admin account requesting the freeze
    /// This must be an account with administrative privileges
    pub authority: String,
    
    /// Address of the account to freeze
    /// The targeted user account that will be frozen
    pub account: String,
}

impl MsgSudoFreezeAccountBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgSudoFreezeAccount>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgSudoFreezeAccountBuilder;
    ///
    /// let proto_msg = MsgSudoFreezeAccountBuilder::default()
    ///     .authority("gevulot1admin".to_string())
    ///     .account("gevulot1user123".to_string())
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
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