
use cosmwasm_std::StdError;
use cw_asset::AssetError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError{
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{0}")]
    AssetError(#[from] AssetError),
    #[error("{0}")]
    Factor(String),
    #[error("Unauthorized")]
    Unauthorized,
}