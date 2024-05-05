use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use pamp::{market::responses::MarketInfoResponse, tokens::Token};

use crate::state::{
    models::{Config, PoolReserves},
    storage::PoolId,
};

#[cw_serde]
pub struct PoolInitArgs {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub reserves: PoolReserves,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub quote_token: Token,
    pub pools: Vec<PoolInitArgs>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Buy(BuyParams),
    Swap(SwapParams),
    Claim {},
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Pools {},
    Account { address: Addr },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SwapParams {
    pub to_pool: PoolId,
    pub from_pool: PoolId,
    pub from_amount: Uint128,
}

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
pub struct PoolBizObject {
    pub id: PoolId,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub reserves: PoolReserves,
    pub winner: bool,
}

#[cw_serde]
pub struct PoolsResponse {
    pub pools: Vec<PoolBizObject>,
}

#[cw_serde]
pub struct PoolBalanceBizObject {
    pub pool: PoolId,
    pub balance: Uint128,
}

#[cw_serde]
pub struct AccountResponse {
    pub balances: Vec<PoolBalanceBizObject>,
}
