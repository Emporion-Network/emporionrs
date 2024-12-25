use std::ops::RangeFrom;

use crate::{contract::ContractResult, module::Module};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Decimal, Deps, DepsMut, Env, MessageInfo, QueryResponse, Reply, Response,
    SubMsg, Uint128,
};
use cw_asset::{Asset, AssetList};
use cw_storage_plus::Map;
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_std::types::{
    cosmos::{
        bank::v1beta1::{DenomUnit, Metadata},
        base::v1beta1::Coin,
    },
    osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint, MsgSetDenomMetadata},
};

use super::msgs::{Exec, Instantiate, Query};

pub const COIN_DENOM_BASE: &str = "uemp";
pub const COIN_DENOM: &str = "emp";
pub const COIN_NAME: &str = "Emporion";
pub const COIN_DISPLAY: &str = "Emporion";
pub const COIN_TICKER: &str = "EMPR";
pub const COIN_DESCRIPTION: &str = "Emporion Staking Coin";
pub const TOTAL_SUPPLY: Uint128 = Uint128::new(10_000_000_000000);

pub const CREATED_DENOM: u64 = 1;
pub const MINTED_DENOM: u64 = 2;

const REPLY_RANGE_START: u64 = CREATED_DENOM;
const REPLY_RANGE_END: u64 = MINTED_DENOM;

pub const HOST_CHAIN_DENOM: &str = "untrn";

/// STATE
pub const INTERCHAIN_ACCOUNT: Map<&str, Option<InterchainAccount>> =
    Map::new("interchain_accounts");

const TREASURY: &str = "TREASURY";

const ACCOUNTS: Map<&str, Account> = Map::new("accounts");

#[cw_serde]
pub struct Account {
    pub name: String,
    pub fee_share: Decimal,
    pub amount: AssetList,
}

#[cw_serde]
pub struct InterchainAccount {
    pub address: String,
}

pub struct Bank {}

impl Module for Bank {
    type InstantiateMsg = Instantiate;
    type ExecuteMsg = Exec;
    type QueryMsg = Query;
    type SudoMsg = SudoMsg;

    const REPLY_RANGE_START: u64 = REPLY_RANGE_START;

    const REPLY_RANGE_END: u64 = REPLY_RANGE_END;

    fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Self::InstantiateMsg,
    ) -> ContractResult<Response> {
        Self::instantiate(deps, env, info, msg)
    }

    fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Self::ExecuteMsg,
    ) -> ContractResult<Response> {
        Self::execute(deps, env, info, msg)
    }

    fn query(deps: Deps, env: Env, msg: Self::QueryMsg) -> ContractResult<QueryResponse> {
        Self::query(deps, env, msg)
    }

    fn sudo(deps: DepsMut, env: Env, msg: Self::SudoMsg) -> ContractResult<QueryResponse> {
        Self::sudo(deps, env, msg)
    }

    fn reply(deps: DepsMut, env: Env, msg: Reply) -> ContractResult<Response> {
        Self::reply(deps, env, msg)
    }
}

impl Bank {
    fn set_metadata(env: &Env, denom: &str) -> ContractResult<Response> {
        let unit_base = DenomUnit {
            denom: denom.to_string(),
            exponent: 0,
            aliases: vec![COIN_DENOM_BASE.to_string()],
        };
        let unit = DenomUnit {
            denom: COIN_TICKER.to_string(),
            exponent: 6,
            aliases: vec![COIN_TICKER.to_string()],
        };
        let meta = Metadata {
            description: COIN_DESCRIPTION.to_string(),
            denom_units: vec![unit_base, unit],
            base: denom.to_string(),
            display: COIN_TICKER.to_string(),
            name: COIN_TICKER.to_string(),
            symbol: COIN_TICKER.to_string(),
            uri: "".to_string(),
            uri_hash: "".to_string(),
        };
        let set_meta_msg = MsgSetDenomMetadata {
            sender: env.contract.address.to_string(),
            metadata: Some(meta),
        };
        let mitn_msg = SubMsg::reply_on_success(
            MsgMint {
                sender: env.contract.address.to_string(),
                amount: Some(Coin {
                    denom: denom.to_string(),
                    amount: TOTAL_SUPPLY.into(),
                }),
                mint_to_address: env.contract.address.to_string(),
            },
            MINTED_DENOM,
        );
        Ok(Response::new()
            .add_message(set_meta_msg)
            .add_submessage(mitn_msg))
    }

    fn add_total_supply(deps: DepsMut, denom: &str) -> ContractResult<Response> {
        let mut amount = AssetList::new();
        amount.add(&Asset::native(denom, TOTAL_SUPPLY))?;
        ACCOUNTS.save(
            deps.storage,
            TREASURY,
            &Account {
                name: TREASURY.to_string(),
                fee_share: Decimal::zero(),
                amount,
            },
        )?;
        Ok(Response::default())
    }

    /**
    Splits amout amongst accounts
    */
    fn split_amount() {}

    /**
    Sends USDC from TREASURY to Noble
    */
    fn withdraw_to_noble() {}

    /**
    Sends USDC from Noble to Osmo and swaps usdc to denom
    */
    fn withdraw_and_swap() {}

    /**
    Deposits Denom from Osmo
    */
    fn deposit() {}

    /**
    Deposits Denom from Osmo
    */
    fn withdraw() {}

    fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Instantiate,
    ) -> ContractResult<Response> {
        let create_denom = SubMsg::reply_on_success(
            MsgCreateDenom {
                sender: env.contract.address.to_string(),
                subdenom: COIN_DENOM_BASE.to_string(),
            },
            CREATED_DENOM,
        );
        Ok(Response::new()
            .add_attribute("action", "instantiate_bank")
            .add_submessage(create_denom))
    }

    fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Exec) -> ContractResult<Response> {
        todo!()
    }

    fn query(deps: Deps, env: Env, msg: Query) -> ContractResult<QueryResponse> {
        match msg {
            Query::Accounts {} => Self::query_accounts(deps),
        }
    }

    fn query_accounts(deps: Deps) -> ContractResult<QueryResponse> {
        let accounts = ACCOUNTS
            .range(deps.storage, None, None, cosmwasm_std::Order::Descending)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(to_json_binary(&accounts)?)
    }

    fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> ContractResult<QueryResponse> {
        todo!()
    }

    fn reply(deps: DepsMut, env: Env, msg: Reply) -> ContractResult<Response> {
        let denom = format!("factory/{}/{}", env.contract.address, COIN_DENOM_BASE);
        match msg.id {
            CREATED_DENOM => Self::set_metadata(&env, &denom),
            MINTED_DENOM => Self::add_total_supply(deps, &denom),
            ..REPLY_RANGE_START => Ok(Response::default()),
            REPLY_RANGE_END.. => Ok(Response::default()),
        }
    }
}
