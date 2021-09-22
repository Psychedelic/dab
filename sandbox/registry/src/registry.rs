use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::*;
use ic_kit::ic::*;
use ic_kit::macros::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 24;

#[derive(Deserialize, CandidType, Clone)]
pub struct CanisterMetadata {
    name: String,
    description: String,
    url: String,
    idl: String,
    logo_url: String,
    version: u32,
}

fn is_controller(canister_id: &Principal, account: &Principal) -> bool {
    return true;
}

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

    pub fn get_info(&mut self, canister: &Principal) -> Option<CanisterMetadata> {
        self.0.get(canister).cloned()
    }

    pub fn add_canister(
        &mut self,
        account: Principal,
        canister: Principal,
        metadata: CanisterMetadata,
    ) {
        assert!(is_controller(&canister, &account));
        // Todo: account should be verified. No one other than canister's controllers should be able to update the information.
        self.0.insert(canister, metadata);
    }

    pub fn set_description(&mut self, account: Principal, canister: &Principal, description: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.description = description;
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_url(&mut self, account: Principal, canister: &Principal, url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.url = url;
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_logo(&mut self, account: Principal, canister: &Principal, logo_url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.logo_url = logo_url;
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_idl(&mut self, account: Principal, canister: &Principal, idl: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.idl = idl;
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
fn get_info(canister: Principal) -> Option<CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_info(&canister)
}

#[update]
fn add_canister(canister: Principal, metadata: CanisterMetadata) {
    assert_eq!(&metadata.version, &0);
    if &metadata.name.len() > &NAME_LIMIT || &metadata.description.len() > &DESCRIPTION_LIMIT {
        return;
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(caller(), canister, metadata);
}

#[update]
fn set_url(canister: Principal, url: String) {
    if validate_url(&url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_url(caller(), &canister, url);
    }
}

#[update]
fn set_description(canister: Principal, description: String) {
    if &description.len() < &DESCRIPTION_LIMIT {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_description(caller(), &canister, description);
    }
}

#[update]
fn set_idl(canister: Principal, idl: String) {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.set_idl(caller(), &canister, idl);
}

#[update]
fn set_logo(canister: Principal, logo_url: String) {
    if validate_url(&logo_url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_logo(caller(), &canister, logo_url);
    }
}