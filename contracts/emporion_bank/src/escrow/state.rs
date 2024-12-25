use crate::{
    contract::{ContractResult, DEFAULT_LIMIT, MAX_LIMIT},
    error::ContractError,
    state::NEXT_ID,
    utils::{assert, Pagination},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Decimal, Deps, DepsMut, Empty, Env, Fraction, MessageInfo,
    QueryResponse, Response, Timestamp,
};
use cw20::Cw20ReceiveMsg;
use cw_asset::{AssetList, AssetListUnchecked, AssetUnchecked};
use cw_storage_plus::{Bound, Map};

use super::msgs::{EscrowFundCw20, EscrowListFor, EscrowListResp, InstantiateMsg};

pub const ESCROWS: Map<u64, Escrow> = Map::new("escrows");
pub const ADDR_TO_ESCROWS: Map<(&Addr, u64), Empty> = Map::new("arbiter_to_escrows");

#[cw_serde]
pub struct EscrowUnchecked {
    pub arbiter: String,
    pub payer: String,
    pub beneficiary: String,
    pub assets: AssetListUnchecked,
    pub split_coefficient: Decimal,
}

impl EscrowUnchecked {
    pub fn check(self, deps: &mut DepsMut, env: &Env) -> ContractResult<Escrow> {
        let arbiter = deps.api.addr_validate(&self.arbiter)?;
        let payer = deps.api.addr_validate(&self.payer)?;
        let beneficiary = deps.api.addr_validate(&self.beneficiary)?;
        let assets = self.assets.check(deps.api, None)?;
        if self.split_coefficient > Decimal::one() || self.split_coefficient < Decimal::zero() {
            return Err(ContractError::Custom(
                "Factor should be between 0 and 1".into(),
            ));
        }
        let split_coefficient = self.split_coefficient;
        let creation_time = env.block.time;
        let id = NEXT_ID.update(deps.storage, |v| Ok::<_, ContractError>(v + 1))?;
        Ok(Escrow {
            id,
            arbiter,
            payer,
            beneficiary,
            assets,
            creation_time,
            split_coefficient,
            status: EscrowStatus::Pending,
        })
    }
}

#[cw_serde]
pub enum EscrowStatus {
    Pending,
    Cancelled,
    Fulfilled,
    Disputed,
}

#[cw_serde]
pub struct Escrow {
    pub id: u64,
    pub arbiter: Addr,
    pub payer: Addr,
    pub beneficiary: Addr,
    pub assets: AssetList,
    pub split_coefficient: Decimal,
    pub status: EscrowStatus,
    pub creation_time: Timestamp,
}

impl Escrow {
    fn save(&self, deps: DepsMut) -> ContractResult<()> {
        ESCROWS.save(deps.storage, self.id, &self)?;
        ADDR_TO_ESCROWS.save(deps.storage, (&self.beneficiary, self.id), &Empty {})?;
        ADDR_TO_ESCROWS.save(deps.storage, (&self.arbiter, self.id), &Empty {})?;
        ADDR_TO_ESCROWS.save(deps.storage, (&self.payer, self.id), &Empty {})?;

        Ok(())
    }

    fn get(deps: &Deps, id: u64) -> ContractResult<Self> {
        Ok(ESCROWS.load(deps.storage, id)?)
    }

    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> ContractResult<Response> {
        NEXT_ID.save(deps.storage, &0)?;
        Ok(Response::default())
    }

    pub fn exec_create(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: EscrowUnchecked,
    ) -> ContractResult<Response> {
        let escrow = msg.check(&mut deps, &env)?;
        escrow.save(deps)?;
        let response = Response::new().add_attributes(vec![
            ("action", "create_escrow"),
            ("sender", &info.sender.to_string()),
        ]);
        Ok(response)
    }

    pub fn exec_fulfill(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: u64,
    ) -> ContractResult<Response> {
        // Send to beneficiary
        let mut escrow = Self::get(&deps.as_ref(), msg)?;
        assert(escrow.arbiter == info.sender, ContractError::Unauthorized)?;
        assert(
            escrow.status == EscrowStatus::Pending,
            ContractError::Unauthorized,
        )?;

        escrow.status = EscrowStatus::Fulfilled;
        let msgs = escrow.assets.transfer_msgs(&escrow.beneficiary)?;
        escrow.save(deps)?;
        Ok(Response::new().add_messages(msgs))
    }

