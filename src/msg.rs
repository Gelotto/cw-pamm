use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128, Uint256};

use crate::{
    state::{
        models::{Config, MarketStats, PoolReserves, TraderStats},
        storage::PoolId,
    },
    token::Token,
};

#[cw_serde]
pub struct PoolInitArgs {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub reserves: PoolReserves,
}

#[cw_serde]
pub struct FeeInitArgs {
    pub manager: Option<Addr>,
    pub pct_swap: Uint128,
    pub pct_buy: Uint128,
    pub pct_sell: Uint128,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub start: Timestamp,
    pub stop: Timestamp,
    pub quote_token: Token,
    pub quote_decimals: u8,
    pub quote_symbol: String,
    pub pools: Vec<PoolInitArgs>,
    pub fees: FeeInitArgs,
}

#[cw_serde]
pub enum ExecuteMsg {
    Buy(BuyParams),
    Sell(SellParams),
    Swap(SwapParams),
    Claim {},
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Pools {},
    Trader { address: Addr },
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
pub struct PoolAmount {
    pub pool_id: PoolId,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SellParams {
    pub amounts: Vec<PoolAmount>,
}

#[cw_serde]
pub struct BuyParams {
    pub amounts: Vec<PoolAmount>,
}

#[cw_serde]
pub struct ConfigResponse(pub Config);

// #[cw_serde]
// pub struct PoolPoolInfo {
//     pub address: Addr,
//     pub info: PoolInfoResponse,
// }

#[cw_serde]
pub struct PoolStats {
    pub num_buys: u32,
    pub num_sells: u32,
    pub num_traders: u32,
    pub quote_amount_in: Uint256,
    pub quote_amount_out: Uint256,
    pub base_amount_in: Uint256,
    pub base_amount_out: Uint256,
    pub fees_collected: Uint128,
}

#[cw_serde]
pub struct SwapStats {
    pub n: u32,
    pub in_amount: Uint256,
    pub out_amount: Uint256,
}

#[cw_serde]
pub struct PoolBizObject {
    pub id: PoolId,
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub reserves: PoolReserves,
    pub winner: bool,
    pub offset: Uint128,
    pub supply: Uint128,
    pub stats: PoolStats,
}

#[cw_serde]
pub struct PoolsResponse {
    pub pools: Vec<PoolBizObject>,
    pub stats: MarketStats,
}

#[cw_serde]
pub struct PoolBalance {
    pub pool: PoolId,
    pub amount: Uint128,
}

#[cw_serde]
pub struct TraderResponse {
    pub balances: Vec<PoolBalance>,
    pub stats: TraderStats,
}
