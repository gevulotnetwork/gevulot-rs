use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use prost::Message;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{base_client::BaseClient, error::Result};

use cosmos_sdk_proto::cosmos::gov::v1beta1::{
    MsgDeposit, MsgDepositResponse, MsgSubmitProposal, MsgSubmitProposalResponse, MsgVote,
    MsgVoteResponse, MsgVoteWeighted, MsgVoteWeightedResponse, QueryDepositRequest,
    QueryDepositResponse, QueryDepositsRequest, QueryDepositsResponse, QueryParamsRequest,
    QueryParamsResponse, QueryProposalRequest, QueryProposalResponse, QueryProposalsRequest,
    QueryProposalsResponse, QueryTallyResultRequest, QueryTallyResultResponse, QueryVoteRequest,
    QueryVoteResponse, QueryVotesRequest, QueryVotesResponse,
};
use cosmos_sdk_proto::cosmos::upgrade::v1beta1::MsgSoftwareUpgrade;
use cosmos_sdk_proto::Any;

/// Client for interacting with the governance module in the Cosmos SDK.
#[derive(Debug, Clone)]
pub struct GovClient {
    base_client: Arc<RwLock<BaseClient>>,
}

impl GovClient {
    /// Creates a new instance of GovClient.
    ///
    /// # Arguments
    ///
    /// * `base_client` - An Arc-wrapped RwLock of the BaseClient.
    ///
    /// # Returns
    ///
    /// A new instance of GovClient.
    pub fn new(base_client: Arc<RwLock<BaseClient>>) -> Self {
        Self { base_client }
    }

    /// Queries a proposal based on proposal ID.
    pub async fn get_proposal(&mut self, proposal_id: u64) -> Result<QueryProposalResponse> {
        let request = QueryProposalRequest { proposal_id };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .proposal(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries all proposals based on given status.
    pub async fn get_proposals(
        &mut self,
        proposal_status: i32,
        voter: String,
        depositor: String,
    ) -> Result<QueryProposalsResponse> {
        let request = QueryProposalsRequest {
            proposal_status,
            voter,
            depositor,
            pagination: None,
        };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .proposals(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries voted information based on proposalID, voter address.
    pub async fn get_vote(&mut self, proposal_id: u64, voter: String) -> Result<QueryVoteResponse> {
        let request = QueryVoteRequest { proposal_id, voter };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .vote(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries votes of a given proposal.
    pub async fn get_votes(&mut self, proposal_id: u64) -> Result<QueryVotesResponse> {
        let request = QueryVotesRequest {
            proposal_id,
            pagination: None,
        };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .votes(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries all parameters of the gov module.
    pub async fn get_params(&mut self, params_type: String) -> Result<QueryParamsResponse> {
        let request = QueryParamsRequest { params_type };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .params(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries single deposit information based on proposalID, depositor address.
    pub async fn get_deposit(
        &mut self,
        proposal_id: u64,
        depositor: String,
    ) -> Result<QueryDepositResponse> {
        let request = QueryDepositRequest {
            proposal_id,
            depositor,
        };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .deposit(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries all deposits of a single proposal.
    pub async fn get_deposits(&mut self, proposal_id: u64) -> Result<QueryDepositsResponse> {
        let request = QueryDepositsRequest {
            proposal_id,
            pagination: None,
        };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .deposits(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Queries the tally of a proposal vote.
    pub async fn get_tally_result(&mut self, proposal_id: u64) -> Result<QueryTallyResultResponse> {
        let request = QueryTallyResultRequest { proposal_id };
        let response = self
            .base_client
            .write()
            .await
            .gov_client
            .tally_result(request)
            .await?;
        Ok(response.into_inner())
    }

    /// Submits a proposal.
    pub async fn submit_proposal(
        &mut self,
        msg: MsgSubmitProposal,
    ) -> Result<MsgSubmitProposalResponse> {
        let resp: MsgSubmitProposalResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Casts a vote.
    pub async fn vote(&mut self, msg: MsgVote) -> Result<MsgVoteResponse> {
        let resp: MsgVoteResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Casts a weighted vote.
    /// @TODO: Doesnt work because of no Name bound on the message type 🤔
    pub async fn vote_weighted(&mut self, msg: MsgVoteWeighted) -> Result<MsgVoteWeightedResponse> {
        let resp: MsgVoteWeightedResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Submits a deposit to an existing proposal.
    pub async fn deposit(&mut self, msg: MsgDeposit) -> Result<MsgDepositResponse> {
        let resp: MsgDepositResponse = self
            .base_client
            .write()
            .await
            .send_msg_sync(msg, "")
            .await?;
        Ok(resp)
    }

    /// Submits a software upgrade proposal.
    pub async fn submit_software_upgrade(
        &mut self,
        proposer: &str,
        upgrade_msg: MsgSoftwareUpgrade,
        deposit: &str,
    ) -> Result<MsgSubmitProposalResponse> {
        let content = Any {
            type_url: "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade".to_string(),
            value: upgrade_msg.encode_to_vec(),
        };

        let deposit = vec![Coin {
            denom: "ucredit".to_string(),
            amount: deposit.to_string(),
        }];
        let msg = MsgSubmitProposal {
            content: Some(content),
            initial_deposit: deposit,
            proposer: proposer.to_string(),
        };

        self.submit_proposal(msg).await
    }
}
