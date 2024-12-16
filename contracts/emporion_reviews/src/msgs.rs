use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::UncheckedReview;

#[cw_serde]
pub struct ReviewUpdate {
    pub id: u64,
    pub rating: u8,
    pub message: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ReviewCreate(UncheckedReview),
    ReviewUpdate(ReviewUpdate),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct InstantiateMsg {}
