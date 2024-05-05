pub mod models;
pub mod storage;
pub mod utils;

use cosmwasm_std::{Addr, Response, Uint128};
use pamp::{
    error::ContractError,
    market::{client::query_market_info, responses::MarketInfoResponse},
    tokens::Token,
};

use crate::{
    execute::Context,
    msg::{InstantiateMsg, PoolInitArgs},
};

use self::{
    models::{Pool, PoolInfo},
    storage::{PoolId, POOLS, POOL_INFOS},
};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let InstantiateMsg { pools } = msg;

    for (
        i,
        PoolInitArgs {
            market,
            name,
            description,
        },
    ) in pools.iter().take(PoolId::MAX as usize).enumerate()
    {
        let pool_id: PoolId = i as PoolId;
        let market_info = query_market_info(deps.querier, market)?;
        POOLS.save(
            deps.storage,
            pool_id,
            &Pool {
                amount: Uint128::zero(),
                market: market.to_owned(),
                token: if market_info.token.cw20 {
                    Token::Address(Addr::unchecked(market_info.token.denom.to_owned()))
                } else {
                    Token::Denom(market_info.token.denom.to_owned())
                },
            },
        )?;
        POOL_INFOS.save(
            deps.storage,
            pool_id,
            &PoolInfo {
                description: description.to_owned(),
                name: name.to_owned(),
            },
        )?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
