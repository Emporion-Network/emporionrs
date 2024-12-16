use crate::error::ContractError;

pub fn assert(v: bool, err: ContractError) -> Result<(), ContractError> {
    if v {
        Ok(())
    } else {
        Err(err)
    }
}
