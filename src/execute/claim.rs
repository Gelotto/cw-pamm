use crate::state::storage::{PoolId, HAS_CLAIMED, MARKET_CLOSE, POOLS, POOL_ACCOUNTS, QUOTE_TOKEN};
use cosmwasm_std::{attr, Order, Response, Uint128};
use pamp::{
    error::ContractError,
    math::{add_u128, mul_ratio_u128},
};

use super::Context;

pub fn exec_claim(ctx: Context) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;

    // Ensure that the market close time has been reached.
    if env.block.time < MARKET_CLOSE.load(deps.storage)? {
        return Err(ContractError::NotAuthorized {
            reason: "the market is still actively trading".to_owned(),
        });
    }

    // Ensure the user has not already claimed.
    HAS_CLAIMED.update(
        deps.storage,
        &info.sender,
        |maybe_has_claimed| -> Result<_, ContractError> {
            let has_claimed = maybe_has_claimed.unwrap_or(false);
            if has_claimed {
                return Err(ContractError::NotAuthorized {
                    reason: "already claimed".to_owned(),
                });
            }
            Ok(true)
        },
    )?;

    // TODO: get this by querying the associated jury contract!
    let winning_pool_id: PoolId = 0;

    let mut pool_balance = Uint128::zero(); // base balance of winning pool
    let mut net_winnings = Uint128::zero(); // total quote balance across all pools

    // Compute net_winnings and pool_balance.
    for result in POOLS.range(deps.storage, None, None, Order::Ascending) {
        let (pool_id, pool) = result?;
        net_winnings = add_u128(net_winnings, pool.reserves.quote)?;
        if pool_id == winning_pool_id {
            pool_balance = pool.reserves.base;
        }
    }

    // Compute the account's claim amount.
    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let account = POOL_ACCOUNTS.load(deps.storage, (&info.sender, winning_pool_id))?;
    let claim_amount = mul_ratio_u128(net_winnings, account.balance, pool_balance)?;

    if claim_amount.is_zero() {
        return Err(ContractError::NotAuthorized {
            reason: "nothing to claim".to_owned(),
        });
    }

    Ok(Response::new()
        .add_submessage(quote_token.transfer(&info.sender, claim_amount)?)
        .add_attributes(vec![
            attr("action", "claim"),
            attr("claim_amount", claim_amount.u128().to_string()),
        ]))
}
