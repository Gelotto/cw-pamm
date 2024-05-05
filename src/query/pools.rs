use cosmwasm_std::{Order, StdResult};
use pamp::error::ContractError;

use crate::{
    msg::{PoolBizObject, PoolsResponse},
    state::{
        models::{Pool, PoolInfo},
        storage::{PoolId, POOLS, POOL_INFOS},
    },
};

use super::ReadonlyContext;

pub fn query_pools(ctx: ReadonlyContext) -> Result<PoolsResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let mut pools: Vec<PoolBizObject> = Vec::with_capacity(2);

    // TODO: get this by querying the associated jury contract!
    let winning_pool_id: PoolId = 0;

    for result in POOLS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<Vec<StdResult<_>>>()
    {
        let (pool_id, Pool { reserves, .. }) = result?;

        let PoolInfo {
            name,
            description,
            image,
        } = POOL_INFOS.load(deps.storage, pool_id)?;

        pools.push(PoolBizObject {
            id: pool_id,
            winner: winning_pool_id == pool_id,
            name,
            description,
            image,
            reserves,
        });
    }

    Ok(PoolsResponse { pools })
}
