use ic_kit::candid::CandidType;
use serde::Deserialize;

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
