use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, Decimal, Deps, DepsMut, Empty, Env, MessageInfo, Order, QueryResponse,
    Response, Timestamp, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::{Bound, Item, Map};

use crate::{
    contract::{
        ContractResult, CONTRACT_NAME, CONTRACT_VERSION, DEFAULT_LIMIT, MAX_LIMIT,
        TEXT_MAX_BYTE_SIZE,
    },
    error::ContractError,
    msgs::{
        InstantiateMsg, Pagination, ReviewListResp, ReviewUpdate, ReviewedListResp,
        ReviewsByReviewer, ReviewsByReviewered,
    },
    utils::{assert, is_valid_str},
};

const REVIEWS: Map<u64, Review> = Map::new("reviews");
const REVIEWED_TO_REVIEW: Map<(&[u8], u64), Empty> = Map::new("reviewed_to_review");
const REVIEWER_TO_REVIEW: Map<(&Addr, u64), Empty> = Map::new("reviewer_to_review");
const VALIDATE_REVIEW_CONTRACT: Item<Addr> = Item::new("validate_review_contract");
const AVG_RATINGS: Map<&[u8], (Decimal, Decimal)> = Map::new("avg_ratings");

const NEXT_ID: Item<u64> = Item::new("next_id");

#[cw_serde]
pub enum Reviewed {
    Addr(Addr),
    Id(u64),
}

impl Reviewed {
    pub fn to_key(&self) -> ContractResult<Vec<u8>> {
        Ok(to_json_binary(&self)?.to_vec())
    }
}

#[cw_serde]
pub enum UncheckedReviewed {
    Addr(String),
    Id(u64),
}

impl UncheckedReviewed {
    fn check(self, deps: Deps) -> ContractResult<Reviewed> {
        Ok(match self {
            UncheckedReviewed::Addr(addr) => Reviewed::Addr(deps.api.addr_validate(&addr)?),
            UncheckedReviewed::Id(id) => Reviewed::Id(id),
        })
    }
}

#[cw_serde]
enum QueryMsg {
    CheckReview(QueryCheckReview),
}

#[cw_serde]
pub struct QueryCheckReview {
    pub reviewer: Addr,
    pub reviewed: Reviewed,
}

#[cw_serde]
pub struct CheckReviewResp {
    pub resp: bool,
}

#[cw_serde]
pub struct Review {
    pub id: u64,
    pub reviewer: Addr,
    pub reviewed: Reviewed,
    pub rating: u8,
    pub message: String,
    pub created_at: Timestamp,
}

#[cw_serde]
pub struct UncheckedReview {
    pub reviewed: UncheckedReviewed,
    pub rating: u8,
    pub message: String,
}

impl UncheckedReview {
    pub fn check(self, deps: &mut DepsMut, env: Env, info: MessageInfo) -> ContractResult<Review> {
        assert(
            is_valid_str(&self.message, 0, TEXT_MAX_BYTE_SIZE),
            ContractError::InvalidMessage,
        )?;
        let reviewed = self.reviewed.check(deps.as_ref())?;
        let contract_addr = VALIDATE_REVIEW_CONTRACT.load(deps.storage)?;
        let res: CheckReviewResp = deps.querier.query_wasm_smart(
            contract_addr,
            &QueryMsg::CheckReview(QueryCheckReview {
                reviewer: info.sender.clone(),
                reviewed: reviewed.clone(),
            }),
        )?;
        assert(res.resp, ContractError::Unauthorized)?;

        let id: ContractResult<_> = NEXT_ID.update(deps.storage, |v| Ok(v + 1));
        Ok(Review {
            id: id?,
            reviewer: info.sender,
            reviewed,
            rating: self.rating,
            message: self.message,
            created_at: env.block.time,
        })
    }
}

