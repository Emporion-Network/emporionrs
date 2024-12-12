use serde::Deserialize;

use crate::structs::msgs::msg_send::Coin;
use super::Query;

pub struct Balances{
    pub address:String,
}


#[derive(Deserialize, Debug)]
pub struct BalancesResp {
    pub balances:Vec<Coin>
}


impl Query for Balances {
    type Output = BalancesResp;

    fn path(&self, rest:&str)->String {
        format!("{}/cosmos/bank/v1beta1/balances/{}",rest, self.address)
    }
}

pub struct Balance {
    pub address:String,
    pub denom:String,
}

#[derive(Deserialize, Debug)]
pub struct BalanceResp {
    pub balance:Coin
}


impl Query for Balance {
    type Output = BalanceResp;
    fn path(&self, rest:&str)->String {
        format!("{}/cosmos/bank/v1beta1/balances/{}/by_denom?denom={}",rest, self.address, self.denom)
    }
}