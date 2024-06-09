pub mod models;
pub mod storage;
pub mod utils;

use crate::{error::ContractError, math::mul_u256};
use cosmwasm_std::{Response, Uint128, Uint256};
use storage::{OPERATOR_ADDR, QUOTE_DECIMALS, QUOTE_SYMBOL, SELL_FEE_PCT};

use crate::{
    execute::Context,
    msg::{InstantiateMsg, PoolInitArgs, PoolStats},
};

use self::{
    models::{MarketStats, Pool, PoolInfo},
    storage::{
        PoolId, AMOUNT_CLAIMED, BUY_FEE_PCT, FEE_MANAGER_ADDR, MARKET_STATS, POOLS, POOL_INFOS,
        POOL_STATS, QUOTE_TOKEN, START_TIME, STOP_TIME, SWAP_FEE_PCT,
    },
};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let InstantiateMsg {
        start,
        stop,
        pools,
        quote_token,
        quote_decimals,
        quote_symbol,
        operator,
        fees,
    } = msg;

    AMOUNT_CLAIMED.save(deps.storage, &Uint128::zero())?;
    QUOTE_TOKEN.save(deps.storage, quote_token)?;
    QUOTE_DECIMALS.save(deps.storage, quote_decimals)?;
    QUOTE_SYMBOL.save(deps.storage, quote_symbol)?;
    BUY_FEE_PCT.save(deps.storage, &fees.pct_buy)?;
    SELL_FEE_PCT.save(deps.storage, &fees.pct_sell)?;
    SWAP_FEE_PCT.save(deps.storage, &fees.pct_swap)?;
    START_TIME.save(deps.storage, &start)?;
    STOP_TIME.save(deps.storage, stop)?;

    if let Some(operator) = operator {
        OPERATOR_ADDR.save(deps.storage, &deps.api.addr_validate(operator.as_str())?)?;
    }

    MARKET_STATS.save(
        deps.storage,
        &MarketStats {
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
    ) in pools.iter().take(PoolId::MAX as usize).enumerate()
    {
        let pool_id: PoolId = i as PoolId;

        POOLS.save(
            deps.storage,
            pool_id,
            &Pool {
                reserves: reserves.to_owned(),
                offset: reserves.quote,
                supply: reserves.base,
                k: mul_u256(reserves.base, reserves.quote)?,
            },
        )?;

        POOL_STATS.save(
            deps.storage,
            pool_id,
            &PoolStats {
                fees_collected: Uint128::zero(),
                quote_amount_in: Uint256::zero(),
                quote_amount_out: Uint256::zero(),
                base_amount_in: Uint256::zero(),
                base_amount_out: Uint256::zero(),
                num_traders: 0,
                num_buys: 0,
                num_sells: 0,
            },
        )?;

        POOL_INFOS.save(
            deps.storage,
            pool_id,
            &PoolInfo {
                description: description.to_owned(),
                symbol: symbol.to_owned(),
                name: name.to_owned(),
                image: image.to_owned(),
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
