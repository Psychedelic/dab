use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use validator::validate_url;

const MAX_DESCRIPTION_LIMIT: usize = 1201;
const MAX_DISPLAY_NAME_LIMIT: usize = 25;

#[derive(Deserialize, CandidType, Clone)]
pub struct CanisterMetadata {
    principal_id: Option<Principal>,
    description: Option<String>,
    url: Option<String>,
    idl: Option<String>,
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
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(String, CanisterMetadata)>) {
        self.0 = archive.into_iter().collect();
        // self.0.reserve(25_000 - self.0.len());
    }

    pub fn get_info(&mut self, canister: &String) -> Option<CanisterMetadata> {
        self.0.get(canister).cloned()
    }

    pub fn add_canister(
        &mut self,
        account: Principal,
        canister: String,
        metadata: CanisterMetadata,
    ) {
        assert_eq!(metadata.version, 0);
        // Todo: account should be verified. No one other than canister's controllers should be able to update the information.
        self.0.insert(canister, metadata);
    }

    pub fn set_description(&mut self, account: Principal, canister: &String, description: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                x.description = Some(description);
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_url(&mut self, account: Principal, canister: &String, url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                x.url = Some(url);
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_idl(&mut self, account: Principal, canister: &String, idl: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                x.idl = Some(idl);
                x.version += 1;
            }
            None => return,
        }
    }
}

#[query]
fn name() -> String {
    String::from("Registry Canister")
}

#[update]
fn get_info(canister: String) -> Option<CanisterMetadata> {
    let canister_db = storage::get_mut::<CanisterDB>();
    canister_db.get_info(&canister)
}

#[update]
fn add_canister(canister: String, metadata: CanisterMetadata) {
    let canister_db = storage::get_mut::<CanisterDB>();
    canister_db.add_canister(caller(), canister, metadata);
}

#[update]
fn set_url(canister: String, url: String) {
    if validate_url(&url) {
        let canister_db = storage::get_mut::<CanisterDB>();
        canister_db.set_url(caller(), &canister, url);
    }
}

#[update]
fn set_description(canister: String, description: String) {
    if &description.len() < &MAX_DESCRIPTION_LIMIT {
        let canister_db = storage::get_mut::<CanisterDB>();
        canister_db.set_description(caller(), &canister, description);
    }
}

#[update]
fn set_idl(canister: String, idl: String) {
    let canister_db = storage::get_mut::<CanisterDB>();
    canister_db.set_idl(caller(), &canister, idl);
}
