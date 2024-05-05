use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::{Item, Map};

use super::models::{Config, Pool, PoolAccount, PoolInfo, SubMsgJob};

pub type PoolId = u8;

pub const CONFIG: Item<Config> = Item::new("config");
pub const POOLS: Map<PoolId, Pool> = Map::new("pools");
pub const POOL_INFOS: Map<PoolId, PoolInfo> = Map::new("pool_infos");
pub const POOL_ACCOUNTS: Map<(&Addr, PoolId), PoolAccount> = Map::new("pool_accounts");
pub const SUBMSG_REPLY_ID_COUNTER: Item<Uint64> = Item::new("submsg_reply_id_counter");
pub const SUBMSG_JOBS: Map<u64, SubMsgJob> = Map::new("submsg_reply_jobs");
