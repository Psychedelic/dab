use ic_kit::candid::{CandidType, Deserialize, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;

use crate::common_types::*;
use crate::management::*;

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

#[derive(Default)]
pub struct TrustedSources(HashMap<Principal, TrustedSource>);

impl TrustedSources {
    pub fn archive(&mut self) -> Vec<(Principal, TrustedSource)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, TrustedSource)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn is_trusted_source(&self, principal_id: &Principal) -> bool {
        self.0.contains_key(principal_id)
    }

    pub fn add(&mut self, trusted_source: AddTrustedSourceInput) -> Result<(), OperationError> {
        let new_trusted_source = TrustedSource {
            added_by: ic::caller(),
            principal_id: trusted_source.principal_id,
            accessible_registries: trusted_source.accessible_registries,
            last_call: 0,
        };

        self.0
            .insert(trusted_source.principal_id, new_trusted_source);

        return Ok(());
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&TrustedSource> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&TrustedSource> {
        self.0.values().collect()
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if !self.0.contains_key(principal_id) {
            return Err(OperationError::NonExistentItem);
        }

        self.0.remove(principal_id);
        return Ok(());
    }
}

#[init]
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

#[query]
pub fn name() -> String {
    String::from("Proxy Canister")
}

#[update]
pub fn add(trusted_source: AddTrustedSourceInput) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) || ic::get::<TrustedSources>().is_trusted_source(&ic::caller()) {
        let db = ic::get_mut::<TrustedSources>();
        return db.add(trusted_source);
    }

    return Err(OperationError::NotAuthorized);
}

#[query]
pub fn get(principal_id: Principal) -> Option<&'static TrustedSource> {
    let db = ic::get_mut::<TrustedSources>();
    db.get(&principal_id)
}

#[query]
pub fn get_all() -> Vec<&'static TrustedSource> {
    let db = ic::get_mut::<TrustedSources>();
    db.get_all()
}

#[update]
pub fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) || ic::get::<TrustedSources>().is_trusted_source(&ic::caller()) {
        let db = ic::get_mut::<TrustedSources>();
        db.remove(&principal_id)?
    }

    return Err(OperationError::NotAuthorized);
}
