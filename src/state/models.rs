use crate::{
    error::ContractError,
    math::{add_u128, add_u32, div_u256, mul_ratio_u128, sub_u128},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, Timestamp, Uint128, Uint256, Uint64};

use super::storage::{PoolId, POOLS, POOL_ACCOUNTS, POOL_OHLC_BARS};

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
    pub offset: Uint128,
    pub supply: Uint128,
    pub k: Uint256,
}

impl Pool {
    pub fn load(
        store: &dyn Storage,
        id: PoolId,
    ) -> Result<Self, ContractError> {
        Ok(POOLS.load(store, id)?)
    }

    /// Buy trades quote tokens for buy-in to the pool.
    pub fn buy(
        &mut self,
        in_amount: Uint128,
    ) -> Result<Uint128, ContractError> {
        self.swap(in_amount, true)
    }

    /// Sell trades back pool buy-in for quote tokens.
    pub fn sell(
        &mut self,
        in_amount: Uint128,
    ) -> Result<Uint128, ContractError> {
        self.swap(in_amount, false)
    }

    /// Swap is for reapportioning buy-in between pools, distinct from "buy" and
    /// "sell", which deal with swapping quote tokens in/out of the contract.
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

    pub fn calc_quote_price(
        &self,
        quote_decimals: u8,
    ) -> Result<Uint128, ContractError> {
        mul_ratio_u128(
            self.reserves.quote,
            10u128.pow(quote_decimals as u32),
            self.reserves.base,
        )
    }
}

#[cw_serde]
pub struct PoolInfo {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[cw_serde]
pub struct MarketStats {
    pub amount_claimed: Uint128,
    pub num_traders: u32,
}

#[cw_serde]
pub struct TraderStats {
    pub amount_claimed: Uint128,
    pub quote_amount_in: Uint128,
    pub quote_amount_out: Uint128,
    pub num_buys: u32,
    pub num_sells: u32,
}

#[cw_serde]
pub struct TraderInfo {
    pub stats: TraderStats,
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

#[cw_serde]
pub struct OhlcBar {
    pub o: Uint128,
    pub c: Uint128,
    pub h: Uint128,
    pub l: Uint128,
    pub v: Uint128,
    pub t: Uint64,
    pub n: u32,
}

impl OhlcBar {
    pub fn new(t: Uint64) -> Self {
        Self {
            o: Uint128::zero(),
            h: Uint128::zero(),
            l: Uint128::zero(),
            c: Uint128::zero(),
            v: Uint128::zero(),
            n: 0,
            t,
        }
    }

    pub fn upsert(
        store: &mut dyn Storage,
        pool_id: PoolId,
        time: Timestamp,
        price: Uint128,
        amount: Uint128,
    ) -> Result<OhlcBar, ContractError> {
        let seconds = time.seconds();
        let t = seconds - (seconds % 60);
        POOL_OHLC_BARS.update(
            store,
            (pool_id, t),
            |maybe_bar| -> Result<_, ContractError> {
                let mut bar = maybe_bar.unwrap_or_else(|| OhlcBar::new(t.into()));
                if bar.n > 0 {
                    if price > bar.h {
                        bar.h = price;
                    }
                    if price < bar.l {
                        bar.l = price;
                    }
                } else {
                    bar.o = price;
                    bar.h = price;
                    bar.l = price;
                }
                bar.c = price;
                bar.v = add_u128(bar.v, amount)?;
                bar.n = add_u32(bar.n, 1)?;
                Ok(bar)
            },
        )
    }
}
