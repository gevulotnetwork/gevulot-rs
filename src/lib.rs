pub mod signer;
mod builders;

use std::collections::HashMap;

use cosmos_sdk_proto::cosmos::tx::v1beta1::{BroadcastTxResponse, SimulateResponse};
use cosmrs::{auth::BaseAccount, Any, Coin};
use prost::{Message, Name};
use tonic::transport::Channel;

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
        pub mod gevulot {
            tonic::include_proto!("gevulot.gevulot");
            pub mod module {
                tonic::include_proto!("gevulot.gevulot.module");
            }
        }
    }
}

type AuthQueryClient<T> = cosmrs::proto::cosmos::auth::v1beta1::query_client::QueryClient<T>;
type GevulotQueryClient<T> = proto::gevulot::gevulot::query_client::QueryClient<T>;
type TxServiceClient<T> = cosmrs::proto::cosmos::tx::v1beta1::service_client::ServiceClient<T>;

pub struct GevulotClient {
    // query
    auth_client: AuthQueryClient<Channel>,
    gevulot_client: GevulotQueryClient<Channel>,
    // message
    tx_client: TxServiceClient<Channel>,

    gas_price: u128,
    denom: String,
    gas_multiplier: f64,
}

impl GevulotClient {
    pub async fn new(endpoint: &str, gas_price: u128, denom: &str, gas_multiplier: f64) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(endpoint.to_owned())?.connect().await?;
        Ok(Self {
            auth_client: AuthQueryClient::new(channel.clone()),
            gevulot_client: GevulotQueryClient::new(channel.clone()),
            tx_client: TxServiceClient::new(channel),
            gas_price: gas_price,
            denom: denom.to_owned(),
            gas_multiplier: gas_multiplier,
        })
    }

    pub async fn get_account(
        &mut self,
        address: &str,
    ) -> Result<BaseAccount, Box<dyn std::error::Error>> {
        let request = cosmrs::proto::cosmos::auth::v1beta1::QueryAccountRequest {
            address: address.to_owned(),
        };
        let response = self.auth_client.account(request).await?;
        if let Some(cosmrs::Any { type_url: _, value }) = response.into_inner().account {
            let base_account = BaseAccount::try_from(
                cosmrs::proto::cosmos::auth::v1beta1::BaseAccount::decode(value.as_ref())?,
            )?;

            Ok(base_account)
        } else {
            Err("Can't load the associated account.".into())
        }
    }

    pub async fn get_all_workers(
        &mut self,
    ) -> Result<Vec<proto::gevulot::gevulot::Worker>, Box<dyn std::error::Error>> {
        let request = proto::gevulot::gevulot::QueryAllWorkerRequest { pagination: None };
        let response = self.gevulot_client.worker_all(request).await?;
        Ok(response.into_inner().worker)
    }

    pub async fn get_worker(
        &mut self,
        id: &str,
    ) -> Result<proto::gevulot::gevulot::Worker, Box<dyn std::error::Error>> {
        let request = proto::gevulot::gevulot::QueryGetWorkerRequest { id: id.to_owned() };
        let response = self.gevulot_client.worker(request).await?;
        Ok(response.into_inner().worker.ok_or("Worker not found")?)
    }

    pub async fn get_all_tasks(
        &mut self,
    ) -> Result<Vec<proto::gevulot::gevulot::Task>, Box<dyn std::error::Error>> {
        let request = proto::gevulot::gevulot::QueryAllTaskRequest { pagination: None };
        let response = self.gevulot_client.task_all(request).await?;
        Ok(response.into_inner().task)
    }

    pub async fn get_task(
        &mut self,
        id: &str,
    ) -> Result<proto::gevulot::gevulot::Task, Box<dyn std::error::Error>> {
        let request = proto::gevulot::gevulot::QueryGetTaskRequest { id: id.to_owned() };
        let response = self.gevulot_client.task(request).await?;
        Ok(response.into_inner().task.ok_or("Task not found")?)
    }

    pub async fn get_all_pins(
        &mut self,
    ) -> Result<Vec<proto::gevulot::gevulot::Pin>, Box<dyn std::error::Error>> {
        let request = proto::gevulot::gevulot::QueryAllPinRequest { pagination: None };
        let response = self.gevulot_client.pin_all(request).await?;
        Ok(response.into_inner().pin)
    }

    pub async fn get_pin(
        &mut self,
        cid: &str,
    ) -> Result<proto::gevulot::gevulot::Pin, Box<dyn std::error::Error>> {
        let request = proto::gevulot::gevulot::QueryGetPinRequest {
            cid: cid.to_owned(),
        };
        let response = self.gevulot_client.pin(request).await?;
        Ok(response.into_inner().pin.ok_or("Pin not found")?)
    }

    pub async fn simulate_msg<M: Name>(
        &mut self,
        msg: M,
        memo: &str,
        signer: &signer::Signer,
    ) -> Result<SimulateResponse, Box<dyn std::error::Error>> {
        let msg = cosmrs::Any::from_msg(&msg)?;
        let gas = 100_000u64;
        let address = signer.public_address.to_string();
        let pub_key = signer.public_key;
        let base_account = self.get_account(&address).await?;
        let chain_id: cosmrs::tendermint::chain::Id = "gevulot".parse()?;
        let tx_body = cosmrs::tx::BodyBuilder::new().msg(msg).memo(memo).finish();
        let signer_info =
            cosmrs::tx::SignerInfo::single_direct(Some(pub_key), base_account.sequence);
        let fee = cosmrs::tx::Fee::from_amount_and_gas(Coin {
                denom: self.denom.parse()?,
                amount: self.gas_price,
            },
            gas,
        );
        let auth_info = signer_info.auth_info(fee);
        let sign_doc =
            cosmrs::tx::SignDoc::new(&tx_body, &auth_info, &chain_id, base_account.account_number)?;
        let tx_raw = sign_doc.sign(&signer.private_key)?;
        let tx_bytes = tx_raw.to_bytes()?;
        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::SimulateRequest {
            tx_bytes: tx_bytes,
            tx: None,
        };
        let response = self.tx_client.simulate(request).await?;
        Ok(response.into_inner())
    }

    pub async fn send_msg<M: Name + Clone>(
        &mut self,
        msg: M,
        memo: &str,
        signer: &signer::Signer,
    ) -> Result<BroadcastTxResponse, Box<dyn std::error::Error>> {
        // Use simulate_msg to estimate gas
        let simulate_response = self.simulate_msg(msg.clone(), memo, signer).await?;
        let gas_info = simulate_response.gas_info.ok_or("Failed to get gas info")?;
        let gas_limit = (gas_info.gas_used * (100. * self.gas_multiplier) as u64) / 100; // Adjust gas limit based on simulation
        let fee = cosmrs::tx::Fee::from_amount_and_gas(Coin {
                denom: self.denom.parse()?,
                amount: self.gas_price,
            },
            gas_limit,
        );

        let msg = cosmrs::Any::from_msg(&msg)?;
        let address = signer.public_address.to_string();
        let pub_key = signer.public_key;
        let base_account = self.get_account(&address).await?;
        let chain_id: cosmrs::tendermint::chain::Id = "gevulot".parse()?;
        let tx_body = cosmrs::tx::BodyBuilder::new().msg(msg).memo(memo).finish();
        let signer_info =
            cosmrs::tx::SignerInfo::single_direct(Some(pub_key), base_account.sequence);
        let auth_info = signer_info.auth_info(fee);
        let sign_doc =
            cosmrs::tx::SignDoc::new(&tx_body, &auth_info, &chain_id, base_account.account_number)?;
        let tx_raw = sign_doc.sign(&signer.private_key)?;
        let tx_bytes = tx_raw.to_bytes()?;

        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest {
            tx_bytes: tx_bytes,
            mode: 2, // BROADCAST_MODE_SYNC -> Wait for the tx to be processed, but not in-block
        };
        let resp = self.tx_client.broadcast_tx(request).await?;

        Ok(resp.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use self::builders::{ByteSize, ByteUnit::{Byte, Gigabyte}};

    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_signer2() {
        let mnemonic = "shop pipe invest plate december much language neutral company notable jacket warm carry taste seat rocket exotic measure scatter tennis build still ten diagram";
        let signer = signer::Signer::from_mnemonic(mnemonic, "gvlt", None).unwrap();
        assert_eq!(
            signer.public_address.to_string(),
            "gvlt1tuy2dkr52tl0pu595n9dwjtqjfztczz7rh78zm"
        );
    }

    #[tokio::test]
    async fn test_get_account() {
        let mut cli = GevulotClient::new("http://127.0.0.1:9090", 1000, "gvlt", 1.2).await.unwrap();
        let account = cli
            .get_account("gvlt1tuy2dkr52tl0pu595n9dwjtqjfztczz7rh78zm")
            .await
            .unwrap();
        println!("{:?}", account);
    }

    #[tokio::test]
    async fn query_all_workers() {
        let mut cli = GevulotClient::new("http://127.0.0.1:9090", 1000, "gvlt", 1.2).await.unwrap();
        let res = cli.get_all_workers().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn create_worker() {
        let mut cli = GevulotClient::new("http://127.0.0.1:9090", 1000, "gvlt", 1.2).await.unwrap();
        let signer = signer::Signer::from_mnemonic(
            "shop pipe invest plate december much language neutral company notable jacket warm carry taste seat rocket exotic measure scatter tennis build still ten diagram",
            "gvlt",
            None,
        ).unwrap();
        let msg = builders::MsgCreateWorkerBuilder::new()
            .creator(&signer.public_address.to_string())
            .name("test")
            .cpus(1000)
            .gpus(1000)
            .memory((32, Gigabyte).into())
            .disk((128, Gigabyte).into())
            .build();
        cli.send_msg(msg, "", &signer).await.unwrap();
    }

    #[tokio::test]
    async fn create_pin() {
        let mut cli = GevulotClient::new("http://127.0.0.1:9090", 1000, "gvlt", 1.2).await.unwrap();
        let signer = signer::Signer::from_mnemonic(
            "shop pipe invest plate december much language neutral company notable jacket warm carry taste seat rocket exotic measure scatter tennis build still ten diagram",
            "gvlt",
            None,
        ).unwrap();
        let msg = builders::MsgCreatePinBuilder::new()
            .cid("QmSWeBJYvDqKUFG3om4gsrKGf379zk8Jq5tYXpDp7Xo")
            .creator(&signer.public_address.to_string())
            .bytes((32, Byte).into())
            .time(3600)
            .redundancy(1)
            .name("test")
            .build();
        
        let resp = cli.send_msg(msg, "", &signer).await.unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn delete_pin() {
        let mut cli = GevulotClient::new("http://127.0.0.1:9090", 1000, "gvlt", 1.2).await.unwrap();
        let signer = signer::Signer::from_mnemonic(
            "shop pipe invest plate december much language neutral company notable jacket warm carry taste seat rocket exotic measure scatter tennis build still ten diagram",
            "gvlt",
            None,
        ).unwrap();
        let msg = builders::MsgDeletePinBuilder::new()
            .creator(&signer.public_address.to_string())
            .cid("QmSWeBJYvDqKUFG3om4gsrKGf379zk8Jq5tYXpDp7Xo")
            .build();
        let resp = cli.send_msg(msg, "", &signer).await.unwrap();
        println!("{:?}", resp);
    }
}
