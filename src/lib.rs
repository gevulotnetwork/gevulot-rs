/// This module contains the base client implementation.
pub mod base_client;
/// This module contains various builders for constructing messages.
pub mod builders;
/// This module contains the client implementation for Gevulot.
pub mod gevulot_client;
/// This module contains the client implementation for managing pins.
pub mod pin_client;
/// This module contains the client implementation for sudo functionality.
pub mod sudo_client;
/// This module contains the client implementation for managing tasks.
pub mod task_client;
/// This module contains the client implementation for managing workers.
pub mod worker_client;
/// This module contains the client implementation for managing workflows.
pub mod workflow_client;

pub mod models;
pub mod runtime_config;

pub mod error;
pub mod event_fetcher;
pub mod events;
pub mod gov_client;
/// This module contains the signer implementation.
mod signer;

/// This module contains the protocol buffer definitions.
pub mod proto {
    pub mod cosmos {
        pub mod base {
            pub mod query {
                pub mod v1beta1 {
                    tonic::include_proto!("cosmos.base.query.v1beta1");
                }
            }
        }

        pub mod app {
            pub mod v1alpha1 {
                tonic::include_proto!("cosmos.app.v1alpha1");
            }
        }
    }

    pub mod cosmos_proto {
        tonic::include_proto!("cosmos_proto");
    }

    pub mod google {
        tonic::include_proto!("google.api");
    }

    pub mod gevulot {
        #![allow(clippy::module_inception)]
        pub mod gevulot {
            tonic::include_proto!("gevulot.gevulot");
            pub mod module {
                tonic::include_proto!("gevulot.gevulot.module");
            }
        }
    }
}

pub use cosmrs::tendermint::abci::Event;
pub use cosmrs::tendermint::block::Height;
pub use error::{Error, Result};
pub use event_fetcher::{EventFetcher, EventHandler};
pub use events::GevulotEvent;
pub use gevulot_client::{GevulotClient, GevulotClientBuilder};

#[cfg(test)]
mod tests {
    use cosmrs::tendermint::block::Height;

    use self::builders::ByteUnit::{Byte, Gigabyte};

    use super::*;

    /// Helper function to read Alice's seed and address from files.
    fn alice() -> (String, String) {
        let seed_path = "../../.dev-node/alice_seed.txt";
        let seed = std::fs::read_to_string(seed_path).expect("Unable to read seed file");
        let address_path = "../../.dev-node/alice_address.txt";
        let address = std::fs::read_to_string(address_path).expect("Unable to read address file");
        (seed.trim().to_string(), address.trim().to_string())
    }

    #[tokio::test]
    async fn test_event_fetching() {
        pretty_env_logger::init();

        struct EventLogger;

        impl event_fetcher::EventHandler for EventLogger {
            async fn handle_event(
                &mut self,
                event: &crate::Event,
                block_height: crate::Height,
            ) -> crate::Result<()> {
                match events::GevulotEvent::from_cosmos(event, block_height) {
                    Ok(e) => println!("{:?}", e),
                    Err(e) => println!("Error: {:?}", e),
                }
                Ok(())
            }
        }

        let mut fetcher = event_fetcher::EventFetcher::new(
            "http://127.0.0.1:26657",
            Some(Height::from(0u32)),
            tokio::time::Duration::from_secs(5),
            EventLogger {},
        );

        fetcher.start_fetching().await.unwrap();
    }

    /// End-to-end test for the Gevulot client.
    #[tokio::test]
    async fn test_e2e() {
        let (mnemonic, address) = alice();

        let mut cli = GevulotClientBuilder::new()
            .endpoint("http://127.0.0.1:9090") // default endpoint
            .gas_price(1000) // default gas price
            .gas_multiplier(1.2) // default gas multiplier
            .mnemonic(mnemonic.as_str())
            .build()
            .await
            .unwrap();

        // Register a worker
        let worker_msg = builders::MsgCreateWorkerBuilder::default()
            .creator(address.clone())
            .name("test_worker".to_string())
            .cpus(1000)
            .gpus(1000)
            .memory((32, Gigabyte).into())
            .disk((128, Gigabyte).into())
            .into_message()
            .expect("Failed to build worker message");

        let worker_id = cli.workers.create(worker_msg).await.unwrap().id;

        // Create a pin
        let pin_msg = builders::MsgCreatePinBuilder::default()
            .creator(address.clone())
            .cid(Some("QmSWeBJYvDqKUFG3om4gsrKGf379zk8Jq5tYXpDp7Xo".to_string()))
            .bytes((32, Byte).into())
            .time(3600)
            .redundancy(1)
            .name("test".to_string())
            .into_message()
            .expect("Failed to build pin message");

        cli.pins.create(pin_msg).await.unwrap();

        // Delete the pin
        let delete_pin_msg = builders::MsgDeletePinBuilder::default()
            .creator(address.clone())
            .cid("QmSWeBJYvDqKUFG3om4gsrKGf379zk8Jq5tYXpDp7Xo".to_string())
            .into_message()
            .expect("Failed to build pin message");

        cli.pins.delete(delete_pin_msg).await.unwrap();

        // Announce worker exit
        let announce_exit_msg = builders::MsgAnnounceWorkerExitBuilder::default()
            .creator(address.clone())
            .worker_id(worker_id.clone())
            .into_message()
            .expect("Failed to build worker message");

        cli.workers.announce_exit(announce_exit_msg).await.unwrap();

        {
            // Wait for 10 blocks to be mined
            let mut base_client = cli.base_client.write().await;
            let current_block = base_client.current_block().await.unwrap();
            let current_height = current_block
                .header
                .as_ref()
                .ok_or("Header not found")
                .unwrap()
                .height;
            base_client
                .wait_for_block(current_height + 10)
                .await
                .unwrap();
        }

        // Delete the worker
        let delete_worker_msg = builders::MsgDeleteWorkerBuilder::default()
            .creator(address.clone())
            .id(worker_id.clone())
            .into_message()
            .expect("Failed to build worker message");

        cli.workers.delete(delete_worker_msg).await.unwrap();
    }
}
