use crate::common_types::*;
use crate::management::*;

use ic_kit::*;
use ic_kit::candid::CandidType;
use ic_kit::macros::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

// The metadata structure that you want to store in your registry.
#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
    name: String,
    thumbnail: String,
    frontend: String,
    description: String,
}

// BTreeMaps and HashMaps are the solutions we follow to store our metadata.
#[derive(Default)]
pub struct CanisterDB(HashMap<Principal, CanisterMetadata>);

impl CanisterDB {
    // The archive method is called by the pre_upgrade method from the upgrade script.
    pub fn archive(&mut self) -> Vec<(Principal, CanisterMetadata)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    // The load method is called by the post_upgrade method from the upgrade script.
    pub fn load(&mut self, archive: Vec<(Principal, CanisterMetadata)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn get(&self, canister: Principal) -> Option<&CanisterMetadata> {
        self.0.get(&canister)
    }

    pub fn add(&mut self, canister: Principal, metadata: CanisterMetadata) -> Result<(), Error> {
        // Whether the canister has already been added to the registry or not
        // this method will add it to the hashmap. If it is already a part of 
        // the HashMap, its metadata will be replaced by the new metadata.
        self.0.insert(canister, metadata);
        return Ok(());
    }

    pub fn remove(&mut self, canister: Principal) -> Result<(), Error> {
        if self.0.contains_key(&canister) {
            // Canister has been added to the registry.
            // We can safely remove it from the registry.
            self.0.remove(&canister);
            return Ok(());
        } else {
            // No metadata has been associated with this canister principal id.
            // We can't remove it if it's not there.
            return Err(Error::NonExistantCanister);
        }
    }
}

#[query]
fn name() -> String {
    String::from("Template Registry Canister")
}

#[query]
fn get(canister: Principal) -> Option<&'static CanisterMetadata> {
    let db = ic::get::<CanisterDB>();
    db.get(canister)
}


// The add method will add new entries to the HashMap
// This method updates the entry if it already exists
#[update]
fn add(canister: Principal, metadata: CanisterMetadata) -> Result<(), Error> {
    if is_admin(&ic::caller()) {
        // The caller is one of the admins.
        // The next step is verifying URLs
        if validate_url(metadata.thumbnail.clone()) && validate_url(metadata.frontend.clone()) {
            let db = ic::get_mut::<CanisterDB>();
            return db.add(canister, metadata);
        }
        return Err(Error::BadParameters);
    }
    Err(Error::NotAuthorized)
}

#[update]
fn remove(canister: Principal) -> Result<(), Error> {
    if is_admin(&ic::caller()) {
        // The caller is one of the admins.
        let db = ic::get_mut::<CanisterDB>();
        return db.remove(canister);
    }
    Err(Error::NotAuthorized)
}