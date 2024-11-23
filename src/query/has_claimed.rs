use cosmwasm_std::Addr;

use crate::{
    error::ContractError,
    responses::HasClaimedResponse,
    state::{QueryContext, CLAIMS},
};

pub fn query_has_claim(
    ctx: QueryContext,
    claimant: Addr,
) -> Result<HasClaimedResponse, ContractError> {
    let QueryContext { deps, .. } = ctx;
    Ok(HasClaimedResponse {
        has_claimed: CLAIMS.has(deps.storage, &claimant),
    })
}
