pub mod models;
pub mod storage;
pub mod utils;

use crate::{error::ContractError, math::mul_u256};
use cosmwasm_std::{Response, Uint128};

use crate::{
    execute::Context,
    msg::{InstantiateMsg, MarketStats, PoolInitArgs},
};

use self::{
    models::{GlobalStats, Market, MarketInfo},
    storage::{
        MarketId, AMOUNT_CLAIMED, BUY_FEE_PCT, FEE_MANAGER_ADDR, GLOBAL_STATS, MARKETS,
        MARKET_INFOS, MARKET_STATS, QUOTE_TOKEN, START_TIME, STOP_TIME, SWAP_FEE_PCT,
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
    SWAP_FEE_PCT.save(deps.storage, &fees.pct_swap)?;
    START_TIME.save(deps.storage, &t_open.clone().unwrap_or(env.block.time))?;
    STOP_TIME.save(deps.storage, t_close)?;

    GLOBAL_STATS.save(
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
    ) in pools.iter().take(MarketId::MAX as usize).enumerate()
    {
        let market_id: MarketId = i as MarketId;
        MARKETS.save(
            deps.storage,
            market_id,
            &Market {
                reserves: reserves.to_owned(),
                offset: reserves.quote,
                supply: reserves.base,
                k: mul_u256(reserves.base, reserves.quote)?,
            },
        )?;
        MARKET_STATS.save(
            deps.storage,
            market_id,
            &MarketStats {
                quote_amount_in: Uint128::zero(),
                quote_amount_out: Uint128::zero(),
                num_traders: 0,
                num_buys: 0,
            },
        )?;
        MARKET_INFOS.save(
            deps.storage,
            market_id,
            &MarketInfo {
                description: description.to_owned(),
                symbol: symbol.to_owned(),
                name: name.to_owned(),
                image: image.to_owned(),
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
