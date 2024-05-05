use crate::{
    msg::BuyParams,
    state::{
        models::{Pool, PoolAccount},
        storage::{POOLS, QUOTE_TOKEN},
    },
};
use cosmwasm_std::{attr, Coin, Response};
use pamp::{error::ContractError, tokens::Token};

use super::Context;

pub fn exec_buy(
    ctx: Context,
    params: BuyParams,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let BuyParams { pool: pool_id } = params;
    let quote_token = QUOTE_TOKEN.load(deps.storage)?;
    let mut pool = Pool::load(deps.storage, pool_id)?;
    let mut attrs = vec![attr("action", "buy")];

    if let Token::Denom(denom) = quote_token {
        if let Some(Coin { amount, .. }) = info.funds.iter().find(|c| c.denom == denom) {
            let out_amount = pool.buy(*amount)?;
            let account =
                PoolAccount::upsert(deps.storage, &info.sender, pool_id, out_amount, true)?;
            attrs.push(attr("account_balance", account.balance.u128().to_string()));
            attrs.push(attr("out_amount", out_amount.u128().to_string()));
        } else {
            return Err(ContractError::InsufficientFunds { msg: "".to_owned() });
        }
    } else {
        return Err(ContractError::NotAuthorized {
            reason: "".to_owned(),
        });
    }

    POOLS.save(deps.storage, pool_id, &pool)?;

    Ok(Response::new().add_attributes(attrs))
}
