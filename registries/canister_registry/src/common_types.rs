use ic_kit::{candid::CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, PartialEq)]
pub enum OperationError {
    NotAuthorized,
    BadParameters(String),
    NonExistentItem,
    Unknown(String),
}

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
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
    pub canisters: Vec<&'static CanisterMetadata>,
}

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

pub const DESCRIPTION_LIMIT: usize = 1200;
pub const NAME_LIMIT: usize = 24;
pub const DEFAULT_LIMIT: usize = 20;
