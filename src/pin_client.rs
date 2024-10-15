use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::{Error, Result},
    proto::gevulot::gevulot::{
        MsgAckPin, MsgAckPinResponse, MsgCreatePin, MsgCreatePinResponse, MsgDeletePin,
        MsgDeletePinResponse,
    },
};

/// Client for managing pins in the Gevulot system.
#[derive(Debug, Clone)]
pub struct PinClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl PinClient {
    /// Creates a new instance of PinClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient.
    ///
    /// # Returns
    ///
    /// A new instance of PinClient.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all pins.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of pins or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn list(&mut self) -> Result<Vec<crate::proto::gevulot::gevulot::Pin>> {
        let request = crate::proto::gevulot::gevulot::QueryAllPinRequest { pagination: None };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .pin_all(request)
            .await?;
        Ok(response.into_inner().pin)
    }

    /// Gets a pin by its CID.
    ///
    /// # Arguments
    ///
    /// * `cid` - The CID of the pin to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the pin or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the pin is not found or if the request to the Gevulot client fails.
    pub async fn get(&mut self, cid: &str) -> Result<crate::proto::gevulot::gevulot::Pin> {
        let request = crate::proto::gevulot::gevulot::QueryGetPinRequest {
            cid: cid.to_owned(),
        };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .pin(request)
            .await?;
        response.into_inner().pin.ok_or(Error::NotFound)
    }

    /// Creates a new pin.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the pin to create.
    ///
    /// # Returns
    ///
    /// A Result containing the response of the create pin operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn create(&mut self, msg: MsgCreatePin) -> Result<MsgCreatePinResponse> {
        let resp: MsgCreatePinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
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
    pub async fn delete(&mut self, msg: MsgDeletePin) -> Result<MsgDeletePinResponse> {
        let resp: MsgDeletePinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Acknowledges a pin.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the details of the pin to acknowledge.
    ///
    /// # Returns
    ///
    /// A Result containing the response of the acknowledge pin operation or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn ack(&mut self, msg: MsgAckPin) -> Result<MsgAckPinResponse> {
        let resp: MsgAckPinResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
