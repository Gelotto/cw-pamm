use crate::{msg::SwapStats, token::Token};
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

use crate::msg::PoolStats;

use super::models::{Config, GlobalStats, OhlcBar, Pool, PoolAccount, PoolInfo, TraderInfo};

pub type PoolId = u8;

pub const CONFIG: Item<Config> = Item::new("config");
pub const QUOTE_TOKEN: Item<Token> = Item::new("quote_token");
pub const QUOTE_DECIMALS: Item<u8> = Item::new("quote_decimals");
pub const MARKET_STATS: Item<GlobalStats> = Item::new("global_stats");
pub const BUY_FEE_PCT: Item<Uint128> = Item::new("buy_fee_pct");
pub const SELL_FEE_PCT: Item<Uint128> = Item::new("sell_fee_pct");
pub const SWAP_FEE_PCT: Item<Uint128> = Item::new("swap_fee_pct");
pub const FEE_MANAGER_ADDR: Item<Addr> = Item::new("fee_manager_addr");
pub const START_TIME: Item<Timestamp> = Item::new("start_time");
pub const STOP_TIME: Item<Timestamp> = Item::new("stop_time");
pub const HAS_CLAIMED: Map<&Addr, bool> = Map::new("has_claimed");
pub const AMOUNT_CLAIMED: Item<Uint128> = Item::new("amount_claimed");
pub const POOLS: Map<PoolId, Pool> = Map::new("pools");
pub const POOL_INFOS: Map<PoolId, PoolInfo> = Map::new("pool_infos");
pub const POOL_STATS: Map<PoolId, PoolStats> = Map::new("pool_stats");
pub const SWAP_STATS: Map<(PoolId, PoolId), SwapStats> = Map::new("swap_stats");
pub const TRADER_INFOS: Map<&Addr, TraderInfo> = Map::new("trader_infos");
pub const POOL_ACCOUNTS: Map<(&Addr, PoolId), PoolAccount> = Map::new("pool_accounts");
pub const POOL_OHLC_BARS: Map<(PoolId, u64), OhlcBar> = Map::new("pool_ohlc_bars");
