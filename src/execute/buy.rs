use crate::{
    error::ContractError,
    math::{add_u128, add_u32, mul_ratio_u128, sub_u128},
    token::Token,
};
use crate::{
    msg::BuyParams,
    state::{
        models::{Market, MarketAccount, TraderInfo, TraderStats},
        storage::{
            BUY_FEE_PCT, FEE_MANAGER_ADDR, GLOBAL_STATS, MARKETS, MARKET_STATS, QUOTE_TOKEN,
            TRADER_INFOS,
        },
    },
};
use cosmwasm_std::{attr, Coin, Response, Uint128};

use super::Context;

pub fn exec_buy(
    ctx: Context,
    params: BuyParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let BuyParams { market: market_id } = params;
    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let mut resp = Response::new().add_attribute("action", "buy");
    let mut market = Market::load(deps.storage, market_id)?;
    let mut in_amount = [Uint128::zero()];

    if let Token::Denom(denom) = &quote_token {
        if let Some(Coin { amount, .. }) = info.funds.iter().find(|c| c.denom == *denom) {
            in_amount[0] = *amount;

            let fee_amount =
                mul_ratio_u128(*amount, BUY_FEE_PCT.load(deps.storage)?, 1_000_000u128)?;

            let in_amount_post_fee = sub_u128(*amount, fee_amount)?;

            let out_amount = market.buy(in_amount_post_fee)?;

            let account =
                MarketAccount::upsert(deps.storage, &info.sender, market_id, out_amount, true)?;

            resp = resp
                .add_submessage(
                    quote_token.transfer(&FEE_MANAGER_ADDR.load(deps.storage)?, fee_amount)?,
                )
                .add_attributes(vec![
                    attr("account_balance", account.balance.u128().to_string()),
                    attr("in_amount", in_amount_post_fee.u128().to_string()),
                    attr("market_id", market_id.to_string()),
                    attr("out_amount", out_amount.u128().to_string()),
                    attr("fee_amount", fee_amount.u128().to_string()),
                ]);
        } else {
            return Err(ContractError::InsufficientFunds { msg: "".to_owned() });
        }
    } else {
        return Err(ContractError::NotAuthorized { msg: "".to_owned() });
    }

    // Upsert a TraderInfo for tx sender
    let trader = TRADER_INFOS.update(
        deps.storage,
        &info.sender,
        |maybe_trader_info| -> Result<_, ContractError> {
            if let Some(mut trader_info) = maybe_trader_info {
                trader_info.stats.num_buys = add_u32(trader_info.stats.num_buys, 1)?;
                trader_info.stats.quote_amount_in =
                    add_u128(trader_info.stats.quote_amount_in, in_amount[0])?;
                Ok(trader_info)
            } else {
                Ok(TraderInfo {
                    stats: TraderStats {
                        amount_claimed: Uint128::zero(),
                        quote_amount_out: Uint128::zero(),
                        quote_amount_in: in_amount[0],
                        num_sells: 0,
                        num_buys: 1,
                    },
                })
            }
        },
    )?;

    // Increment global trader count if this is a new account
    if trader.stats.num_buys == 1 {
        GLOBAL_STATS.update(deps.storage, |mut stats| -> Result<_, ContractError> {
            stats.num_traders = add_u32(stats.num_traders, 1)?;
            Ok(stats)
        })?;
    }

    MARKET_STATS.update(
        deps.storage,
        market_id,
        |maybe_stats| -> Result<_, ContractError> {
            if let Some(mut stats) = maybe_stats {
                stats.quote_amount_in = add_u128(stats.quote_amount_in, in_amount[0])?;
                stats.num_buys = add_u32(stats.num_buys, 1)?;
                if trader.stats.num_buys == 1 {
                    stats.num_traders = add_u32(stats.num_traders, 1)?;
                }
                Ok(stats)
            } else {
                return Err(ContractError::NotAuthorized {
                    msg: format!("could not load stats for market {}", market_id),
                });
            }
        },
    )?;

    MARKETS.save(deps.storage, market_id, &market)?;

    Ok(resp)
}
