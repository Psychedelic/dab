use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType)]
pub enum Error {
    NotAuthorized,
    BadParameters,
    NonExistentItem,
    Unknown(String),
}

// The metadata structure that you want to store in your registry.
#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub details: Vec<(String, String)>,
}
