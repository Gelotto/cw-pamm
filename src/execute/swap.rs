use crate::{
    msg::SwapParams,
    state::{
        models::{Pool, PoolAccount},
        storage::POOLS,
    },
};
use cosmwasm_std::{attr, Response};
use pamp::error::ContractError;

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

    let mut from_pool = Pool::load(deps.storage, from_pool_id)?;
    let mut to_pool = Pool::load(deps.storage, to_pool_id)?;

    let quote_amount = from_pool.swap(from_amount, false)?;
    let to_amount = to_pool.swap(quote_amount, true)?;

    POOLS.save(deps.storage, from_pool_id, &from_pool)?;
    POOLS.save(deps.storage, to_pool_id, &to_pool)?;

    PoolAccount::upsert(deps.storage, &info.sender, from_pool_id, from_amount, false)?;
    PoolAccount::upsert(deps.storage, &info.sender, from_pool_id, to_amount, true)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "swap"),
        attr("quote_amount", quote_amount.to_string()),
        attr("from_pool_id", from_pool_id.to_string()),
        attr("from_amount", from_amount.to_string()),
        attr("to_pool_id", to_pool_id.to_string()),
        attr("to_amount", to_amount.to_string()),
        attr("from_pool_reserve", from_pool.reserves.base.to_string()),
        attr("to_pool_reserve", to_pool.reserves.base.to_string()),
    ]))
}
