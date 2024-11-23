use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[allow(unused_imports)]
use crate::{responses::ConfigResponse, state::Config};
use crate::{responses::HasClaimedResponse, state::AirdropMetadata};

#[cw_serde]
pub struct InstantiateMsg {
    pub metadata: AirdropMetadata,
    pub config: Config,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Claim(ClaimMsg),
}

#[cw_serde]
pub struct ClaimMsg {
    pub proof: Vec<String>,
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(HasClaimedResponse)]
    HasClaimed { address: Addr },
}

#[cw_serde]
pub struct MigrateMsg {}
