/*!
 * # Params Builder Types
 *
 * This module provides builders for creating parameter-related messages in the Gevulot network.
 * These include messages for updating the module parameters.
 */
use derive_builder::Builder;

use crate::{
    error::{Error, Result},
    proto::gevulot::gevulot::{self, Params},
};

/// Builder for constructing parameter update messages for the Gevulot blockchain.
///
/// This struct represents the parameters needed to update the Gevulot module parameters.
/// Parameter updates are typically governance operations and can only be performed
/// by accounts with administrative privileges.
///
/// # Fields
///
/// * `authority` - Identity of the account with authority to update parameters
/// * `params` - The complete set of parameters to update
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::MsgUpdateParamsBuilder;
/// use gevulot_rs::proto::gevulot::gevulot::Params;
///
/// let mut params = Params::default();
/// params.required_worker_stake = "1000000000".to_string(); // 1000 GVLT
/// params.worker_exit_delay = 100; // 100 blocks
/// 
/// let msg = MsgUpdateParamsBuilder::default()
///     .authority("gevulot1admin".to_string())
///     .params(params)
///     .build()
///     .unwrap();
/// ```
#[derive(Builder)]
pub struct MsgUpdateParams {
    /// Identity of the account with authority to update parameters
    /// This must be an account with administrative privileges
    pub authority: String,
    
    /// The complete set of parameters to update
    /// All parameter values must be supplied, even ones that aren't changing
    pub params: Params,
}

impl MsgUpdateParamsBuilder {
    /// Converts the builder into a protocol message ready for transmission.
    ///
    /// This method transforms the builder's configuration into the proper protobuf
    /// message structure used by the Gevulot blockchain.
    ///
    /// # Returns
    ///
    /// * `Result<gevulot::MsgUpdateParams>` - The protocol message on success, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the builder is missing required fields or has invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::MsgUpdateParamsBuilder;
    /// use gevulot_rs::proto::gevulot::gevulot::Params;
    ///
    /// let params = Params::default(); // Using default values
    ///
    /// let proto_msg = MsgUpdateParamsBuilder::default()
    ///     .authority("gevulot1admin".to_string())
    ///     .params(params)
    ///     .into_message()
    ///     .unwrap();
    ///
    /// // proto_msg can now be sent to the blockchain
    /// ```
    pub fn into_message(&self) -> Result<gevulot::MsgUpdateParams> {
        let msg = self
            .build()
            .map_err(|e| Error::EncodeError(e.to_string()))?;
        Ok(gevulot::MsgUpdateParams {
            authority: msg.authority,
            params: Some(msg.params),
        })
    }
} 