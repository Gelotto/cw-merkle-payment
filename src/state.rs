use cosmwasm_schema::cw_serde;
use cosmwasm_std::{attr, Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, Uint128, Uint64};
use cw_storage_plus::{Item, Map};
use sha2::{Digest, Sha256};

use crate::{error::ContractError, msg::InstantiateMsg, token::Token};

pub const CONFIG: Item<Config> = Item::new("config");
pub const METADATA: Item<AirdropMetadata> = Item::new("metadata");
pub const MERKLE_ROOT: Item<String> = Item::new("root");
pub const CLAIMS: Map<&Addr, bool> = Map::new("claims");
pub const N_CLAIMS_PROCESSED: Item<Uint64> = Item::new("n_claims_processed");

pub type ExecuteResult = Result<Response, ContractError>;

#[cw_serde]
pub struct Config {
    /// Admin of the airdrop
    pub owner: Addr,
    /// Token type being airdropped
    pub token: Token,
    /// Token amount claimable by each recipient
    pub amount: Uint128,
    /// Merkel tree root hash
    pub root: String,
    /// Number of total claims in merkle tree
    pub size: Uint64,
}

#[cw_serde]
pub struct AirdropMetadata {
    /// Name of the airdrop
    pub name: String,
    /// Name of the airdrop
    pub description: Option<String>,
}

pub struct ExecuteContext<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

pub struct QueryContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}

impl ExecuteContext<'_> {
    /// Top-level initialization of contract state
    pub fn instantiate(
        &mut self,
        msg: InstantiateMsg,
    ) -> ExecuteResult {
        let InstantiateMsg { config, metadata } = msg;

        // TODO: validation

        CONFIG.save(self.deps.storage, &config)?;
        METADATA.save(self.deps.storage, &metadata)?;
        N_CLAIMS_PROCESSED.save(self.deps.storage, &Uint64::zero())?;

        self.set_merkle_root(config.root)?;

        Ok(Response::new().add_attribute("action", "instantiate"))
    }

    pub fn claim(
        &mut self,
        proof: Vec<String>,
    ) -> ExecuteResult {
        let claimant = self.info.sender.clone();

        // Prevent double claims
        if CLAIMS.has(self.deps.storage, &claimant) {
            return Err(ContractError::NotAuthorized {
                reason: "claimed".into(),
            });
        }

        // Apply Merkle tree verification
        let proposed_root_hash = Self::compute_merkle_hash(&claimant, proof)?;
        let expected_root_hash = self.load_merkle_root()?;
        if proposed_root_hash != expected_root_hash {
            return Err(ContractError::NotAuthorized {
                reason: "merkle tree verification failed".into(),
            });
        }

        // Send claim amount to claimant
        let config = CONFIG.load(self.deps.storage)?;
        let transfer_submsg = config.token.transfer(&claimant, config.amount)?;

        // Increment total number of claims processed to date
        N_CLAIMS_PROCESSED.update(self.deps.storage, |n| -> Result<_, ContractError> {
            Ok((n.u64() + 1u64).into())
        })?;

        Ok(Response::new().add_submessage(transfer_submsg).add_attributes(vec![
            attr("action", "claim"),
            attr("amount", config.amount),
            attr("claimant", claimant),
        ]))
    }

    fn set_merkle_root(
        &mut self,
        root: String,
    ) -> Result<(), ContractError> {
        // check merkle root length
        let mut root_buf: [u8; 32] = [0; 32];
        hex::decode_to_slice(&root, &mut root_buf)
            .map_err(|e| ContractError::Std(StdError::generic_err(e.to_string())))?;
        MERKLE_ROOT.save(self.deps.storage, &root)?;
        Ok(())
    }

    fn load_merkle_root(&self) -> Result<[u8; 32], ContractError> {
        let mut root_buf: [u8; 32] = [0; 32];
        let merkle_root = MERKLE_ROOT.load(self.deps.storage)?;
        hex::decode_to_slice(merkle_root, &mut root_buf).map_err(|_| ContractError::ValidationError {
            reason: "error hex decoding merkle root".into(),
        })?;
        Ok(root_buf)
    }

    fn compute_merkle_hash(
        claimant_addr: &Addr,
        merkle_proof: Vec<String>,
    ) -> Result<[u8; 32], ContractError> {
        let hash = Sha256::digest(claimant_addr.as_bytes())
            .as_slice()
            .try_into()
            .map_err(|_| ContractError::ValidationError {
                reason: "sha hash input too long".into(),
            })?;

        let hash = merkle_proof.into_iter().try_fold(hash, |hash, p| {
            let mut proof_buf = [0; 32];
            hex::decode_to_slice(p, &mut proof_buf).map_err(|_| ContractError::ValidationError {
                reason: "error hex decoding merkle proof".into(),
            })?;
            let mut hashes = [hash, proof_buf];
            hashes.sort_unstable();
            sha2::Sha256::digest(&hashes.concat())
                .as_slice()
                .try_into()
                .map_err(|_| ContractError::ValidationError {
                    reason: "invalid merkle proof length".into(),
                })
        })?;

        Ok(hash)
    }
}
