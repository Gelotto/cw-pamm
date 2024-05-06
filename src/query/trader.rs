use cosmwasm_std::{Addr, Order};
use crate::error::ContractError;

use crate::{
    msg::{PoolBalance, TraderResponse},
    state::{
        models::TraderInfo,
        storage::{MARKET_ACCOUNTS, TRADER_INFOS},
    },
};

use super::ReadonlyContext;

pub fn query_trader(
    ctx: ReadonlyContext,
    address: Addr,
) -> Result<TraderResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    let TraderInfo { stats } = TRADER_INFOS.load(deps.storage, &address)?;

    // Collect account's balances in each market
    let mut balances: Vec<PoolBalance> = Vec::with_capacity(2);

    for result in MARKET_ACCOUNTS
        .prefix(&address)
        .range(deps.storage, None, None, Order::Ascending)
    {
        let (market_id, pool_account) = result?;
        balances.push(PoolBalance {
            market: market_id,
            amount: pool_account.balance,
        })
    }

    Ok(TraderResponse { balances, stats })
}
