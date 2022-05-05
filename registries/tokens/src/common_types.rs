use ic_kit::{candid::CandidType, ic, Principal};
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
pub struct AddTokenInput {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Token {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub submitter: Principal,
    pub last_updated_by: Principal,
    pub last_modification: u64,
    pub details: Vec<(String, DetailValue)>,
}

impl From<AddTokenInput> for Token {
    fn from(wrapper: AddTokenInput) -> Token {
        return Token {
            name: wrapper.name,
            description: wrapper.description,
            thumbnail: wrapper.thumbnail,
            frontend: wrapper.frontend,
            principal_id: wrapper.principal_id,
            details: wrapper.details,
            submitter: ic::caller(),
            last_updated_by: ic::caller(),
            last_modification: ic::time(),
        };
    }
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
