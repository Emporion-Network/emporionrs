use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse,
    Response, Timestamp,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};

use crate::{
    contract::{ContractResult, CONTRACT_NAME, CONTRACT_VERSION, TEXT_MAX_BYTE_SIZE},
    error::ContractError,
    msgs::{InstantiateMsg, ReviewUpdate},
    utils::{assert, is_valid_str},
};

const REVIEWS: Map<u64, Review> = Map::new("reviews");
const REVIEWED_TO_REVIEW: Map<(Reviewed, u64), Empty> = Map::new("reviewed_to_review");
const REVIEWER_TO_REVIEW: Map<(Addr, u64), Empty> = Map::new("reviewer_to_review");
const VALIDATE_REVIEW_CONTRACT: Item<Addr> = Item::new("validate_review_contract");

const NEXT_ID: Item<u64> = Item::new("next_id");

#[cw_serde]
pub enum Reviewed {
    Addr(Addr),
    Id(u64),
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

    fn load(deps: Deps, id: u64) -> ContractResult<Self> {
        Ok(REVIEWS.load(deps.storage, id)?)
    }

    pub fn exec_create(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: UncheckedReview,
    ) -> ContractResult<Response> {
        let review = msg.check(&mut deps, env, info)?;
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
        let mut review = Self::load(deps.as_ref(), msg.id)?;
        assert(review.reviewer == info.sender, ContractError::Unauthorized)?;
        assert(
            is_valid_str(&msg.message, 0, TEXT_MAX_BYTE_SIZE),
            ContractError::InvalidMessage,
        )?;
        review.message = msg.message;
        review.rating = msg.rating;
        review.save(deps)?;
        Ok(Response::new())
    }
}
