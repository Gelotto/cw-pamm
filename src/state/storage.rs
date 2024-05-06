use crate::token::Token;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

use crate::msg::MarketStats;

use super::models::{Config, GlobalStats, Market, MarketAccount, MarketInfo, TraderInfo};

pub type MarketId = u8;

pub const CONFIG: Item<Config> = Item::new("config");
pub const QUOTE_TOKEN: Item<Token> = Item::new("quote_token");
pub const GLOBAL_STATS: Item<GlobalStats> = Item::new("global_stats");
pub const BUY_FEE_PCT: Item<Uint128> = Item::new("buy_fee_pct");
pub const SWAP_FEE_PCT: Item<Uint128> = Item::new("swap_fee_pct");
pub const FEE_MANAGER_ADDR: Item<Addr> = Item::new("fee_manager_addr");
pub const START_TIME: Item<Timestamp> = Item::new("start_time");
pub const STOP_TIME: Item<Timestamp> = Item::new("stop_time");
pub const HAS_CLAIMED: Map<&Addr, bool> = Map::new("has_claimed");
pub const AMOUNT_CLAIMED: Item<Uint128> = Item::new("amount_claimed");
pub const MARKETS: Map<MarketId, Market> = Map::new("pools");
pub const MARKET_INFOS: Map<MarketId, MarketInfo> = Map::new("market_infos");
pub const MARKET_STATS: Map<MarketId, MarketStats> = Map::new("market_stats");
pub const TRADER_INFOS: Map<&Addr, TraderInfo> = Map::new("trader_infos");
pub const MARKET_ACCOUNTS: Map<(&Addr, MarketId), MarketAccount> = Map::new("market_accounts");
