use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use pamp::{market::responses::MarketInfoResponse, tokens::Token};

use crate::state::{models::Config, storage::PoolId};

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
    Buy(BuyParams),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Pools {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct BuyParams {
    pub pool: PoolId,
}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct PoolMarketInfo {
    pub address: Addr,
    pub info: MarketInfoResponse,
}

#[cw_serde]
pub struct PoolAppObject {
    pub name: String,
    pub description: Option<String>,
    pub market: PoolMarketInfo,
    pub token: Token,
    pub amount: Uint128,
}

#[cw_serde]
pub struct PoolsResponse {
    pub pools: Vec<PoolAppObject>,
}

#[cw_serde]
pub struct PoolBalanceAppObject {
    pub pool: PoolId,
    pub balance: Uint128,
}

#[cw_serde]
pub struct AccountResponse {
    pub balances: Vec<PoolBalanceAppObject>,
}
