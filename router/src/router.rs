use crate::common_types::*;
use crate::management::*;

use ic_kit::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

#[derive(Default)]
pub struct Registries(HashMap<Principal, Registry>);

impl Registries {
    pub fn archive(&mut self) -> Vec<(Principal, Registry)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, Registry)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, canister: Principal, registry_info: Registry) -> Result<(), OperationError> {
        self.0.insert(canister, registry_info);
        Ok(())
    }
    

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if self.0.contains_key(principal_id) {
            self.0.remove(principal_id);
            return Ok(());
        }

        Err(OperationError::NonExistentRegistry)
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&Registry> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&Registry> {
        self.0.values().collect()
    }
}

#[query]
fn name() -> String {
    String::from("Router Canister")
}

#[update]
fn add(canister: Principal, registry_info: Registry) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if !validate_url(&registry_info.logo_url) {
        return Err(OperationError::BadParameters);
    }

    let name = registry_info.name.clone();
    if name.len() <= 120 && &registry_info.description.len() <= &1200 {
        let db = ic::get_mut::<Registries>();
        return db.add(canister, registry_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registries>();
    db.remove(&principal_id)
}

#[query]
fn get_registry(principal_id: Principal) -> Option<&'static Registry> {
    let db = ic::get_mut::<Registries>();
    db.get(&principal_id)
}

#[query]
fn get_all() -> Vec<&'static Registry> {
    let db = ic::get_mut::<Registries>();
    db.get_all()
}