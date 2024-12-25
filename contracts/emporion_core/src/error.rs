use cosmwasm_std::StdError;
use cw_asset::AssetError;
use neutron_sdk::NeutronError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{0}")]
    AssetError(#[from] AssetError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Custom(String),
    #[error("ICA not found")]
    Ica,
    #[error("{0}")]
    NeutronError(#[from] NeutronError),
}
