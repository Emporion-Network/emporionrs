use cosmwasm_std::StdError;
use cw_asset::AssetError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    AssetError(#[from] AssetError),
    #[error("{0}")]
    Custom(String),
}

impl ContractError {
    fn custom(e: impl Into<String>) -> Self {
        Self::Custom(e.into())
    }
}
