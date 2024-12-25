use super::reply_cases;
use super::utils::{query_register_fee, OpenAckVersion};
use crate::utils::Append;
use crate::{contract::ContractResult, error::ContractError};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, SubMsg, Uint128,
};
use cw_asset::{Asset, AssetList};
use cw_storage_plus::{Item, Map};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::bindings::types::ProtobufAny;
use neutron_sdk::interchain_txs::helpers::decode_message_response;
use neutron_sdk::query::min_ibc_fee::query_min_ibc_fee;
use neutron_sdk::sudo::msg::{RequestPacket, RequestPacketTimeoutHeight, SudoMsg};
use neutron_std::types::ibc::applications::transfer::v1::MsgTransfer;
use neutron_std::types::ibc::core::channel::v1::Order;
use neutron_std::types::neutron::interchaintxs::v1::{
    MsgRegisterInterchainAccount, MsgSubmitTxResponse,
};
use neutron_std::types::{
    cosmos::bank::v1beta1::{DenomUnit, Metadata},
    cosmos::base::v1beta1::Coin,
    osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint, MsgSetDenomMetadata},
};

pub const COIN_DENOM_BASE: &str = "uemp";
pub const COIN_DENOM: &str = "emp";
pub const COIN_NAME: &str = "Emporion";
pub const COIN_DISPLAY: &str = "Emporion";
pub const COIN_TICKER: &str = "EMPR";
pub const COIN_DESCRIPTION: &str = "Emporion Staking Coin";
pub const TOTAL_SUPPLY: Uint128 = Uint128::new(10_000_000_000000);

pub const HOST_CHAIN_DENOM: &str = "untrn";

const TREASURY_SHARES: &str = "TREASURY_SHARES";
const BUY_BACK: &str = "BUY_BACK";
const RND: &str = "RND";
const DIVIDENTS: &str = "DIVIDENTS";

const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
pub const INTERCHAIN_ACCOUNT: Map<&str, Option<InterchainAccount>> =
    Map::new("interchain_accounts");

pub const ERRORS: Item<Vec<(String, RequestPacket)>> = Item::new("errors");

#[cw_serde]
pub struct InterchainAccount {
    pub address: String,
    pub port_id: String,
    pub channel_id: String,
    pub counterparty_channel_id: String,
}

#[cw_serde]
pub struct Account {
    pub name: String,
    pub fee_share: Decimal,
    pub admin: Addr,
    pub amount: AssetList,
}

#[cw_serde]
pub struct UncheckedAccount {
    pub name: String,
    pub share: Decimal,
    pub admin: String,
}

impl UncheckedAccount {
    pub fn check(self, deps: &Deps) -> ContractResult<Account> {
        Ok(Account {
            name: self.name,
            fee_share: self.share,
            admin: deps.api.addr_validate(&self.admin)?,
            amount: AssetList::new(),
        })
    }
}

pub struct Bank {
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn load(deps: &Deps) -> ContractResult<Self> {
        Ok(Bank {
            accounts: ACCOUNTS
                .range(deps.storage, None, None, cosmwasm_std::Order::Descending)
                .map(|e| Ok(e?.1))
                .collect::<ContractResult<Vec<_>>>()?,
        })
    }
    pub fn save(&self, deps: &mut DepsMut) -> ContractResult<()> {
        self.accounts
            .iter()
            .map(|a| {
                ACCOUNTS.save(deps.storage, &a.name, a)?;
                Ok(())
            })
            .collect::<ContractResult<Vec<_>>>()?;
        Ok(())
    }

    fn update_denom(env: &Env, denom: &str) -> ContractResult<Response> {
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
        let msg = MsgSetDenomMetadata {
            sender: env.contract.address.to_string(),
            metadata: Some(meta),
        };
        Ok(Response::new().add_message(msg))
    }

    pub fn query(deps: Deps<NeutronQuery>) -> ContractResult<QueryResponse> {
        let r = INTERCHAIN_ACCOUNT
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(to_json_binary(&r)?)
    }

