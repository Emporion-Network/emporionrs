use cosmwasm_std::{entry_point, Deps, Empty, QueryResponse};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Review;

pub type ContractResult<T> = Result<T, ContractError>;
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TEXT_MAX_BYTE_SIZE: usize = 4000;
pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    Review::instaniate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    use ExecuteMsg::*;
    match msg {
        ReviewCreate(msg) => Review::exec_create(deps, env, info, msg),
        ReviewUpdate(msg) => Review::exec_update(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<QueryResponse> {
    match msg {
        QueryMsg::ReviewGet(msg) => Review::query_review(deps, env, msg),
        QueryMsg::ReviewList(msg) => Review::query_reviews(deps, env, msg),
        QueryMsg::ReviewByReviewer(msg) => Review::query_reviews_by_reviewer(deps, env, msg),
        QueryMsg::ReviewsByReviewed(msg) => Review::query_reviews_by_reviewed(deps, env, msg),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> ContractResult<Response> {
    Ok(Response::default())
}
