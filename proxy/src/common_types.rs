use ic_kit::{candid::CandidType, Principal};
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

#[derive(Deserialize, CandidType)]
pub enum Event {
    Addition {
        time: u64,
        by: Principal, // The trusted source who has committed this action
        registry: Principal, // The PID of the contacted registry
        canister: Principal, // The PID of the deleted canister
        metadata: CanisterMetadata, // The metadata of the canister
    },
    Deletion {
        time: u64,
        by: Principal, // The trusted source who has committed this action
        registry: Principal, // The PID of the contacted registry
        canister: Principal, // The PID of the deleted canister
    },
    TrustedSourceAddition {
        time: u64,
        by: Principal, // The admin who has added the trusted source
        trusted_source: Principal, // The PID of the trusted source
        accessible_registries: Vec<Prinicpal>, // A vector of all registries that the trusted source can access
    },
    TrustedSourceDeletion {
        time: u64,
        by: Principal, // The admin who has deleted the trusted source from the proxy canister
        trusted_sourced: Principal, // The PID of the deleted trusted source
    },
    AccessChange {
        time: u64,
        by: Principal, // The admin who has changed the accessible registries list of the trusted source
        trusted_source: Principal, // The affected trusted source
        new_accessible_registries: Vec<Principal>, // The new vector of all registries that the trusted source can access
    }
}

pub struct History(Vec<Event>);
