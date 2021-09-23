use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 24;

pub struct Fleek(pub Vec<Principal>);

impl Default for Fleek {
    fn default() -> Self {
        panic!()
    }
}

#[derive(Deserialize, CandidType, Clone)]
pub struct CanisterMetadata {
    name: String,
    description: String,
    url: String,
    logo_url: String,
    version: u32,
}


#[derive(Deserialize, CandidType, Clone)]
pub struct InputCanisterMetadata {
    name: String,
    description: String,
    url: String,
    logo_url: String,
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

    pub fn get_info(&mut self, canisters: Vec<Principal>) -> Vec<Option<&CanisterMetadata>> {
        // self.0.get(canister).cloned()
        let mut list: Vec<Option<&CanisterMetadata>> = vec![];
        for canister in canisters {
            let item = self.0.get(&canister);
            list.push(item);
        }
        list
    }

    pub fn add_canister(&mut self, canister: Principal, metadata: InputCanisterMetadata) {
        let canister_metadata = CanisterMetadata {
            name: metadata.name,
            description: metadata.description,
            url: metadata.url,
            logo_url: metadata.logo_url,
            version: 0,
        };
        self.0.insert(canister, canister_metadata);
    }

    pub fn remove_canister(&mut self, canister: &Principal) {
        self.0.remove(canister);
    }

    pub fn get_all(&self) -> Vec<&CanisterMetadata> {
        self.0.values().collect()
    }

    /* pub fn set_description(&mut self, account: Principal, canister: &Principal, description: String) {
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
    } **/
}

#[init]
fn init() {
    ic::store(Fleek(vec![ic::caller()]));
}

fn is_fleek(account: &Principal) -> bool {
    ic::get::<Fleek>().0.contains(account)
}

#[update]
fn set_admin(new_admin: Principal) {
    if is_fleek(&ic::caller()) {
        ic::get_mut::<Fleek>().0.push(new_admin);
    }
}

#[query]
fn name() -> String {
    String::from("Canister Registry")
}

#[query]
fn get_info(canisters: Vec<Principal>) -> Vec<Option<&'static CanisterMetadata>> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_info(canisters)
}

#[query]
fn get_all() -> Vec<&'static CanisterMetadata> {
    let db = ic::get_mut::<CanisterDB>();
    db.get_all()
}

#[update]
fn add_canister(canister: Principal, metadata: InputCanisterMetadata) -> Option<String> {
    assert!(is_fleek(&ic::caller()), "{}", String::from("Not Fleek"));
    if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.logo_url)
        || !validate_url(&metadata.url)
    {
        return Some(String::from("Bad Parameters"));
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(canister, metadata);
    return Some(String::from("Operation Successful"));
}

#[update]
fn remove_canister(canister: Principal) {
    assert!(is_fleek(&ic::caller()));
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.remove_canister(&canister);
}

/* #[update]
async fn update_canister(canister: Principal) -> Result<Option<String>, String>{
    match ic::call(canister, registry, None).await {
        Ok(x) => {

        },
        Err((code, msg)) => {
            Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ))
        }
    }
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
fn set_logo(canister: Principal, logo_url: String) {
    if validate_url(&logo_url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_logo(caller(), &canister, logo_url);
    }
} **/
