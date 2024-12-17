use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

use crate::state::{Review, UncheckedReview, UncheckedReviewed};

#[cw_serde]
pub struct Pagination<T> {
    pub start_after: Option<T>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct ReviewsByReviewer {
    pub reviewer: String,
    pub pagination: Pagination<u64>,
}

#[cw_serde]
pub struct ReviewsByReviewered {
    pub reviewed: UncheckedReviewed,
    pub pagination: Pagination<u64>,
}

#[cw_serde]
pub struct ReviewListResp {
    pub reviews: Vec<Review>,
}

#[cw_serde]
pub struct ReviewedListResp {
    pub reviews: Vec<Review>,
    pub avg: (Decimal, Decimal),
}

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
pub enum QueryMsg {
    #[returns(Review)]
    ReviewGet(u64),
    #[returns(ReviewListResp)]
    ReviewList(Pagination<u64>),
    #[returns(ReviewListResp)]
    ReviewByReviewer(ReviewsByReviewer),
    #[returns(ReviewedListResp)]
    ReviewsByReviewed(ReviewsByReviewered),
}

#[cw_serde]
pub struct InstantiateMsg {}
