use crate::{
    error::ContractError,
    math::{add_u128, add_u32, mul_pct_u128, sub_u128},
    msg::{PoolAmount, SellParams},
    state::storage::{POOL_ACCOUNTS, POOL_STATS, SELL_FEE_PCT},
};
use crate::{
    math::add_u256,
    state::{
        models::Pool,
        storage::{POOLS, QUOTE_TOKEN},
    },
};
use cosmwasm_std::{attr, Response, Uint128};

use super::Context;

pub fn exec_sell(
    ctx: Context,
    params: SellParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let SellParams { amounts } = params;
    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let seller = info.sender;

    let mut total_in_amount = Uint128::zero();
    let mut total_out_amount = Uint128::zero();

    for PoolAmount { pool_id, amount } in amounts.iter() {
        let pool_id = *pool_id;
        let in_amount = *amount;

        POOL_ACCOUNTS.update(
            deps.storage,
            (&seller, pool_id),
            |maybe_account| -> Result<_, ContractError> {
                // Ensure seller has required min balance in pool
                if let Some(mut account) = maybe_account {
                    if account.balance < in_amount {
                        return Err(ContractError::InsufficientFunds {
                            msg: format!("insufficient balance in pool {}", pool_id),
                        });
                    }
                    account.balance = sub_u128(account.balance, in_amount)?;
                    Ok(account)
                } else {
                    Err(ContractError::NotAuthorized {
                        msg: format!("account not found for pool {}", pool_id),
                    })
                }
            },
        )?;

        let mut pool = Pool::load(deps.storage, pool_id)?;
        let out_amount = pool.sell(in_amount)?;

        POOLS.save(deps.storage, pool_id, &pool)?;

        // Update statistics pertaining specifically to this pool
        POOL_STATS.update(
            deps.storage,
            pool_id,
            |maybe_stats| -> Result<_, ContractError> {
                if let Some(mut stats) = maybe_stats {
                    stats.quote_amount_out = add_u256(stats.quote_amount_out, out_amount)?;
                    stats.base_amount_in = add_u256(stats.base_amount_in, in_amount)?;
                    stats.num_sells = add_u32(stats.num_buys, 1)?;
                    Ok(stats)
                } else {
                    return Err(ContractError::NotAuthorized {
                        msg: format!("could not load stats for pool {}", pool_id),
                    });
                }
            },
        )?;

        total_in_amount = add_u128(total_in_amount, in_amount)?;
        total_out_amount = add_u128(total_out_amount, out_amount)?;
    }

    let fee_pct = SELL_FEE_PCT.load(deps.storage)?;
    let fee_amount = mul_pct_u128(total_out_amount, fee_pct)?;
    let total_out_amount_post_fee = sub_u128(total_out_amount, fee_amount)?;

    Ok(Response::new()
        .add_submessage(quote_token.transfer(&seller, total_out_amount_post_fee)?)
        .add_attributes(vec![
            attr("action", "sell"),
            attr("fee_amount", fee_amount.u128().to_string()),
            attr("in_amount", total_in_amount.u128().to_string()),
            attr("out_amount", total_out_amount.u128().to_string()),
        ]))
}
