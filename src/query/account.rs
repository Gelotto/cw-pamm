use cosmwasm_std::{Addr, Order};
use pamp::error::ContractError;

use crate::{
    msg::{AccountResponse, PoolBalanceAppObject},
    state::storage::POOL_ACCOUNTS,
};

use super::ReadonlyContext;

pub fn query_account(
    ctx: ReadonlyContext,
    address: Addr,
) -> Result<AccountResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let mut balances: Vec<PoolBalanceAppObject> = Vec::with_capacity(2);

    // Collect account's balances in each pool
    for result in POOL_ACCOUNTS
        .prefix(&address)
        .range(deps.storage, None, None, Order::Ascending)
    {
        let (pool_id, pool_account) = result?;
        balances.push(PoolBalanceAppObject {
            pool: pool_id,
            balance: pool_account.balance,
        })
    }

    Ok(AccountResponse { balances })
}
