use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
};
use cw2::set_contract_version;
