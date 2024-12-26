use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use ulid::Ulid;

use crate::structs::error::Error;

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct NonceReq {
    pub address: String,
}
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct NonceRes {
    nonce:String
}

pub struct NonceToAddr {
    timeout: Duration,
    map: HashMap<String, (String, Instant)>,
}

impl NonceToAddr {
    pub fn new(timeout: Duration) -> Self {
        NonceToAddr {
            map: HashMap::new(),
            timeout,
        }
    }

    fn clear(&mut self) -> Instant {
        let now = Instant::now();
        self.map
            .retain(|_, (_, t)| now.duration_since(*t) < self.timeout);
        now
    }

    pub fn check(&mut self, nonce: &str, addr: &str) -> Result<(), Error> {
        self.clear();
        self.map
            .get(nonce)
            .ok_or(Error::new("Not found"))
            .and_then(|(expected, _)| {
                    if expected != addr {
                        Err(Error::new("Wrong address"))
                    } else {
                        Ok(())
                    }
                })
    }

    pub fn add(&mut self, addr: &str) -> Result<NonceRes, Error> {
        let now = self.clear();
        if let Some(_) = self.map.values().find(|(other, _)| other == addr) {
            return Err(Error::too_many_requests("Nonce has been sent"));
        };
        let nonce = Ulid::new().to_string();
        self.map.insert(nonce.clone(), (addr.to_string(), now));
        Ok(NonceRes {
            nonce
        })
    }
}
