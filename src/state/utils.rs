use cosmwasm_std::{Addr, Api, Storage};

use crate::error::ContractError;

use super::storage::OPERATOR_ADDR;

/// Return the tx sender address of the initiator address if exists AND the tx
/// sender is the registered "operator" address.
pub fn resolve_initiator(
    store: &dyn Storage,
    api: &dyn Api,
    sender: &Addr,
    maybe_initiator: Option<Addr>,
) -> Result<Addr, ContractError> {
    if let Some(candidate_initiator) = maybe_initiator {
        if let Some(operator_addr) = OPERATOR_ADDR.may_load(store)? {
            if operator_addr == sender {
                return Ok(api.addr_validate(candidate_initiator.as_str())?);
            }
        }
        Err(ContractError::NotAuthorized {
            msg: "Only contract operator can specify initiator".to_owned(),
        })
    } else {
        Ok(sender.clone())
    }
}