    pub fn instantiate(
        deps: &mut DepsMut,
        env: &Env,
        info: &MessageInfo,
    ) -> ContractResult<Response> {
        let msg = SubMsg::reply_on_success(
            MsgCreateDenom {
                sender: env.contract.address.to_string(),
                subdenom: COIN_DENOM_BASE.to_string(),
            },
            reply_cases::CREATED_DENOM,
        );

        ERRORS.save(deps.storage, &vec![])?;

        ACCOUNTS.save(
            deps.storage,
            TREASURY_SHARES,
            &Account {
                name: TREASURY_SHARES.to_string(),
                fee_share: Decimal::zero(),
                admin: env.contract.address.clone(),
                amount: AssetList::new(),
            },
        )?;

        ACCOUNTS.save(
            deps.storage,
            BUY_BACK,
            &Account {
                name: BUY_BACK.to_string(),
                fee_share: Decimal::checked_from_ratio(1u64, 3u64).unwrap(),
                admin: env.contract.address.clone(),
                amount: AssetList::new(),
            },
        )?;

        ACCOUNTS.save(
            deps.storage,
            DIVIDENTS,
            &Account {
                name: DIVIDENTS.to_string(),
                fee_share: Decimal::checked_from_ratio(1u64, 3u64).unwrap(),
                admin: env.contract.address.clone(),
                amount: AssetList::new(),
            },
        )?;

        ACCOUNTS.save(
            deps.storage,
            RND,
            &Account {
                name: RND.to_string(),
                fee_share: Decimal::checked_from_ratio(1u64, 3u64).unwrap(),
                admin: info.sender.clone(),
                amount: AssetList::new(),
            },
        )?;
        let mut resp = Bank::open_isc(deps, env, "gia", "connection-0")?;

        Ok(Response::new().add_submessage(msg).append(&mut resp))
    }

    pub fn open_isc(
        deps: &mut DepsMut,
        env: &Env,
        name: &str,
        connection_id: &str,
    ) -> ContractResult<Response> {
        if INTERCHAIN_ACCOUNT.may_load(deps.storage, name)?.is_some() {
            return Err(ContractError::Custom(
                "ICA channel already exists and open".to_string(),
            ));
        }
        let fee = query_register_fee(deps.as_ref())?
            .params
            .map(|v| v.register_fee)
            .unwrap_or_default();

        let msg: CosmosMsg = MsgRegisterInterchainAccount {
            from_address: env.contract.address.to_string(),
            connection_id: connection_id.to_string(),
            interchain_account_id: name.to_string(),
            ordering: Order::Unordered.into(),
            register_fee: fee,
        }
        .into();
        INTERCHAIN_ACCOUNT.save(deps.storage, name, &None)?;

        Ok(Response::new().add_message(msg))
    }

    pub fn withdraw(
        deps: &mut DepsMut<NeutronQuery>,
        env: Env,
        chain: &str,
    ) -> ContractResult<Response<NeutronMsg>> {
        let ica = INTERCHAIN_ACCOUNT
            .load(deps.storage, chain)?
            .ok_or(ContractError::Ica)?;
        let fee = query_min_ibc_fee(deps.as_ref())?.min_fee;
        let msg = NeutronMsg::IbcTransfer {
            source_port: "transfer".into(),
            source_channel: "channel-0".to_string(),
            token: cosmwasm_std::Coin {
                denom: "untrn".to_string(),
                amount: Uint128::new(1000u128),
            },
            sender: env.contract.address.to_string(),
            receiver: ica.address.to_string(),
            timeout_height: RequestPacketTimeoutHeight {
                revision_height: None,
                revision_number: None,
            },
            timeout_timestamp: env.block.time.plus_seconds(200).nanos(),
            memo: "".to_string(),
            fee,
        };

        Ok(Response::new().add_message(msg))
    }

