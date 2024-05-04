use cosmwasm_std::{Order, StdResult};

use crate::{
    error::ContractError,
    msg::{PoolAppObject, PoolsResponse},
    state::{
        models::{Pool, PoolInfo},
        storage::{CONFIG, POOLS, POOL_INFOS},
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
        let (pool_id, Pool { token, amount }) = result?;
        let PoolInfo { name, description } = POOL_INFOS.load(deps.storage, pool_id)?;
        pools.push(PoolAppObject {
            name,
            description,
            token,
            amount,
        });
    }
    Ok(PoolsResponse { pools })
}
