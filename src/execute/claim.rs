use crate::{error::ContractError, msg::ClaimMsg, state::ExecuteContext};
use cosmwasm_std::Response;

pub fn exec_claim(
    mut ctx: ExecuteContext,
    msg: ClaimMsg,
) -> Result<Response, ContractError> {
    ctx.claim(msg.proof)
}
