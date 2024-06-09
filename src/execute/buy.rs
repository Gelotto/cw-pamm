use crate::{
    error::ContractError,
    math::{add_u128, add_u256, add_u32, mul_pct_u128, sub_u128},
    msg::{BuyParams, PoolAmount},
    state::{
        models::{OhlcBar, Pool, PoolAccount, TraderInfo, TraderStats},
        storage::{
            BUY_FEE_PCT, FEE_MANAGER_ADDR, MARKET_STATS, POOLS, POOL_STATS, QUOTE_DECIMALS,
            QUOTE_TOKEN, TRADER_INFOS,
        },
        utils::resolve_initiator,
    },
};
use cosmwasm_std::{attr, Response, Uint128};

use super::Context;

pub fn exec_buy(
    ctx: Context,
    params: BuyParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;
    let BuyParams { amounts, initiator } = params;
    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let quote_decimals = QUOTE_DECIMALS.load(deps.storage)?;
    let fee_pct = BUY_FEE_PCT.load(deps.storage)?;
    let buyer = resolve_initiator(deps.storage, deps.api, &info.sender, initiator)?;

    // Total quote amount swapping in
    let total_in_amount: Uint128 = amounts
        .iter()
        .map(|a| a.amount)
        .reduce(|a, b| add_u128(a, b).unwrap_or_default())
        .unwrap_or_default();

    if total_in_amount.is_zero() {
        return Err(ContractError::ValidationError {
            msg: "cannot buy 0 amount".to_owned(),
        });
    }

    if !quote_token.has_in_funds(&info.funds, Some(total_in_amount)) {
        return Err(ContractError::InsufficientFunds {
            msg: "insufficient funds".to_owned(),
        });
    }

    let mut resp = Response::new().add_attribute("action", "buy");
    let mut total_fee_amount = Uint128::zero();
    let mut total_in_amount = Uint128::zero();
    let mut total_out_amount = Uint128::zero();

    for PoolAmount { pool_id, amount } in amounts.iter() {
        let pool_id = *pool_id;
        let amount = *amount;

        let mut pool = Pool::load(deps.storage, pool_id)?;
        let fee_amount = mul_pct_u128(amount, fee_pct)?;
        let in_amount_post_fee = sub_u128(amount, fee_amount)?;

        // Swap in quote token
        let out_amount = pool.buy(in_amount_post_fee)?;

        // Update or create sender's account for specifically this pool
        let account = PoolAccount::upsert(deps.storage, &buyer, pool_id, out_amount, true)?;

        // Agg running totals
        total_fee_amount = add_u128(total_fee_amount, fee_amount)?;
        total_in_amount = add_u128(total_in_amount, in_amount_post_fee)?;
        total_out_amount = add_u128(total_out_amount, out_amount)?;

        POOLS.save(deps.storage, pool_id, &pool)?;

        let price = pool.calc_quote_price(quote_decimals)?;

        // Update statistics pertaining specifically to this pool
        POOL_STATS.update(
            deps.storage,
            pool_id,
            |maybe_stats| -> Result<_, ContractError> {
                if let Some(mut stats) = maybe_stats {
                    stats.quote_amount_in = add_u256(stats.quote_amount_in, in_amount_post_fee)?;
                    stats.base_amount_out = add_u256(stats.base_amount_out, out_amount)?;
                    stats.fees_collected = add_u128(stats.fees_collected, fee_amount)?;
                    stats.num_buys = add_u32(stats.num_buys, 1)?;
                    if account.balance == out_amount {
                        stats.num_traders = add_u32(stats.num_traders, 1)?;
                    }
                    Ok(stats)
                } else {
                    return Err(ContractError::NotAuthorized {
                        msg: format!("could not load stats for pool {}", pool_id),
                    });
                }
            },
        )?;

        // Update or add a historical trading OHLC "candlestick"
        OhlcBar::upsert(deps.storage, pool_id, env.block.time, price, out_amount)?;

        // Add submsg to transfer fee to fee manager account
        resp = resp.add_submessage(
            quote_token.transfer(&FEE_MANAGER_ADDR.load(deps.storage)?, fee_amount)?,
        )
    }

    // Upsert a TraderInfo for tx sender
    let trader = TRADER_INFOS.update(
        deps.storage,
        &buyer,
        |maybe_trader_info| -> Result<_, ContractError> {
            if let Some(mut trader_info) = maybe_trader_info {
                trader_info.stats.num_buys = add_u32(trader_info.stats.num_buys, 1)?;
                trader_info.stats.quote_amount_in =
                    add_u128(trader_info.stats.quote_amount_in, total_in_amount)?;
                Ok(trader_info)
            } else {
                Ok(TraderInfo {
                    stats: TraderStats {
                        amount_claimed: Uint128::zero(),
                        quote_amount_out: Uint128::zero(),
                        quote_amount_in: total_in_amount,
                        num_sells: 0,
                        num_buys: 1,
                    },
                })
            }
        },
    )?;

    // Increment global trader count if this is a new account
    if trader.stats.num_buys == 1 {
        MARKET_STATS.update(deps.storage, |mut stats| -> Result<_, ContractError> {
            stats.num_traders = add_u32(stats.num_traders, 1)?;
            Ok(stats)
        })?;
    }

    Ok(resp.add_attributes(vec![
        attr("fee_amount", total_fee_amount.u128().to_string()),
        attr("in_amount", total_in_amount.u128().to_string()),
        attr("out_amount", total_out_amount.u128().to_string()),
    ]))
}
