use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};
use pamp::tokens::Token;

use super::models::{Config, Pool, PoolAccount, PoolInfo};

pub type PoolId = u8;

pub const CONFIG: Item<Config> = Item::new("config");
pub const QUOTE_TOKEN: Item<Token> = Item::new("quote_token");
pub const MARKET_CLOSE: Item<Timestamp> = Item::new("market_close");
pub const HAS_CLAIMED: Map<&Addr, bool> = Map::new("has_claimed");
pub const POOLS: Map<PoolId, Pool> = Map::new("pools");
pub const POOL_INFOS: Map<PoolId, PoolInfo> = Map::new("pool_infos");
pub const POOL_ACCOUNTS: Map<(&Addr, PoolId), PoolAccount> = Map::new("pool_accounts");
pub const N_ACCOUNTS: Item<u32> = Item::new("n_accounts");
