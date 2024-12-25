use cosmwasm_schema::cw_serde;
use cosmwasm_std::Response;

use crate::{contract::ContractResult, error::ContractError};

pub trait Append {
    fn append(self, other: &mut Self) -> Self;
}

#[cw_serde]
pub struct Pagination<T> {
    pub start_after: Option<T>,
    pub limit: Option<u32>,
}

impl Append for Response {
    fn append(self, other: &mut Self) -> Self {
        let mut ret = self;
        ret.messages.append(&mut other.messages);
        ret.attributes.append(&mut other.attributes);
        ret.events.append(&mut other.events);
        ret
    }
}

pub fn assert(v: bool, err: ContractError) -> ContractResult<()> {
    if v {
        Ok(())
    } else {
        Err(err)
    }
}
