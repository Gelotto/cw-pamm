use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Storage, Uint128};
use pamp::{error::ContractError, tokens::Token};

use super::storage::{PoolId, POOLS};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct Pool {
    pub market: Addr,
    pub token: Token,
    pub amount: Uint128,
}

impl Pool {
    pub fn load(
        store: &dyn Storage,
        id: PoolId,
    ) -> Result<Self, ContractError> {
        Ok(POOLS.load(store, id)?)
    }
}

#[cw_serde]
pub struct PoolInfo {
    pub name: String,
    pub description: Option<String>,
}

#[cw_serde]
pub struct PoolAccount {
    pub balance: Uint128,
}

#[cw_serde]
pub enum SubMsgJob {
    Buy { pool: PoolId, buyer: Addr },
}
