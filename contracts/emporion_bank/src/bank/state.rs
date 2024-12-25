use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Decimal, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    SubMsg, Uint128,
};

use cw_asset::{Asset, AssetList};
use cw_storage_plus::Map;
use neutron_std::types::{
    cosmos::bank::v1beta1::{DenomUnit, Metadata},
    cosmos::base::v1beta1::Coin,
    osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgCreateDenomResponse, MsgMint, MsgSetDenomMetadata,
    },
};
const CREATED_DENOM: u64 = 1;
const MINTED_DENOM: u64 = 2;
pub const COIN_DENOM_BASE: &str = "uemp";
pub const COIN_DENOM: &str = "emp";
pub const COIN_NAME: &str = "Emporion";
pub const COIN_DISPLAY: &str = "Emporion";
pub const COIN_TICKER: &str = "EMPR";
pub const COIN_DESCRIPTION: &str = "Emporion Staking Coin";
pub const TOTAL_SUPPLY: Uint128 = Uint128::new(10_000_000_000000);
use crate::{contract::ContractResult, error::ContractError, msgs::InstantiateMsg};

const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
const SUPPLY_ACCOUNT: &str = "SUPPLY";

#[cw_serde]
pub struct Account {
    pub name: String,
    pub share: Decimal,
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
            share: self.share,
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
            denom: COIN_DENOM.to_string(),
            exponent: 6,
            aliases: vec![],
        };
        let meta = Metadata {
            description: COIN_DESCRIPTION.to_string(),
            denom_units: vec![unit_base, unit],
            base: denom.to_string(),
            display: COIN_DISPLAY.to_string(),
            name: COIN_NAME.to_string(),
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

    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResult<Response> {
        let msg = SubMsg::reply_on_success(
            MsgCreateDenom {
                sender: env.contract.address.to_string(),
                subdenom: COIN_DENOM_BASE.to_string(),
            },
            CREATED_DENOM,
        );

        ACCOUNTS.save(
            deps.storage,
            SUPPLY_ACCOUNT,
            &Account {
                name: SUPPLY_ACCOUNT.to_string(),
                share: Decimal::zero(),
                admin: env.contract.address,
                amount: AssetList::new(),
            },
        )?;

        Ok(Response::new().add_submessage(msg))
    }
    pub fn handle_reply(deps: DepsMut, env: Env, msg: Reply) -> ContractResult<Response> {
        match msg.id {
            CREATED_DENOM => {
                let msg = msg
                    .result
                    .into_result()
                    .map_err(|v| ContractError::Custom(v))?;
                let msg = msg.msg_responses.into_iter().nth(0).unwrap();
                let denom = MsgCreateDenomResponse::try_from(msg.value)?.new_token_denom;
                Self::update_denom(&env, &denom).map(|response| {
                    response.add_message(MsgMint {
                        sender: env.contract.address.to_string(),
                        amount: Some(Coin {
                            denom,
                            amount: TOTAL_SUPPLY.into(),
                        }),
                        mint_to_address: env.contract.address.to_string(),
                    })
                })
            }
            MINTED_DENOM => {
                let _: ContractResult<_> =
                    ACCOUNTS.update(deps.storage, SUPPLY_ACCOUNT, |account| {
                        if let Some(mut account) = account {
                            account.amount.add(&Asset::native(
                                format!("factory/{}/{}", env.contract.address, COIN_DENOM_BASE),
                                TOTAL_SUPPLY,
                            ))?;
                            Ok(account)
                        } else {
                            Err(ContractError::Custom("Account not found".into()))
                        }
                    });
                Ok(Response::default())
            }
            _ => Ok(Response::default()),
        }
    }
}
