use crate::error::ContractError;
pub type ContractResult<T> = Result<T, ContractError>;


pub fn assert(v:bool, err:ContractError)->Result<(), ContractError>{
    if v {
        Ok(())
    } else {
        Err(err)
    }
}