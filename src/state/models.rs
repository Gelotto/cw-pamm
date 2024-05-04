use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

use crate::token::Token;

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct Pool {
    pub market: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct PoolInfo {
    pub name: String,
    pub description: Option<String>,
}
