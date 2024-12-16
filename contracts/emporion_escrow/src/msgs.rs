use cosmwasm_schema::{cw_serde, QueryResponses};
use cw20::Cw20ReceiveMsg;

use crate::state::{Escrow, EscrowUnchecked};

///////////////////////
///   ExecuteMsg    ///
///////////////////////

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct EscrowFundCw20 {
    pub  id:u64
}


#[cw_serde]
pub enum ExecuteMsg {
    EscrowCreate(EscrowUnchecked),
    EscrowFullfil(u64),
    EscrowCancel(u64),
    EscrowSplit(u64),
    EscrowFund(u64),
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub struct Pagination<T> {
    pub start_after: Option<T>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct EscrowListFor {
    pub addr:String,
    pub pagination:Pagination<u64>
}

#[cw_serde]
pub struct EscrowListResp {
    pub escrows:Vec<Escrow>
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Escrow)]
    EscrowGet(u64),
    #[returns(EscrowListResp)]
    EscrowList(Pagination<u64>),
    #[returns(EscrowListResp)]
    EscrowListFor(EscrowListFor)
}
