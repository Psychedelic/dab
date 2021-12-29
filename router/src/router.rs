use ic_kit::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

pub struct Admins(pub Vec<Principal>);

impl Default for Admins {
    fn default() -> Self {
        panic!()
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Registry {
    pub principal_id: Principal,
    pub name: String,
    pub description: String,
    pub logo_url: String,
    pub front_end: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputAddRegistry {
    principal_id: Principal,
    name: String,
    description: String,
    logo_url: String,
    front_end: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputEditRegistry {
    principal_id: Principal,
    name: Option<String>,
    description: Option<String>,
    logo_url: Option<String>,
}

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

    pub fn add(&mut self, registry_info: InputAddRegistry) -> Result<(), OperationError> {
        let registry = Registry {
            principal_id: registry_info.principal_id,
            name: registry_info.name,
            description: registry_info.description,
            logo_url: registry_info.logo_url,
            front_end: registry_info.front_end,
        };

        self.0.insert(registry_info.principal_id.clone(), registry);
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

#[init]
fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

fn is_admin(account: &Principal) -> bool {
    ic::get::<Admins>().0.contains(account)
}

#[update]
fn set_admin(new_admin: Principal) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) {
        ic::get_mut::<Admins>().0.push(new_admin);
        return Ok(());
    }
    Err(OperationError::NotAuthorized)
}

#[query]
fn name() -> String {
    String::from("Router Canister")
}

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentRegistry,
    BadParameters,
}

#[update]
fn add(registry_info: InputAddRegistry) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if !validate_url(&registry_info.logo_url) {
        return Err(OperationError::BadParameters);
    }

    let name = registry_info.name.clone();
    if name.len() <= 120 && &registry_info.description.len() <= &1200 {
        let db = ic::get_mut::<Registries>();
        return db.add(registry_info);
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