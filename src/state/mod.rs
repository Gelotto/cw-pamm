pub mod models;
pub mod storage;
pub mod utils;

use crate::{error::ContractError, math::mul_u256};
use cosmwasm_std::{Response, Uint128, Uint256};
use storage::SELL_FEE_PCT;

use crate::{
    execute::Context,
    msg::{InstantiateMsg, PoolInitArgs, PoolStats},
};

use self::{
    models::{GlobalStats, Pool, PoolInfo},
    storage::{
        PoolId, AMOUNT_CLAIMED, BUY_FEE_PCT, FEE_MANAGER_ADDR, MARKET_STATS, POOLS, POOL_INFOS,
        POOL_STATS, QUOTE_TOKEN, START_TIME, STOP_TIME, SWAP_FEE_PCT,
    },
};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;
    let InstantiateMsg {
        t_open,
        t_close,
        pools,
        quote,
        fees,
    } = msg;

    AMOUNT_CLAIMED.save(deps.storage, &Uint128::zero())?;
    QUOTE_TOKEN.save(deps.storage, quote)?;
    BUY_FEE_PCT.save(deps.storage, &fees.pct_buy)?;
    SELL_FEE_PCT.save(deps.storage, &fees.pct_sell)?;
    SWAP_FEE_PCT.save(deps.storage, &fees.pct_swap)?;
    START_TIME.save(deps.storage, &t_open.clone().unwrap_or(env.block.time))?;
    STOP_TIME.save(deps.storage, t_close)?;

    MARKET_STATS.save(
        deps.storage,
        &GlobalStats {
            amount_claimed: Uint128::zero(),
            num_traders: 0,
        },
    )?;

    if let Some(addr) = &fees.manager {
        FEE_MANAGER_ADDR.save(deps.storage, addr)?;
    } else {
        FEE_MANAGER_ADDR.save(deps.storage, &info.sender)?;
    }

    for (
        i,
        PoolInitArgs {
            symbol,
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
                offset: reserves.quote,
                supply: reserves.base,
                k: mul_u256(reserves.base, reserves.quote)?,
            },
        )?;

        POOL_STATS.save(
            deps.storage,
            pool_id,
            &PoolStats {
                quote_amount_in: Uint256::zero(),
                quote_amount_out: Uint256::zero(),
                base_amount_in: Uint256::zero(),
                base_amount_out: Uint256::zero(),
                num_traders: 0,
                num_buys: 0,
                num_sells: 0,
            },
        )?;

        POOL_INFOS.save(
            deps.storage,
            pool_id,
            &PoolInfo {
                description: description.to_owned(),
                symbol: symbol.to_owned(),
                name: name.to_owned(),
                image: image.to_owned(),
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
