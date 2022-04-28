use ic_kit::{candid::CandidType, Principal};
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
pub struct Token {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Clone, Debug, PartialEq)]
pub struct GetAllPaginatedResponse {
    pub amount: usize,
    pub tokens: Vec<&'static Token>,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum OperationError {
    NotAuthorized,
    NonExistentItem,
    BadParameters,
    Unknown(String),
}

#[derive(Deserialize, CandidType)]
pub enum RegistryResponse {
    Ok(Option<String>),
    Err(OperationError),
}

pub const CANISTER_REGISTRY_ID: &'static str = "curr3-vaaaa-aaaah-abbdq-cai";
pub const DEFAULT_LIMIT: usize = 20;
