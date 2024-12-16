use cosmwasm_std::{
    to_json_binary, Addr, Decimal, Deps, DepsMut, Env, MessageInfo, Order, QueryResponse, Response,
    Timestamp,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};

use crate::{
    contract::{ContractResult, CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractError,
    msgs::{Filter, InstantiateMsg, MsgUpdate, PricesResp},
};

pub const PRICES: Map<&str, Decimal> = Map::new("prices");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const TIMESTAMP: Item<Timestamp> = Item::new("last_updated_at");
pub const PROVIDER_NAME: Item<String> = Item::new("provider_name");

pub struct Price {}

impl Price {
    pub fn instaniate(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        ADMIN.save(deps.storage, &info.sender)?;
        PROVIDER_NAME.save(deps.storage, &msg.provider_name)?;
        Ok(Response::new())
    }

    pub fn update(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: MsgUpdate,
    ) -> ContractResult<Response> {
        if info.sender != ADMIN.load(deps.storage)? {
            return Err(ContractError::Unauthorized);
        }
        msg.prices
            .iter()
            .map(|v| Ok(PRICES.save(deps.storage, &v.0, &v.1)?))
            .collect::<ContractResult<Vec<_>>>()?;
        TIMESTAMP.save(deps.storage, &env.block.time)?;
        Ok(Response::new())
    }

    pub fn query_prices(deps: Deps, _env: Env, filter: Filter) -> ContractResult<QueryResponse> {
        let last_updated_at = TIMESTAMP.load(deps.storage)?;
        let prices = if let Some(keys) = filter {
            keys.into_iter()
                .filter_map(|k| PRICES.load(deps.storage, &k).map(|v| (k, v)).ok())
                .collect::<Vec<_>>()
        } else {
            PRICES
                .range(deps.storage, None, None, Order::Descending)
                .collect::<Result<Vec<_>, _>>()?
        };

        Ok(to_json_binary(&PricesResp {
            prices,
            last_updated_at,
        })?)
    }
}
