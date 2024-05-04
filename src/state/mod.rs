pub mod models;
pub mod storage;

use cosmwasm_std::{Response, Uint128};

use crate::{
    error::ContractError,
    execute::Context,
    msg::{InstantiateMsg, PoolInitArgs},
};

use self::{
    models::{Pool, PoolInfo},
    storage::{PoolId, POOLS, POOL_INFOS},
};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let InstantiateMsg { pools } = msg;

    for (
        i,
        PoolInitArgs {
            market,
            name,
            description,
        },
    ) in pools.iter().take(PoolId::MAX as usize).enumerate()
    {
        let pool_id: PoolId = i as PoolId;
        POOLS.save(
            deps.storage,
            pool_id,
            &Pool {
                amount: Uint128::zero(),
                market: market.to_owned(),
            },
        )?;
        POOL_INFOS.save(
            deps.storage,
            pool_id,
            &PoolInfo {
                description: description.to_owned(),
                name: name.to_owned(),
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
