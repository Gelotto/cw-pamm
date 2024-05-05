use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, Uint128, Uint256};
use pamp::{
    error::ContractError,
    math::{add_u128, add_u32, div_u256, sub_u128},
};

use super::storage::{PoolId, N_ACCOUNTS, POOLS, POOL_ACCOUNTS};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct PoolReserves {
    pub base: Uint128,
    pub quote: Uint128,
}

#[cw_serde]
pub struct Pool {
    pub reserves: PoolReserves,
    pub k: Uint256,
}

impl Pool {
    pub fn load(
        store: &dyn Storage,
        id: PoolId,
    ) -> Result<Self, ContractError> {
        Ok(POOLS.load(store, id)?)
    }

    pub fn buy(
        &mut self,
        in_amount: Uint128,
    ) -> Result<Uint128, ContractError> {
        self.swap(in_amount, true)
    }

    pub fn sell(
        &mut self,
        in_amount: Uint128,
    ) -> Result<Uint128, ContractError> {
        self.swap(in_amount, false)
    }

    pub fn swap(
        &mut self,
        in_amount: Uint128,
        is_buy: bool,
    ) -> Result<Uint128, ContractError> {
        let (new_quote_reserve, new_base_reserve, out_amount) = if is_buy {
            let new_quote_reserve = add_u128(self.reserves.quote, in_amount)?;
            let new_base_reserve = div_u256(self.k, new_quote_reserve)?.try_into().unwrap();
            let out_amount = sub_u128(self.reserves.base, new_base_reserve)?;
            (new_quote_reserve, new_base_reserve, out_amount)
        } else {
            let new_base_reserve = add_u128(self.reserves.base, in_amount)?;
            let new_quote_reserve = div_u256(self.k, new_base_reserve)?.try_into().unwrap();
            let out_amount = sub_u128(self.reserves.quote, new_quote_reserve)?;
            (new_quote_reserve, new_base_reserve, out_amount)
        };
        self.reserves.base = new_base_reserve;
        self.reserves.quote = new_quote_reserve;
        Ok(out_amount)
    }
}

#[cw_serde]
pub struct PoolInfo {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[cw_serde]
pub struct PoolAccount {
    pub balance: Uint128,
}

impl PoolAccount {
    pub fn upsert(
        store: &mut dyn Storage,
        owner: &Addr,
        pool_id: PoolId,
        balance_delta: Uint128,
        is_positive_delta: bool,
    ) -> Result<Self, ContractError> {
        // Create or insert PoolAccount
        let account = POOL_ACCOUNTS.update(
            store,
            (owner, pool_id),
            |maybe_account| -> Result<_, ContractError> {
                if let Some(mut account) = maybe_account {
                    account.balance = (if is_positive_delta {
                        add_u128(account.balance, balance_delta)
                    } else {
                        sub_u128(account.balance, balance_delta)
                    })?;
                    Ok(account)
                } else {
                    Ok(Self {
                        balance: balance_delta,
                    })
                }
            },
        )?;

        Ok(account)
    }
}
