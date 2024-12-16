use cosmwasm_std::{entry_point, Deps, QueryResponse};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Escrow;

pub type ContractResult<T> = Result<T, ContractError>;
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    Escrow::instantiate(deps, env, info, msg)
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
        EscrowCreate(msg) => Escrow::exec_create(deps, env, info, msg),
        EscrowFullfil(msg) => Escrow::exec_fulfill(deps, env, info, msg),
        EscrowCancel(msg) => Escrow::exec_cancel(deps, env, info, msg),
        EscrowSplit(msg) => Escrow::exec_split(deps, env, info, msg),
        EscrowFund(msg) => Escrow::exec_fund(deps, env, info, msg),
        Receive(msg) => Escrow::exec_fund_cw20(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<QueryResponse> {
    use QueryMsg::*;
    match msg {
        EscrowGet(msg) => Escrow::query_escrow(deps, env, msg),
        EscrowList(msg) => Escrow::query_escrows(deps, env, msg),
        EscrowListFor(msg) => Escrow::query_escrows_for(deps, env, msg),
    }
}
