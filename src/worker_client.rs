use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    base_client::BaseClient,
    error::{Error, Result},
    proto::{
        cosmos::base::query::v1beta1::PageRequest,
        gevulot::gevulot::{
            MsgAnnounceWorkerExit, MsgAnnounceWorkerExitResponse, MsgCreateWorker,
            MsgCreateWorkerResponse, MsgDeleteWorker, MsgDeleteWorkerResponse, MsgUpdateWorker,
            MsgUpdateWorkerResponse, QueryAllWorkerRequest,
        },
    },
};

/// Default page size for pagination.
const PAGE_SIZE: u64 = 100;

/// Client for managing workers in the Gevulot system.
#[derive(Debug, Clone)]
pub struct WorkerClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl WorkerClient {
    /// Creates a new instance of WorkerClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient.
    ///
    /// # Returns
    ///
    /// A new instance of WorkerClient.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Lists all workers.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of workers or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn list(&mut self) -> Result<Vec<crate::proto::gevulot::gevulot::Worker>> {
        let mut all_workers = Vec::new();
        let mut next_key: Option<Vec<u8>> = None;

        loop {
            // Construct request with pagination for the current page.
            let pagination = Some(PageRequest {
                key: next_key.unwrap_or_default(),
                limit: PAGE_SIZE,
                ..Default::default()
            });
            let request = QueryAllWorkerRequest { pagination };

            let response = self
                .base_client
                .write()
                .await
                .gevulot_client
                .worker_all(request)
                .await?;

            let inner = response.into_inner();
            all_workers.extend(inner.worker);

            // Handle next page.
            next_key = inner.pagination.and_then(|p| {
                if p.next_key.is_empty() {
                    None
                } else {
                    Some(p.next_key)
                }
            });
            if next_key.is_none() {
                break;
            }
        }

        Ok(all_workers)
    }

    /// Gets a worker by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the worker to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the worker or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the worker is not found or if the request to the Gevulot client fails.
    pub async fn get(&mut self, id: &str) -> Result<crate::proto::gevulot::gevulot::Worker> {
        let request = crate::proto::gevulot::gevulot::QueryGetWorkerRequest { id: id.to_owned() };
        let response = self
            .base_client
            .write()
            .await
            .gevulot_client
            .worker(request)
            .await?;
        response.into_inner().worker.ok_or(Error::NotFound)
    }

    /// Creates a new worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker details.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn create(&mut self, msg: MsgCreateWorker) -> Result<MsgCreateWorkerResponse> {
        let resp: MsgCreateWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Updates a worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker details to update.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn update(&mut self, msg: MsgUpdateWorker) -> Result<MsgUpdateWorkerResponse> {
        let resp: MsgUpdateWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Deletes a worker.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker ID to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn delete(&mut self, msg: MsgDeleteWorker) -> Result<MsgDeleteWorkerResponse> {
        let resp: MsgDeleteWorkerResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Announces a worker's exit.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message containing the worker ID to announce exit.
    ///
    /// # Returns
    ///
    /// A Result containing the response or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the Gevulot client fails.
    pub async fn announce_exit(
        &mut self,
        msg: MsgAnnounceWorkerExit,
    ) -> Result<MsgAnnounceWorkerExitResponse> {
        let resp: MsgAnnounceWorkerExitResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }
}
