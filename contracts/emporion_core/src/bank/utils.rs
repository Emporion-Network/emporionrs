use cosmwasm_schema::cw_serde;
use cosmwasm_std::Deps;
use neutron_std::types::neutron::interchaintxs::{self};

use crate::contract::ContractResult;

struct RegisterFeeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}

impl<'a, Q: cosmwasm_std::CustomQuery> RegisterFeeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<interchaintxs::v1::QueryParamsResponse, cosmwasm_std::StdError> {
        interchaintxs::v1::QueryParamsRequest {}.query(self.querier)
    }
}

pub fn query_register_fee(deps: Deps) -> ContractResult<interchaintxs::v1::QueryParamsResponse> {
    let r = RegisterFeeQuerier::new(&deps.querier);
    Ok(r.params()?)
}

#[cw_serde]
pub struct OpenAckVersion {
    pub version: String,
    pub controller_connection_id: String,
    pub host_connection_id: String,
    pub address: String,
    pub encoding: String,
    pub tx_type: String,
}
