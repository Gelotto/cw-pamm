use crate::error::ContractError;
use cosmwasm_std::{Order, StdResult};

use crate::{
    msg::{PoolBizObject, PoolsResponse},
    state::{
        models::{Pool, PoolInfo},
        storage::{PoolId, MARKET_STATS, POOLS, POOL_INFOS, POOL_STATS},
    },
};

use super::ReadonlyContext;

pub fn query_pools(ctx: ReadonlyContext) -> Result<PoolsResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    // TODO: get this by querying the associated jury contract!
    let winning_pool_id: PoolId = 0;

    let stats = MARKET_STATS.load(deps.storage)?;

    let mut pools: Vec<PoolBizObject> = Vec::with_capacity(2);

    for result in POOLS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<Vec<StdResult<_>>>()
    {
        let (
            pool_id,
            Pool {
                reserves,
                offset,
                supply,
                ..
            },
        ) = result?;

        let PoolInfo {
            name,
            description,
            image,
            symbol,
        } = POOL_INFOS.load(deps.storage, pool_id)?;

        let pool_stats = POOL_STATS.load(deps.storage, pool_id)?;

        pools.push(PoolBizObject {
            id: pool_id,
            winner: winning_pool_id == pool_id,
            stats: pool_stats,
            symbol,
            name,
            description,
            image,
            reserves,
            offset,
            supply,
        });
    }

    Ok(PoolsResponse { pools, stats })
}