impl Review {
    pub fn instaniate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> ContractResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        NEXT_ID.save(deps.storage, &0)?;
        Ok(Response::new())
    }

    fn save(&self, deps: DepsMut) -> ContractResult<()> {
        REVIEWS.save(deps.storage, self.id, &self)?;
        Ok(())
    }

    fn get(deps: Deps, id: u64) -> ContractResult<Self> {
        Ok(REVIEWS.load(deps.storage, id)?)
    }

    pub fn exec_create(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: UncheckedReview,
    ) -> ContractResult<Response> {
        let review = msg.check(&mut deps, env, info)?;
        let key = review.reviewed.to_key()?;
        REVIEWED_TO_REVIEW.save(deps.storage, (key.as_slice(), review.id), &Empty {})?;
        REVIEWER_TO_REVIEW.save(deps.storage, (&review.reviewer, review.id), &Empty {})?;
        let _: ContractResult<_> = AVG_RATINGS.update(deps.storage, &key, |v| {
            let nr: Decimal = Decimal::new(Uint128::from(review.rating));
            if let Some((n, total)) = v {
                let nv = (n * total + nr) / (total + Decimal::one());
                Ok((nv, total + Decimal::one()))
            } else {
                Ok((nr, Decimal::one()))
            }
        });
        review.save(deps)?;
        Ok(Response::new())
    }

    pub fn exec_update(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ReviewUpdate,
    ) -> ContractResult<Response> {
        assert(msg.rating <= 5, ContractError::InvalidRating)?;
        let mut review = Self::get(deps.as_ref(), msg.id)?;
        assert(review.reviewer == info.sender, ContractError::Unauthorized)?;
        assert(
            is_valid_str(&msg.message, 0, TEXT_MAX_BYTE_SIZE),
            ContractError::InvalidMessage,
        )?;
        let key = review.reviewed.to_key()?;

        let _: ContractResult<_> = AVG_RATINGS.update(deps.storage, &key, |v| {
            let old: Decimal = Decimal::new(Uint128::from(review.rating));
            let new: Decimal = Decimal::new(Uint128::from(msg.rating));
            if let Some((n, total)) = v {
                let nv = (n * total - old + new) / total;
                Ok((nv, total))
            } else {
                Err(ContractError::InvalidRating)
            }
        });
        review.message = msg.message;
        review.rating = msg.rating;
        review.save(deps)?;
        Ok(Response::new())
    }

    pub fn query_review(deps: Deps, _env: Env, msg: u64) -> ContractResult<QueryResponse> {
        Ok(to_json_binary(&Self::get(deps, msg)?)?)
    }

    pub fn query_reviews(
        deps: Deps,
        _env: Env,
        msg: Pagination<u64>,
    ) -> ContractResult<QueryResponse> {
        let start = msg.start_after.map(|v| Bound::exclusive(v));
        let limit = msg.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let reviews = REVIEWS
            .range(deps.storage, start, None, Order::Descending)
            .take(limit)
            .map(|v| Ok(Self::get(deps, v?.0)?))
            .collect::<ContractResult<Vec<_>>>()?;
        Ok(to_json_binary(&ReviewListResp { reviews })?)
    }

    pub fn query_reviews_by_reviewer(
        deps: Deps,
        _env: Env,
        msg: ReviewsByReviewer,
    ) -> ContractResult<QueryResponse> {
        let addr = deps.api.addr_validate(&msg.reviewer)?;
        let start = msg.pagination.start_after.map(|v| Bound::exclusive(v));
        let limit = msg.pagination.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let reviews = REVIEWER_TO_REVIEW
            .prefix(&addr)
            .range(deps.storage, start, None, Order::Descending)
            .take(limit)
            .map(|v| Ok(Self::get(deps, v?.0)?))
            .collect::<ContractResult<Vec<_>>>()?;
        Ok(to_json_binary(&ReviewListResp { reviews })?)
    }

    pub fn query_reviews_by_reviewed(
        deps: Deps,
        _env: Env,
        msg: ReviewsByReviewered,
    ) -> ContractResult<QueryResponse> {
        let reviewed = msg.reviewed.check(deps)?.to_key()?;
        let start = msg.pagination.start_after.map(|v| Bound::exclusive(v));
        let limit = msg.pagination.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

        let reviews = REVIEWED_TO_REVIEW
            .prefix(&reviewed)
            .range(deps.storage, start, None, Order::Descending)
            .take(limit)
            .map(|v| Ok(Self::get(deps, v?.0)?))
            .collect::<ContractResult<Vec<_>>>()?;
        let avg = AVG_RATINGS.load(deps.storage, &reviewed)?;

        Ok(to_json_binary(&ReviewedListResp { reviews, avg })?)
    }
}
