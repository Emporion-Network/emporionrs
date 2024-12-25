use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Exec {}

#[cw_serde]
pub enum Query {
    Accounts {},
}

#[cw_serde]
pub struct Instantiate {}
