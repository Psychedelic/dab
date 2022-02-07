use ic_kit::candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum DetailValue {
    True,
    False,
    U64(u64),
    I64(i64),
    Float(f64),
    Text(String),
    Principal(Principal),
    #[serde(with = "serde_bytes")]
    Slice(Vec<u8>),
    Vec(Vec<DetailValue>),
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Registry {
    pub principal_id: Principal,
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    NonExistentItem,
    BadParameters,
    Unknown(String),
}
