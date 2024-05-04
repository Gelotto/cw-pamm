use cw_storage_plus::{Item, Map};

use super::models::{Config, Pool, PoolInfo};

pub type PoolId = u8;

pub const CONFIG: Item<Config> = Item::new("config");
pub const POOLS: Map<PoolId, Pool> = Map::new("pools");
pub const POOL_INFOS: Map<PoolId, PoolInfo> = Map::new("pool_infos");
