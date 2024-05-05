pub mod models;
pub mod storage;
pub mod utils;

use cosmwasm_std::Response;
use pamp::{error::ContractError, math::mul_u256};

use crate::{
    execute::Context,
    msg::{InstantiateMsg, PoolInitArgs},
};

use self::{
    models::{Pool, PoolInfo},
    storage::{PoolId, N_ACCOUNTS, POOLS, POOL_INFOS, QUOTE_TOKEN},
};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let InstantiateMsg { pools, quote_token } = msg;

    QUOTE_TOKEN.save(deps.storage, &quote_token)?;
    N_ACCOUNTS.save(deps.storage, &0)?;

    for (
        i,
        PoolInitArgs {
            name,
            description,
            image,
            reserves,
        },
    ) in pools.iter().take(PoolId::MAX as usize).enumerate()
    {
        let pool_id: PoolId = i as PoolId;
        POOLS.save(
            deps.storage,
            pool_id,
            &Pool {
                reserves: reserves.to_owned(),
                k: mul_u256(reserves.base, reserves.quote)?,
            },
        )?;
        POOL_INFOS.save(
            deps.storage,
            pool_id,
            &PoolInfo {
                description: description.to_owned(),
                name: name.to_owned(),
                image: image.to_owned(),
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
