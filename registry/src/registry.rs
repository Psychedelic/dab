use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use validator::validate_url;

const MAX_DESCRIPTION_LIMIT  : usize = 1201;
const MAX_DISPLAY_NAME_LIMIT : usize = 25;

#[derive(Deserialize, CandidType, Clone)]
pub struct CanisterMetadata {
    principalID: Option<Principal>,
    description: Option<String>,
    keywords: Option<Vec>,
    url: Option<String>,
    IDL: Option<String>,
    version: u32,
}

pub struct CanisterDB(BTreeMap<String, CanisterMetadata>);

impl Default for CanisterDB {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl CanisterDB {
    pub fn archive(&mut self) -> Vec<(String, CanisterMetadata)> {
        let map = std::mem::replace(&mut self.0, BTreeMap::new());
        map.into_iter()
            .collect()
    }
    
    pub fn load(&mut self, archive: Vec<(String, CanisterMetadata)>) {
        self.0 = archive.into_iter().collect();
        // self.0.reserve(25_000 - self.0.len());
    }

    pub fn get_info(&mut self, canister: &String) -> Option<CanisterMetadata> {
        self.0.get(canister).cloned()
    }

    pub fn add_canister(&mut self, account: Principal, metadata: CanisterMetadata) {
        
    }

    pub fn set_description(&mut self, account: Principal, canister: String, description: String) {
        match self.0.get_mut(&canister) {
            Some(x) => {
                x.description = Some(description);
                x.version += 1;
            }
            None => return,
        }
    }
}

#[query]
fn name() -> String {
    String::from("Profile Canister")
}

#[update]
fn get_info(account: Option<Principal>) -> Option<CanisterMetadata> {
    let canister_db = storage::get_mut::<CanisterDB>();
    canister_db.get_profile(&account.unwrap_or_else(|| caller()))
}

#[update]
fn set_description(description: String) {
    if &description.len() < &MAX_DESCRIPTION_LIMIT {
        let canister_db = storage::get_mut::<CanisterDB>();
        canister_db.set_description(caller(), description);
    }
}

#[update]
fn add_canister(name: String, metadata: CanisterMetadata) {
    if &metadata.description.len() < &MAX_DESCRIPTION_LIMIT {
        let canister_db = storage::get_mut::<CanisterDB>();
        canister_db.add_canister(caller(), name, metadata);
    }
}

