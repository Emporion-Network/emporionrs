use cosmwasm_std::StdError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum  ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized,
}