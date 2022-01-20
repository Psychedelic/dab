use ic_kit::candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Registry {
    pub principal_id: Principal,
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub details: Vec<(String, String)>,
}

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    NonExistentItem,
    BadParameters,
    Unknown(String),
}
