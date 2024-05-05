pub mod buy;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Reply};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

pub struct ReplyContext<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub reply: Reply,
}
