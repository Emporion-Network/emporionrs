use super::Query;

pub struct AccountInfo{
    pub address:String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SequenceResp {
    pub account:Account,
}
#[derive(serde::Deserialize, Debug)]
pub struct Account {
    pub sequence:String,
    pub account_number:String
}

impl Query for AccountInfo {
    type Output = SequenceResp;

    fn path(&self, rest:&str)->String {
        format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            rest, self.address
        )
    }
}