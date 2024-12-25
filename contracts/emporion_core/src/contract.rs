use cosmwasm_std::{entry_point, to_json_binary, Deps, Empty, QueryResponse};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::sudo::msg::SudoMsg;

use crate::bank::bank::Bank;
use crate::error::ContractError;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub type ContractResult<T> = Result<T, ContractError>;
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> ContractResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Bank::instantiate(&mut deps, &env, &info)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::Withdraw => Bank::withdraw(&mut deps, env, "gia"),
        ExecuteMsg::Deposit => Bank::deposit(&mut deps, env, "gia"),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> ContractResult<QueryResponse> {
    // use QueryMsg::*;
    let r = match msg {
        QueryMsg::Some => Bank::query(deps)?,
        QueryMsg::Errors => Bank::query_errors(deps)?,
    };
    Ok(r)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(mut deps: DepsMut, env: Env, msg: cosmwasm_std::Reply) -> ContractResult<Response> {
    match msg.id {
        crate::bank::reply_cases::CREATED_DENOM | crate::bank::reply_cases::MINTED_DENOM => {
            Bank::reply(&mut deps, env, msg)
        }
        _ => Ok(Response::default()),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> ContractResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn sudo(mut deps: DepsMut, env: Env, msg: SudoMsg) -> ContractResult<Response> {
    Bank::sudo(&mut deps, env, msg)?;
    Ok(Response::new())
}
