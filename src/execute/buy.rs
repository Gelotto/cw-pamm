use crate::{
    msg::BuyParams,
    state::{
        models::{Pool, PoolAccount, SubMsgJob},
        storage::{PoolId, POOLS, POOL_ACCOUNTS, SUBMSG_JOBS},
        utils::get_and_increment_reply_id,
    },
};
use cosmwasm_std::{attr, Addr, Coin, Response, StdError, SubMsg, Uint128};
use pamp::{
    error::ContractError, market::client::market_buy_native, math::add_u128, tokens::Token,
};

use super::{Context, ReplyContext};

pub fn exec_init_buy(
    ctx: Context,
    params: BuyParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let BuyParams { pool: pool_id } = params;
    let mut resp = Response::new().add_attributes(vec![attr("action", "buy")]);
    let pool = Pool::load(deps.storage, pool_id)?;

    if let Token::Denom(denom) = pool.token {
        if let Some(Coin { amount, .. }) = info.funds.iter().find(|c| c.denom == denom) {
            let wasm_msg = market_buy_native(&pool.market, *amount, &info.funds)?;
            let reply_id = get_and_increment_reply_id(deps.storage)?;
            resp = resp.add_submessage(SubMsg::reply_always(wasm_msg, reply_id));
            SUBMSG_JOBS.save(
                deps.storage,
                reply_id,
                &SubMsgJob::Buy {
                    pool: pool_id,
                    buyer: info.sender.to_owned(),
                },
            )?;
        } else {
            return Err(ContractError::InsufficientFunds { msg: "".to_owned() });
        }
    } else {
        return Err(ContractError::NotAuthorized {
            reason: "".to_owned(),
        });
    }

    Ok(resp)
}

pub fn exec_finalize_buy(
    ctx: ReplyContext,
    pool_id: PoolId,
    buyer_addr: Addr,
) -> Result<Response, ContractError> {
    let ReplyContext { deps, reply, .. } = ctx;
    let resp = reply
        .result
        .into_result()
        .map_err(|e| ContractError::Std(StdError::generic_err(e)))?;

    let mut pool = Pool::load(deps.storage, pool_id)?;

    // Extract the amount of token swapped out of the token AMM and increment
    // the corresponding pool and user account balance here.
    for e in resp.events.iter().filter(|e| e.ty == "wasm") {
        // Skip the event if its _contract_address is not the AMM market addr
        if let Some(a) = e.attributes.iter().find(|a| a.key == "_contract_address") {
            if a.value != pool.market.to_string() {
                continue;
            }
        } else {
            continue;
        }

        if let Some(a) = e.attributes.iter().find(|a| a.key == "out_amount") {
            // Extract the amount of base token swapped out of the market
            let amount: Uint128 = a
                .value
                .parse::<u128>()
                .map_err(|e| ContractError::Std(StdError::generic_err(e.to_string())))?
                .into();

            // Increment the total pool size
            pool.amount = add_u128(pool.amount, amount)?;

            // Increment the account's balance in this pool
            POOL_ACCOUNTS.update(
                deps.storage,
                (&buyer_addr, pool_id),
                |maybe_account| -> Result<_, ContractError> {
                    if let Some(mut account) = maybe_account {
                        account.balance = add_u128(account.balance, amount)?;
                        Ok(account)
                    } else {
                        Ok(PoolAccount { balance: amount })
                    }
                },
            )?;

            POOLS.save(deps.storage, pool_id, &pool)?;

            break;
        }
    }

    Ok(Response::new())
}
