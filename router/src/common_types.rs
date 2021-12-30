use ic_kit::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Registry {
    pub principal_id: Principal,
    pub name: String,
    pub description: String,
    pub logo_url: String,
    pub front_end: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputAddRegistry {
    principal_id: Principal,
    name: String,
    description: String,
    logo_url: String,
    front_end: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputEditRegistry {
    principal_id: Principal,
    name: Option<String>,
    description: Option<String>,
    logo_url: Option<String>,
}

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentRegistry,
    BadParameters,
}