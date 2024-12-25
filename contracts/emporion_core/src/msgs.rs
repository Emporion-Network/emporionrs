use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw,
    Deposit,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(())]
    Some,
    #[returns(())]
    Errors,
}

#[cw_serde]
pub struct InstantiateMsg {}
