use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("InvalidReplyId: {id:?}")]
    InvalidReplyId { id: u64 },

    #[error("NotAuthorized: {msg:?}")]
    NotAuthorized { msg: String },

    #[error("ValidationError: {msg:?}")]
    ValidationError { msg: String },

    #[error("InternalError: {msg:?}")]
    InternalError { msg: String },

    #[error("NotImplemented: {msg:?}")]
    NotImplemented { msg: String },

    #[error("InsufficientFunds: {msg:?}")]
    InsufficientFunds { msg: String },

    #[error("InsufficientBalance: {msg:?}")]
    InsufficientBalance { msg: String },
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        StdError::generic_err(err.to_string())
    }
}
