use std::time::Duration;

use backon::{ExponentialBuilder, Retryable};
use cosmrs::{
    rpc::{self, endpoint::block_results::Response as BlockResults, Client},
    tendermint::block::Height,
};

use crate::error::Result;

// Trait for handling events asynchronously
pub trait EventHandler: Send + Sync {
    // Asynchronously handles an event
    fn handle_event(
        &mut self,
        event: &crate::Event,
        block_height: crate::Height,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

// Fetches events from the blockchain and processes them using the provided handler
pub struct EventFetcher<H: EventHandler> {
    pub handler: H,
    pub rpc_url: String,
    pub start_height: Option<Height>,
    pub sleep_time: Duration,
    pub max_retries: usize,
}

impl<H> EventFetcher<H>
where
    H: EventHandler,
{
    // Creates a new EventFetcher
    pub fn new(
        rpc_url: &str,
        start_height: Option<Height>,
        sleep_time: Duration,
        handler: H,
    ) -> Self {
        Self {
            handler,
            rpc_url: rpc_url.to_string(),
            start_height,
            sleep_time,
            max_retries: 3,
        }
    }

    async fn fetch_latest_block_number_no_retry(
        &self,
        rpc_client: &rpc::HttpClient,
    ) -> Result<Height> {
        let status = rpc_client.status().await?;
        Ok(status.sync_info.latest_block_height)
    }

    async fn fetch_latest_block_number(&self, rpc_client: &rpc::HttpClient) -> Result<Height> {
        let backoff = ExponentialBuilder::default()
            .with_max_times(self.max_retries)
            .with_jitter();

        (|| async { self.fetch_latest_block_number_no_retry(rpc_client).await })
            .retry(backoff)
            .await
            .map_err(|e| {
                log::error!(
                    "Error fetching latest block status after {} retries: {:?}",
                    self.max_retries,
                    e
                );
                e
            })
    }

    async fn fetch_block_results_no_retry(
        &self,
        rpc_client: &rpc::HttpClient,
        height: Height,
    ) -> Result<BlockResults> {
        rpc_client.block_results(height).await.map_err(Into::into)
    }

    async fn fetch_block_results(
        &self,
        rpc_client: &rpc::HttpClient,
        height: Height,
    ) -> Result<BlockResults> {
        let backoff = ExponentialBuilder::default()
            .with_max_times(self.max_retries)
            .with_jitter();

        (|| async { self.fetch_block_results_no_retry(rpc_client, height).await })
            .retry(backoff)
            .await
            .map_err(|e| {
                log::error!(
                    "Error fetching block results for height {} after {} retries: {:?}",
                    height,
                    self.max_retries,
                    e
                );
                e
            })
    }

    async fn process_block_results(&mut self, block_results: &BlockResults) -> Result<()> {
        if let Some(txs_results) = &block_results.txs_results {
            for event in txs_results.iter().flat_map(|tx| tx.events.iter()) {
                self.handler
                    .handle_event(event, block_results.height)
                    .await?;
            }
        }
        Ok(())
    }

    // Starts fetching events from the blockchain
    pub async fn start_fetching(&mut self) -> Result<()> {
        let rpc_client = rpc::HttpClient::new(self.rpc_url.as_str())?;
        let mut last_indexed_block = if let Some(start_height) = self.start_height {
            start_height
        } else {
            self.fetch_latest_block_number(&rpc_client).await?
        };

        loop {
            let latest_block = self.fetch_latest_block_number(&rpc_client).await?;

            if latest_block > last_indexed_block {
                for height in (last_indexed_block.value() + 1)..=latest_block.value() {
                    let block_results = self
                        .fetch_block_results(&rpc_client, Height::from(height as u32))
                        .await?;
                    log::debug!("Processing block results for height {}", height);
                    self.process_block_results(&block_results).await?;
                    last_indexed_block = Height::from(height as u32);
                }
            }
            tokio::time::sleep(self.sleep_time).await;
        }
    }
}
