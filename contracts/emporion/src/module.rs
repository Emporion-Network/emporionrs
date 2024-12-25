use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, QueryResponse, Reply, Response};

use crate::contract::ContractResult;

pub trait Module {
    type InstantiateMsg;
    type ExecuteMsg;
    type QueryMsg;
    type SudoMsg;
    const REPLY_RANGE_START: u64;
    const REPLY_RANGE_END: u64;

    fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Self::InstantiateMsg,
    ) -> ContractResult<Response>;

    fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Self::ExecuteMsg,
    ) -> ContractResult<Response>;

    fn query(deps: Deps, env: Env, msg: Self::QueryMsg) -> ContractResult<QueryResponse>;

    fn sudo(deps: DepsMut, env: Env, msg: Self::SudoMsg) -> ContractResult<QueryResponse>;

    fn reply(deps: DepsMut, env: Env, msg: Reply) -> ContractResult<Response>;
}
