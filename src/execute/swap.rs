use crate::{
    msg::SwapParams,
    state::{
        models::{Market, MarketAccount},
        storage::{FEE_MANAGER_ADDR, MARKETS, QUOTE_TOKEN, SWAP_FEE_PCT},
    },
};
use cosmwasm_std::{attr, Response};
use crate::{
    error::ContractError,
    math::{mul_ratio_u128, sub_u128},
};

use super::Context;

pub fn exec_swap(
    ctx: Context,
    params: SwapParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let SwapParams {
        from_amount,
        from_pool: from_pool_id,
        to_pool: to_pool_id,
    } = params;

    let mut from_pool = Market::load(deps.storage, from_pool_id)?;
    let mut to_pool = Market::load(deps.storage, to_pool_id)?;

    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let quote_amount = from_pool.swap(from_amount, false)?;

    let fee_amount = mul_ratio_u128(
        quote_amount,
        SWAP_FEE_PCT.load(deps.storage)?,
        1_000_000u128,
    )?;

    let to_amount = to_pool.swap(sub_u128(quote_amount, fee_amount)?, true)?;

    MarketAccount::upsert(deps.storage, &info.sender, from_pool_id, from_amount, false)?;
    MarketAccount::upsert(deps.storage, &info.sender, to_pool_id, to_amount, true)?;

    MARKETS.save(deps.storage, from_pool_id, &from_pool)?;
    MARKETS.save(deps.storage, to_pool_id, &to_pool)?;

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
