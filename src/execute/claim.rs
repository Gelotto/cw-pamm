use crate::state::storage::{
    PoolId, AMOUNT_CLAIMED, HAS_CLAIMED, POOLS, POOL_ACCOUNTS, QUOTE_TOKEN, STOP_TIME, TRADER_INFOS,
};
use crate::{
    error::ContractError,
    math::{add_u128, mul_ratio_u128, sub_u128},
};
use cosmwasm_std::{attr, Order, Response, Uint128};

use super::Context;

pub fn exec_claim(ctx: Context) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;

    // TODO: get this by querying the associated jury contract!
    let winning_pool_id: PoolId = 0;

    // Ensure that the pool close time has been reached.
    if env.block.time <= STOP_TIME.load(deps.storage)? {
        return Err(ContractError::NotAuthorized {
            msg: "the pool is still actively trading".to_owned(),
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
                    msg: "already claimed".to_owned(),
                });
            }
            Ok(true)
        },
    )?;

    // Ensure the user account has a non-zero balance in the winning pool
    let account = if let Some(account) =
        POOL_ACCOUNTS.may_load(deps.storage, (&info.sender, winning_pool_id))?
    {
        if account.balance.is_zero() {
            return Err(ContractError::NotAuthorized {
                msg: "nothing to claim".to_owned(),
            });
        }
        account
    } else {
        return Err(ContractError::NotAuthorized {
            msg: "nothing to claim".to_owned(),
        });
    };

    // Compute net_winnings and pool_balance.
    let mut pool_balance = Uint128::zero(); // base balance of winning pool
    let mut net_winnings = Uint128::zero(); // total quote balance across all pools

    for result in POOLS.range(deps.storage, None, None, Order::Ascending) {
        let (pool_id, pool) = result?;
        net_winnings = add_u128(net_winnings, sub_u128(pool.reserves.quote, pool.offset)?)?;
        if pool_id == winning_pool_id {
            pool_balance = sub_u128(pool.supply, pool.reserves.base)?;
        }
    }

    // Compute the account's claim amount.
    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let claim_amount = mul_ratio_u128(net_winnings, account.balance, pool_balance)?;

    // Increment the trader's running total amount claimed
    TRADER_INFOS.update(
        deps.storage,
        &info.sender,
        |maybe_info| -> Result<_, ContractError> {
            if let Some(mut info) = maybe_info {
                info.stats.amount_claimed = add_u128(info.stats.amount_claimed, claim_amount)?;
                Ok(info)
            } else {
                Err(ContractError::NotAuthorized {
                    msg: "No trader associated with sender".to_owned(),
                })
            }
        },
    )?;

    // Increment global amount claimed
    AMOUNT_CLAIMED.update(deps.storage, |n| -> Result<_, ContractError> {
        add_u128(n, claim_amount)
    })?;

    Ok(Response::new()
        .add_submessage(quote_token.transfer(&info.sender, claim_amount)?)
        .add_attributes(vec![
            attr("action", "claim"),
            attr("claim_amount", claim_amount.u128().to_string()),
        ]))
}
