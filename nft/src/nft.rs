use std::collections::HashMap;
use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;

fn is_controller() -> bool {
    true
}

#[derive(CandidType, Deserialize)]
pub struct NftCanister {
    principal_id: Principal,
    name: String,
    standard: String
}

pub struct Registry(HashMap<String, NftCanister>);

impl Default for Registry {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Registry {
    pub fn add(&mut self, name: String, canister_info: NftCanister) -> String {
        self.0.insert(name, canister_info);
        String::from("Operation was successful.")
    }

    pub fn get_all(&self) -> Vec<&NftCanister> {
        self.0.values().collect()
    }

    pub fn remove(&mut self, name: &String) -> String {
        if self.0.contains_key(name) {
            self.0.remove(name);
            return String::from("Operation was successful.");
        }

        String::from("No such entry exists in the registry.")
    }
}

#[query]
fn name() -> String {
    String::from("NFT Registry Canister")
}

#[update]
fn add(canister_info: NftCanister) -> String {
    if !is_controller() {
        return String::from("You are not authorized to add and delete canisters.");
    }

    let name = canister_info.name.clone();
    if name.len() <= 120 {
        let db = storage::get_mut::<Registry>();
        return db.add(name, canister_info);
    }

    String::from("The name of this canister has exceeded the limitation of 120 characters.")
}

#[update]
fn get_all() -> Vec<&'static NftCanister> {
    let db = storage::get::<Registry>();
    db.get_all()
}

#[update]
fn remove(name: String) -> String {
    if !is_controller() {
        return String::from("You are not authorized to add and delete canisters.");
    }

    let db = storage::get_mut::<Registry>();
    db.remove(&name)
}