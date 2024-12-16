use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Timestamp};

#[cw_serde]
pub struct PricesResp {
    pub prices: Vec<(String, Decimal)>,
    pub last_updated_at: Timestamp,
}
pub type Filter = Option<Vec<String>>;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PricesResp)]
    Prices(Filter),
}

pub struct InstantiateMsg {
    pub provider_name: String,
}

#[cw_serde]
pub struct MsgUpdate {
    pub prices: Vec<(String, Decimal)>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Update(MsgUpdate),
}
