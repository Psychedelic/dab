use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::*;
use ic_kit::ic::*;
use ic_kit::macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use validator::validate_url;

const MAX_DESCRIPTION_LIMIT: usize = 1201;
const MAX_DISPLAY_NAME_LIMIT: usize = 25;

#[derive(Deserialize, CandidType, Clone)]
pub struct CanisterMetadata {
    principal_id: Principal,
    description: Option<String>,
    url: Option<String>,
    idl: Option<String>,
    logo_url: Option<String>,
    version: u32,
}

fn is_controller(canister_id: &Principal, account: &Principal) -> bool {
    return true;
}

#[derive(Default)]
pub struct CanisterDB(BTreeMap<String, CanisterMetadata>);

impl CanisterDB {
    pub fn archive(&mut self) -> Vec<(String, CanisterMetadata)> {
        let map = std::mem::replace(&mut self.0, BTreeMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(String, CanisterMetadata)>) {
        self.0 = archive.into_iter().collect();
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
        assert!(is_controller(&metadata.principal_id, &account));
        // Todo: account should be verified. No one other than canister's controllers should be able to update the information.
        self.0.insert(canister, metadata);
    }

    pub fn set_description(&mut self, account: Principal, canister: &String, description: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(&x.principal_id, &account));
                x.description = Some(description);
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_url(&mut self, account: Principal, canister: &String, url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(&x.principal_id, &account));
                x.url = Some(url);
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_logo(&mut self, account: Principal, canister: &String, logo_url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(&x.principal_id, &account));
                x.logo_url = Some(logo_url);
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_idl(&mut self, account: Principal, canister: &String, idl: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(&x.principal_id, &account));
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
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_info(&canister)
}

#[update]
fn add_canister(canister: String, metadata: CanisterMetadata) {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(caller(), canister, metadata);
}

#[update]
fn set_url(canister: String, url: String) {
    if validate_url(&url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_url(caller(), &canister, url);
    }
}

#[update]
fn set_description(canister: String, description: String) {
    if &description.len() < &MAX_DESCRIPTION_LIMIT {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_description(caller(), &canister, description);
    }
}

#[update]
fn set_idl(canister: String, idl: String) {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.set_idl(caller(), &canister, idl);
}

#[update]
fn set_logo(canister: String, logo_url: String) {
    if validate_url(&logo_url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_logo(caller(), &canister, logo_url);
    }
}