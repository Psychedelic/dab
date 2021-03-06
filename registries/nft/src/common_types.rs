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

pub const DESCRIPTION_LIMIT: usize = 1200;
pub const NAME_LIMIT: usize = 120;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct AddNftInput {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct NftCanister {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub submitter: Principal,
    pub last_updated_by: Principal,
    pub last_updated_at: u64,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Debug, PartialEq, Deserialize, Clone)]
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
