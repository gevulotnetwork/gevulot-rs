use std::time::Duration;

use cosmrs::{
    rpc::{self, Client},
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
        }
    }

    // Starts fetching events from the blockchain
    pub async fn start_fetching(&mut self) -> Result<()> {
        let rpc_client = rpc::HttpClient::new(self.rpc_url.as_str())?;
        let mut last_indexed_block = if let Some(start_height) = self.start_height {
            start_height
        } else {
            match rpc_client.status().await {
                Ok(status) => status.sync_info.latest_block_height,
                Err(e) => {
                    log::error!("Error fetching latest block status: {:?}", e);
                    return Err(e.into());
                }
            }
        };
        loop {
            // Fetch the latest block height
            let latest_block = match rpc_client
                .status()
                .await
                .map(|status| status.sync_info.latest_block_height)
            {
                Ok(latest_block) => latest_block,
                Err(e) => {
                    log::error!("Error fetching latest block status: {:?}", e);
                    tokio::time::sleep(self.sleep_time).await;
                    continue;
                }
            };

            log::debug!("Latest block number: {}", latest_block);

            // Process new blocks if there are any
            if latest_block > last_indexed_block {
                for height in (last_indexed_block.value() + 1)..=latest_block.value() {
                    // Fetch block results for the given height
                    let block_results =
                        match rpc_client.block_results(Height::from(height as u32)).await {
                            Ok(block_results) => block_results,
                            Err(e) => {
                                log::error!(
                                    "Error fetching block results for height {}: {:?}",
                                    height,
                                    e
                                );
                                tokio::time::sleep(self.sleep_time).await;
                                continue;
                            }
                        };
                    log::debug!("Processing block results for height {}", height);
                    if let Some(txs_results) = block_results.txs_results {
                        // Process each transaction result
                        for event in txs_results.iter().flat_map(|tx| tx.events.iter()) {
                            // Handle each event
                            if let Err(e) = self
                                .handler
                                .handle_event(event, Height::from(height as u32))
                                .await
                            {
                                log::error!("Error handling event: {:?}", e);
                                return Err(e);
                            }
                        }
                    } else {
                        log::debug!("No transaction results found for block height {}", height);
                    }
                    last_indexed_block = latest_block;
                }
            }
            tokio::time::sleep(self.sleep_time).await;
        }
    }
}
