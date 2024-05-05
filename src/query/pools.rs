use cosmwasm_std::{Order, StdResult};
use pamp::{error::ContractError, market::client::query_market_info};

use crate::{
    msg::{PoolAppObject, PoolMarketInfo, PoolsResponse},
    state::{
        models::{Pool, PoolInfo},
        storage::{POOLS, POOL_INFOS},
    },
};

use super::ReadonlyContext;

pub fn query_pools(ctx: ReadonlyContext) -> Result<PoolsResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let mut pools: Vec<PoolAppObject> = Vec::with_capacity(2);
    for result in POOLS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<Vec<StdResult<_>>>()
    {
        let (
            pool_id,
            Pool {
                market: market_addr,
                amount,
                token,
            },
        ) = result?;

        let PoolInfo { name, description } = POOL_INFOS.load(deps.storage, pool_id)?;
        let market_info = query_market_info(deps.querier, &market_addr)?;

        pools.push(PoolAppObject {
            name,
            description,
            token,
            amount,
            market: PoolMarketInfo {
                address: market_addr,
                info: market_info,
            },
        });
    }
    Ok(PoolsResponse { pools })
}
