use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType)]
pub enum Error {
    CanisterAlreadyExists,
    BadParameters,
    NonExistantCanister,
    NotAuthorized
}

// The metadata structure that you want to store in your registry.
#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: String,
    pub details: Vec<(String, String)>,
    pub version: u32
}