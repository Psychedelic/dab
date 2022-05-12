use ic_kit::candid::{Principal, CandidType};
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;

use crate::common_types::*;
use crate::management::*;

#[derive(Default)]
pub struct TrustedSources(HashMap<Principal, TrustedSource>);

impl TrustedSources {
    pub fn archive(&mut self) -> Vec<(Principal, TrustedSource)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, TrustedSource)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, trusted_source: AddTrustedSourceInput) -> Result<(), OperationError> {
        let new_trusted_source = TrustedSource {
            added_by: ic::caller(),
            principal_id: trusted_source.principal_id,
            accessible_registries: trusted_source.accessible_registries,
            last_call: 0,
        };

        self.0
            .insert(trusted_source.principal_id, new_trusted_source);

        return Ok(());
    }

    pub fn has_access_to_registry(&self, principal_id: &Principal, registry_principal_id: &Principal) -> bool {
        match self.0.get(principal_id) {
            Some(v) if v.accessible_registries.contains(registry_principal_id) => true,
            _ => false
        }
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&TrustedSource> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&TrustedSource> {
        self.0.values().collect()
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if !self.0.contains_key(principal_id) {
            return Err(OperationError::NonExistentItem);
        }

        self.0.remove(principal_id);
        return Ok(());
    }
}

#[init]
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

#[query]
pub fn name() -> String {
    String::from("Proxy Canister")
}

#[derive(CandidType)]
pub struct AddRegistryInput {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub submitter: Principal,
    pub last_updated_by: Principal,
    pub last_updated_at: u64,
    pub details: Vec<(String, DetailValue)>,
}

#[update]
pub async fn add_registry() -> Result<(), OperationError> {
    if ic::get::<TrustedSources>().has_access_to_registry(&ic::caller(), &principal_id) {
        return Err(OperationError::NotAuthorized);
    }

    let add_registry_input = AddRegistryInput {
        name: String::from("TEST TOKEN"),
        description: String::from("TEST DESCRIPTION"),
        thumbnail: String::from("www.thumbnail.com"),
        frontend: Some(String::from("www.frontend.com")),
        principal_id: Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap(),
        submitter: ic::caller(),
        last_updated_by: ic::caller(),
        last_updated_at: 0,
        details: vec![
            (String::from("symbol"), DetailValue::Text(String::from("TEST"))),
            (String::from("standard"), DetailValue::Text(String::from("TEST"))),
            (String::from("total_supply"), DetailValue::Text(String::from("TEST"))),
            (String::from("verified"), DetailValue::True),
            (String::from("decimals"), DetailValue::Text(String::from("TEST"))),
            (String::from("fee"), DetailValue::Text(String::from("TEST"))),
        ],
    };

    let response: (Option<String>,) = ic::call(Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(), "add", (add_registry_input,)).await.unwrap();

    ic::print(response.0.is_some().to_string());

    return Ok(());
}

#[update]
pub fn add(trusted_source: AddTrustedSourceInput) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TrustedSources>();
    return db.add(trusted_source);
}

#[query]
pub fn get(principal_id: Principal) -> Option<&'static TrustedSource> {
    let db = ic::get_mut::<TrustedSources>();
    return db.get(&principal_id);
}

#[query]
pub fn get_all() -> Vec<&'static TrustedSource> {
    let db = ic::get_mut::<TrustedSources>();
    return db.get_all();
}

#[update]
pub fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TrustedSources>();
    return db.remove(&principal_id);
}
