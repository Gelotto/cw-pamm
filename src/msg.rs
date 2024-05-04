use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

use crate::{state::models::Config, token::Token};

#[cw_serde]
pub struct PoolInitArgs {
    pub name: String,
    pub description: Option<String>,
    pub market: Addr,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub pools: Vec<PoolInitArgs>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Pools {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct PoolAppObject {
    pub name: String,
    pub description: Option<String>,
    pub token: Token,
    pub amount: Uint128,
}

#[cw_serde]
pub struct PoolsResponse {
    pub pools: Vec<PoolAppObject>,
}
