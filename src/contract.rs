use crate::error::ContractError;
use crate::execute::claim::exec_claim;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::config::query_config;
use crate::query::has_claimed::query_has_claim;
use crate::state::{ExecuteContext, QueryContext};
use cosmwasm_std::{entry_point, to_json_binary, Env};
use cosmwasm_std::{Binary, Deps, DepsMut, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let mut ctx = ExecuteContext { deps, env, info };
    ctx.instantiate(msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = ExecuteContext { deps, env, info };
    match msg {
        ExecuteMsg::Claim(msg) => exec_claim(ctx, msg),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let ctx = QueryContext { deps, env };
    let result = match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(ctx)?),
        QueryMsg::HasClaimed { address } => to_json_binary(&query_has_claim(ctx, address)?),
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
