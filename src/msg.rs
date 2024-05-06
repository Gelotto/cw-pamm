use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

use crate::{
    state::{
        models::{Config, GlobalStats, MarketReserves, TraderStats},
        storage::MarketId,
    },
    token::Token,
};

#[cw_serde]
pub struct PoolInitArgs {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub reserves: MarketReserves,
}

#[cw_serde]
pub struct FeeInitArgs {
    pub manager: Option<Addr>,
    pub pct_swap: Uint128,
    pub pct_buy: Uint128,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub t_open: Option<Timestamp>,
    pub t_close: Timestamp,
    pub quote: Token,
    pub pools: Vec<PoolInitArgs>,
    pub fees: FeeInitArgs,
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
    Markets {},
    Trader { address: Addr },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SwapParams {
    pub to_pool: MarketId,
    pub from_pool: MarketId,
    pub from_amount: Uint128,
}

#[cw_serde]
pub struct BuyParams {
    pub market: MarketId,
}

#[cw_serde]
pub struct ConfigResponse(pub Config);

// #[cw_serde]
// pub struct PoolMarketInfo {
//     pub address: Addr,
//     pub info: MarketInfoResponse,
// }

#[cw_serde]
pub struct MarketStats {
    pub num_buys: u32,
    pub num_traders: u32,
    pub quote_amount_in: Uint128,
    pub quote_amount_out: Uint128,
}

#[cw_serde]
pub struct MarketBizObject {
    pub id: MarketId,
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub reserves: MarketReserves,
    pub winner: bool,
    pub offset: Uint128,
    pub supply: Uint128,
    pub stats: MarketStats,
}

#[cw_serde]
pub struct MarketsResponse {
    pub markets: Vec<MarketBizObject>,
    pub stats: GlobalStats,
}

#[cw_serde]
pub struct PoolBalance {
    pub market: MarketId,
    pub amount: Uint128,
}

#[cw_serde]
pub struct TraderResponse {
    pub balances: Vec<PoolBalance>,
    pub stats: TraderStats,
}
