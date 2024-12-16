use cosmwasm_std::{entry_point, Deps, Empty, QueryResponse};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub type ContractResult<T> = Result<T, ContractError>;
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    todo!()
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
        _ => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<QueryResponse> {
    use QueryMsg::*;
    match msg {
        _ => todo!(),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> ContractResult<Response> {
    Ok(Response::default())
}
