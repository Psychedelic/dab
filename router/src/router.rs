use crate::common_types::*;
use crate::management::*;

use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
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

    pub fn add(&mut self, registry_info: Registry) -> Result<(), OperationError> {
        self.0.insert(registry_info.principal_id, registry_info);
        Ok(())
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if self.0.remove(principal_id).is_some() {
            return Ok(());
        }

        Err(OperationError::NonExistentItem)
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&Registry> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&Registry> {
        self.0.values().collect()
    }
}

#[query]
pub fn name() -> String {
    String::from("Router Canister")
}

#[update]
pub fn add(registry_info: Registry) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if !validate_url(&registry_info.thumbnail) {
        return Err(OperationError::BadParameters);
    }

    let name = registry_info.name.clone();
    if name.len() <= 120
        && &registry_info.description.len() <= &1200
        && registry_info.details.len() == 1
        && registry_info.details[0].0 == String::from("verified")
        && (registry_info.details[0].1 == DetailValue::True
            || registry_info.details[0].1 == DetailValue::False)
    {
        let db = ic::get_mut::<Registries>();
        return db.add(registry_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
pub fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registries>();
    db.remove(&principal_id)
}

#[query]
pub fn get(principal_id: Principal) -> Option<&'static Registry> {
    let db = ic::get_mut::<Registries>();
    db.get(&principal_id)
}

#[query]
pub fn get_all() -> Vec<&'static Registry> {
    let db = ic::get_mut::<Registries>();
    db.get_all()
}
