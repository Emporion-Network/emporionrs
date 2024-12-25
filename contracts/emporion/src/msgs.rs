use cosmwasm_schema::cw_serde;

use crate::bank::msgs::Query;

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub enum QueryMsg {
    Bank(Query),
}

#[cw_serde]
pub struct InstantiateMsg {}
