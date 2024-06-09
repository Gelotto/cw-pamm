use crate::{
    error::ContractError,
    math::{add_u128, add_u256, add_u32, mul_ratio_u128, sub_u128},
    msg::SwapStats,
    state::{
        models::OhlcBar,
        storage::{POOL_STATS, QUOTE_DECIMALS, SWAP_STATS},
    },
};
use crate::{
    msg::SwapParams,
    state::{
        models::{Pool, PoolAccount},
        storage::{FEE_MANAGER_ADDR, POOLS, QUOTE_TOKEN, SWAP_FEE_PCT},
    },
};
use cosmwasm_std::{attr, Response, Uint256};

use super::Context;

pub fn exec_swap(
    ctx: Context,
    params: SwapParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;
    let SwapParams {
        from_amount,
        from_pool: from_pool_id,
        to_pool: to_pool_id,
    } = params;

    let mut from_pool = Pool::load(deps.storage, from_pool_id)?;
    let mut to_pool = Pool::load(deps.storage, to_pool_id)?;

    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let quote_decimals = QUOTE_DECIMALS.load(deps.storage)?;
    let quote_amount = from_pool.swap(from_amount, false)?;

    let fee_amount = mul_ratio_u128(
        quote_amount,
        SWAP_FEE_PCT.load(deps.storage)?,
        1_000_000u128,
    )?;

    let to_amount = to_pool.swap(sub_u128(quote_amount, fee_amount)?, true)?;

    PoolAccount::upsert(deps.storage, &info.sender, from_pool_id, from_amount, false)?;
    PoolAccount::upsert(deps.storage, &info.sender, to_pool_id, to_amount, true)?;

    POOLS.save(deps.storage, from_pool_id, &from_pool)?;
    POOLS.save(deps.storage, to_pool_id, &to_pool)?;

    POOL_STATS.update(
        deps.storage,
        from_pool_id,
        |stats| -> Result<_, ContractError> {
            let mut stats = stats.unwrap();
            stats.fees_collected = add_u128(stats.fees_collected, fee_amount)?;
            Ok(stats)
        },
    )?;

    SWAP_STATS.update(
        deps.storage,
        (from_pool_id, to_pool_id),
        |maybe_stats| -> Result<_, ContractError> {
            let mut stats = maybe_stats.unwrap_or_else(|| SwapStats {
                in_amount: Uint256::zero(),
                out_amount: Uint256::zero(),
                n: 0,
            });
            stats.in_amount = add_u256(stats.in_amount, from_amount)?;
            stats.out_amount = add_u256(stats.out_amount, to_amount)?;
            stats.n = add_u32(stats.n, 1)?;
            Ok(stats)
        },
    )?;
    // Update or add a historical trading OHLC "candlestick"
    OhlcBar::upsert(
        deps.storage,
        from_pool_id,
        env.block.time,
        from_pool.calc_quote_price(quote_decimals)?,
        from_amount,
    )?;

    OhlcBar::upsert(
        deps.storage,
        to_pool_id,
        env.block.time,
        to_pool.calc_quote_price(quote_decimals)?,
        to_amount,
    )?;

    Ok(Response::new()
        .add_submessage(quote_token.transfer(&FEE_MANAGER_ADDR.load(deps.storage)?, fee_amount)?)
        .add_attributes(vec![
            attr("action", "swap"),
            attr("quote_amount", quote_amount.to_string()),
            attr("fee_amount", fee_amount.to_string()),
            attr("from_pool_id", from_pool_id.to_string()),
            attr("from_amount", from_amount.to_string()),
            attr("to_pool_id", to_pool_id.to_string()),
            attr("to_amount", to_amount.to_string()),
            attr("from_pool_reserve", from_pool.reserves.base.to_string()),
            attr("to_pool_reserve", to_pool.reserves.base.to_string()),
        ]))
}
