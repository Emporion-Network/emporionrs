use crate::error::ContractError;

pub fn assert(v: bool, err: ContractError) -> Result<(), ContractError> {
    if v {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn is_valid_str(str: &str, min: usize, max: usize) -> bool {
    let bytes = str.as_bytes();
    if bytes.len() < min || bytes.len() > max {
        return false;
    }
    true
}
