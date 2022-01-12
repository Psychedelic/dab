use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Registry {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub details: Vec<(String, String)>,
    pub version: u32
}

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentRegistry,
    BadParameters,
}