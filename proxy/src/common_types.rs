use ic_kit::{candid::CandidType, Principal};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
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

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct AddCanisterMetadataInput {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

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

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Event {
    Addition {
        time: u64,
        by: Principal,              // The trusted source who has committed this action
        registry: Principal,        // The PID of the contacted registry
        metadata: CanisterMetadata, // The metadata of the canister
    },
    Deletion {
        time: u64,
        by: Principal,       // The trusted source who has committed this action
        registry: Principal, // The PID of the contacted registry
        canister: Principal, // The PID of the deleted canister
    },
    TrustedSourceAddition {
        time: u64,
        by: Principal,             // The admin who has added the trusted source
        trusted_source: Principal, // The PID of the trusted source
        accessible_registries: Vec<Principal>, // A vector of all registries that the trusted source can access
    },
    TrustedSourceDeletion {
        time: u64,
        by: Principal, // The admin who has deleted the trusted source from the proxy canister
        trusted_source: Principal, // The PID of the deleted trusted source
    },
    AccessChange {
        time: u64,
        by: Principal, // The admin who has changed the accessible registries list of the trusted source
        trusted_source: Principal, // The affected trusted source
        new_accessible_registries: Vec<Principal>, // The new vector of all registries that the trusted source can access
    },
}
