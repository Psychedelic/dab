use ic_cdk::export::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;
use validator::validate_url;

use crate::common_types::*;
use crate::management::{is_admin, Admins};

#[derive(Default)]
pub struct CanisterDB(HashMap<Principal, CanisterMetadata>);

impl CanisterDB {
    pub fn archive(&mut self) -> Vec<(Principal, CanisterMetadata)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, CanisterMetadata)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn get_info(&mut self, canister: Principal) -> Option<&CanisterMetadata> {
        self.0.get(&canister)
    }

    pub fn add_canister(&mut self, metadata: CanisterMetadata) -> Result<(), Failure> {
        let id: Principal = metadata.principal_id;
        self.0.insert(metadata.principal_id, metadata);
        if !self.0.contains_key(&id) {
            return Err(Failure::Unknown(String::from(
                "Something unexpected happend. Try again.",
            )));
        }
        Ok(())
    }

    pub fn remove_canister(&mut self, canister: &Principal) -> Result<(), Failure> {
        if !self.0.contains_key(canister) {
            return Err(Failure::NonExistentItem);
        }
        self.0.remove(canister);
        Ok(())
    }

    pub fn get_all(&self) -> Vec<&CanisterMetadata> {
        self.0.values().collect()
    }
}

#[init]
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

#[query]
fn name() -> String {
    String::from("Canister Registry")
}

#[query]
pub fn get(canister: Principal) -> Option<&'static CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_info(canister)
}

#[update]
pub fn add(metadata: CanisterMetadata) -> Result<(), Failure> {
    if !is_admin(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    } else if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.thumbnail)
        || !metadata.clone().frontend.map(validate_url).unwrap_or(true)
    {
        return Err(Failure::BadParameters);
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(metadata)
}

#[update]
pub fn remove(canister: Principal) -> Result<(), Failure> {
    if !is_admin(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    }
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.remove_canister(&canister)
}

#[query]
pub fn get_all() -> Vec<&'static CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_all()
}
