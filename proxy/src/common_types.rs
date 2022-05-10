use ic_kit::{candid::CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct AddTrustedSourceInput {
    pub principal_id: Principal,
    pub accessible_registries: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct TrustedSource {
    pub added_by: Principal,
    pub principal_id: Principal,
    pub last_call: u64,
    pub accessible_registries: Vec<Principal>,
}

#[derive(CandidType, Debug, Deserialize)]
pub enum OperationError {
    NotAuthorized,
    NonExistentItem,
    BadParameters(String),
    Unknown(String),
}

#[derive(Deserialize, CandidType)]
pub enum RegistryResponse {
    Ok(Option<String>),
    Err(OperationError),
}
