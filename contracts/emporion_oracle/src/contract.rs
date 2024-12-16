use cosmwasm_std::{entry_point, Deps, QueryResponse};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::msgs::{ExecuteMsg, QueryMsg};
use crate::state::Price;
use crate::{msgs::InstantiateMsg, state::ContractResult};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    Price::instaniate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::Update(msg) => Price::update(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<QueryResponse> {
    match msg {
        QueryMsg::Prices(filter) => Price::query_prices(deps, env, filter),
    }
}