    pub fn deposit(
        deps: &mut DepsMut<NeutronQuery>,
        env: Env,
        chain: &str,
    ) -> ContractResult<Response<NeutronMsg>> {
        let ica = INTERCHAIN_ACCOUNT
            .load(deps.storage, chain)?
            .ok_or(ContractError::Ica)?;
        let fee = query_min_ibc_fee(deps.as_ref())?.min_fee;
        let msg = MsgTransfer {
            source_port: "transfer".into(),
            source_channel: "channel-0".to_string(),
            token: Some(Coin {
                denom: "ibc/4E41ED8F3DCAEA15F4D6ADC6EDD7C04A676160735C9710B904B7BF53525B56D6"
                    .to_string(),
                amount: "500".to_string(),
            }),
            receiver: env.contract.address.to_string(),
            sender: ica.address.to_string(),
            timeout_height: None,
            timeout_timestamp: env.block.time.plus_minutes(1).nanos(),
            memo: "".to_string(),
        }
        .to_any();
        let type_url = msg.type_url.clone();
        let msg = NeutronMsg::SubmitTx {
            msgs: vec![ProtobufAny {
                type_url: msg.type_url.clone(),
                value: msg.value.into(),
            }],
            connection_id: "connection-0".to_string(),
            interchain_account_id: chain.to_string(),
            memo: "".to_string(),
            timeout: 50,
            fee,
        };

        Ok(Response::new()
            .add_message(msg)
            .add_attribute("type_url", type_url))
    }

    pub fn query_errors(deps: Deps<NeutronQuery>) -> ContractResult<QueryResponse> {
        let r = ERRORS.load(deps.storage)?;
        Ok(to_json_binary(&r)?)
    }

    pub fn sudo(deps: &mut DepsMut, _env: Env, msg: SudoMsg) -> ContractResult<()> {
        match msg {
            SudoMsg::OpenAck {
                port_id,
                channel_id,
                counterparty_channel_id,
                counterparty_version,
            } => {
                let name = port_id
                    .split(".")
                    .nth(1)
                    .ok_or(ContractError::Custom("Could not find the key".to_string()))?;
                let parsed_version: OpenAckVersion = from_json(counterparty_version)?;
                INTERCHAIN_ACCOUNT.save(
                    deps.storage,
                    name,
                    &Some(InterchainAccount {
                        address: parsed_version.address,
                        port_id: port_id.to_string(),
                        channel_id,
                        counterparty_channel_id,
                    }),
                )?;
            }
            SudoMsg::Timeout { request } => {
                let name = request
                    .source_port
                    .ok_or(ContractError::Custom("Could not find the key".to_string()))?;
                let name = name
                    .split(".")
                    .nth(1)
                    .ok_or(ContractError::Custom("Could not find the key".to_string()))?;

                INTERCHAIN_ACCOUNT.save(deps.storage, name, &None)?
            }
            SudoMsg::Error { request, details } => {
                let name = request
                    .source_port
                    .clone()
                    .ok_or(ContractError::Custom("Could not find the key".to_string()))?;
                let name = name
                    .split(".")
                    .nth(1)
                    .ok_or(ContractError::Custom("Could not find the key".to_string()))?;

                INTERCHAIN_ACCOUNT.save(deps.storage, name, &None)?;
                let x: ContractResult<_> = ERRORS.update(deps.storage, |mut v| {
                    v.push((details, request));
                    Ok(v)
                });
                x?;
            }
            SudoMsg::Response { request, data } => {}
            _ => (),
        };
        Ok(())
    }

    pub fn reply(deps: &mut DepsMut, env: Env, msg: Reply) -> ContractResult<Response> {
        let denom = format!("factory/{}/{}", env.contract.address, COIN_DENOM_BASE);
        match msg.id {
            reply_cases::CREATED_DENOM => Self::update_denom(&env, &denom).map(|response| {
                response.add_message(MsgMint {
                    sender: env.contract.address.to_string(),
                    amount: Some(Coin {
                        denom,
                        amount: TOTAL_SUPPLY.into(),
                    }),
                    mint_to_address: env.contract.address.to_string(),
                })
            }),
            reply_cases::MINTED_DENOM => {
                let _: ContractResult<_> =
                    ACCOUNTS.update(deps.storage, TREASURY_SHARES, |account| {
                        if let Some(mut account) = account {
                            account.amount.add(&Asset::native(denom, TOTAL_SUPPLY))?;
                            Ok(account)
                        } else {
                            Err(ContractError::Custom("Account not found".into()))
                        }
                    });
                Ok(Response::default())
            }
            reply_cases::SUBMIT_TX_RESP => {
                let r = &msg
                    .result
                    .into_result()
                    .map_err(|v| ContractError::Custom(v))?
                    .msg_responses[0]
                    .value
                    .to_vec();
                let r: MsgSubmitTxResponse = decode_message_response(r)?;
                Ok(Response::default())
            }
            _ => Ok(Response::default()),
        }
    }
}