    pub fn exec_cancel(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: u64,
    ) -> ContractResult<Response> {
        // Send to payer
        let mut escrow = Self::get(&deps.as_ref(), msg)?;
        assert(info.sender == escrow.arbiter, ContractError::Unauthorized)?;
        assert(
            escrow.status == EscrowStatus::Pending,
            ContractError::Unauthorized,
        )?;

        escrow.status = EscrowStatus::Cancelled;
        let msgs = escrow.assets.transfer_msgs(&escrow.payer)?;
        escrow.save(deps)?;
        Ok(Response::new().add_messages(msgs))
    }

    pub fn exec_split(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: u64,
    ) -> ContractResult<Response> {
        let mut escrow = Self::get(&deps.as_ref(), msg)?;
        assert(info.sender == escrow.arbiter, ContractError::Unauthorized)?;
        assert(
            escrow.status == EscrowStatus::Pending,
            ContractError::Unauthorized,
        )?;

        escrow.status = EscrowStatus::Disputed;
        let mut to_payer = escrow.assets.clone();
        let to_payer = to_payer.apply(|a| {
            a.amount = a.amount.multiply_ratio(
                escrow.split_coefficient.numerator(),
                escrow.split_coefficient.denominator(),
            );
        });
        let mut to_beneficiary = escrow.assets.clone();
        let to_beneficiary = to_beneficiary.deduct_many(to_payer)?;

        escrow.save(deps)?;

        Ok(Response::new()
            .add_messages(to_payer.transfer_msgs(escrow.payer)?)
            .add_messages(to_beneficiary.transfer_msgs(escrow.beneficiary)?))
    }

    pub fn exec_fund(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: u64,
    ) -> ContractResult<Response> {
        let funds = AssetList::from(&info.funds);
        let mut escrow = Self::get(&deps.as_ref(), msg)?;
        assert(
            escrow.status == EscrowStatus::Pending,
            ContractError::Unauthorized,
        )?;
        escrow.assets.add_many(&funds)?;
        escrow.save(deps)?;
        Ok(Response::new())
    }

    pub fn exec_fund_cw20(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: Cw20ReceiveMsg,
    ) -> ContractResult<Response> {
        let id = from_json::<EscrowFundCw20>(msg.msg)?.id;
        let asset = AssetUnchecked::cw20(info.sender, msg.amount).check(deps.api, None)?;
        let mut escrow = Self::get(&deps.as_ref(), id)?;
        escrow.assets.add(&asset)?;
        escrow.save(deps)?;

        Ok(Response::new())
    }

    pub fn query_escrow(deps: Deps, _env: Env, msg: u64) -> ContractResult<QueryResponse> {
        let escrow = Self::get(&deps, msg)?;
        Ok(to_json_binary(&escrow)?)
    }

    pub fn query_escrows(
        deps: Deps,
        _env: Env,
        msg: Pagination<u64>,
    ) -> ContractResult<QueryResponse> {
        let start = msg.start_after.map(|v| Bound::exclusive(v));
        let limit = msg.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let escrows = ESCROWS
            .range(deps.storage, start, None, cosmwasm_std::Order::Descending)
            .take(limit)
            .map(|v| Ok(v?.1))
            .collect::<ContractResult<Vec<_>>>()?;
        let resp = EscrowListResp { escrows };
        Ok(to_json_binary(&resp)?)
    }

    pub fn query_escrows_for(
        deps: Deps,
        _env: Env,
        msg: EscrowListFor,
    ) -> ContractResult<QueryResponse> {
        let addr = deps.api.addr_validate(&msg.addr)?;
        let start = msg.pagination.start_after.map(|v| Bound::exclusive(v));
        let limit = msg.pagination.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let escrows = ADDR_TO_ESCROWS
            .prefix(&addr)
            .range(deps.storage, start, None, cosmwasm_std::Order::Descending)
            .take(limit)
            .map(|v| Ok(Self::get(&deps, v?.0)?))
            .collect::<ContractResult<Vec<_>>>()?;
        let resp = EscrowListResp { escrows };
        Ok(to_json_binary(&resp)?)
    }
}
