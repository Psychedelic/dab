use ic_kit::{candid::CandidType, candid::Int, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum AddressType {
    PrincipalId(Principal),
    AccountId(String),
    Icns(String),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Address {
    pub name: String,
    pub value: AddressType,
    pub description: Option<String>,
    pub emoji: Option<String>,
}

#[derive(Deserialize, CandidType)]
pub struct GetRecordResponse {
    pub ttl: u64,
    controller: Principal,
    resolver: Principal,
    owner: Principal,
    operator: Principal,
    name: String,
    expiry: Int,
}

pub type Key = (Principal, String);

pub const DESCRIPTION_LIMIT: usize = 1200;
pub const NAME_LIMIT: usize = 24;
pub const ACCOUNT_ID_LENGTH: usize = 64;
pub const ICNS_REGISTRY_PRINCIPAL_ID: &str = "e5kvl-zyaaa-aaaan-qabaq-cai";
pub const DEFAULT_LIMIT: usize = 20;

#[derive(CandidType, Debug, PartialEq)]
pub enum OperationError {
    NotAuthorized,
    BadParameters,
    NonExistentItem,
    Unknown(String),
}
