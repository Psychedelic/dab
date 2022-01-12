use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Registry {
    pub name: String,
    pub description: String,
    pub logo_url: String,
    pub front_end: Option<String>,
}

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentRegistry,
    BadParameters,
}