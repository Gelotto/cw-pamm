use cosmwasm_std::{StdResult, Storage, Uint64};
use pamp::math::add_u64;

use super::storage::SUBMSG_REPLY_ID_COUNTER;

pub fn get_and_increment_reply_id(store: &mut dyn Storage) -> StdResult<u64> {
    Ok(SUBMSG_REPLY_ID_COUNTER
        .update(store, |n| -> StdResult<Uint64> {
            Ok(add_u64(n, Uint64::one())?)
        })?
        .u64()
        - 1)
}
