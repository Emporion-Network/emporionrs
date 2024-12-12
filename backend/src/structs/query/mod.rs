use serde::de::DeserializeOwned;

pub mod bank;
pub mod wasm;
pub mod auth;

pub trait Query {
    type Output: DeserializeOwned;
    fn path(&self, rest:&str)->String;
}