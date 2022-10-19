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

    pub fn add_canister(
        &mut self,
        caller: &Principal,
        metadata: AddCanisterInput,
    ) -> Result<(), OperationError> {
        let canister = self.0.get(&metadata.principal_id);

        // If its an update, check if the caller matches the submitter or if its an admin
        if canister.is_some() && !is_admin(caller) && canister.unwrap().submitter != *caller {
            return Err(OperationError::NotAuthorized);
        }
        // An admin can update any entry
        else if canister.is_some() && is_admin(caller) {
            let updated_canister = CanisterMetadata {
                name: metadata.name,
                description: metadata.description,
                thumbnail: metadata.thumbnail,
                frontend: metadata.frontend,
                principal_id: metadata.principal_id,
                submitter: canister.unwrap().submitter,
                last_updated_by: *caller,
                last_updated_at: ic::time(),
                details: metadata.details.clone(),
            };

            self.0.insert(metadata.principal_id, updated_canister);
        }
        // Its a new entry
        else {
            let new_canister = CanisterMetadata {
                name: metadata.name,
                description: metadata.description,
                thumbnail: metadata.thumbnail,
                frontend: metadata.frontend,
                principal_id: metadata.principal_id,
                submitter: *caller,
                last_updated_by: *caller,
                last_updated_at: ic::time(),
                details: metadata.details.clone(),
            };

            self.0.insert(metadata.principal_id, new_canister);
        }

        return Ok(());
    }

    pub fn remove_canister(
        &mut self,
        caller: &Principal,
        principal_id: &Principal,
    ) -> Result<(), OperationError> {
        if !self.0.contains_key(principal_id) {
            return Err(OperationError::NonExistentItem);
        }

        let canister = self.0.get(principal_id).unwrap();

        if canister.submitter != *caller && !is_admin(caller) {
            return Err(OperationError::NotAuthorized);
        }

        self.0.remove(principal_id);

        return Ok(());
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

#[query]
pub fn get_multiple(canister_ids: Vec<Principal>) -> Vec<Option<&'static CanisterMetadata>> {
    let response = vec![];
    let canister_db = ic::get_mut::<CanisterDB>();

    for canister_id in canister_ids {
        let canister = canister_db.get_info(canister_id);
        response.push(canister);
    }

    return response;
}

#[update]
pub fn add(
    trusted_source: Option<Principal>,
    metadata: AddCanisterInput,
) -> Result<(), OperationError> {
    let caller = ic::caller();

    if !is_admin(&caller) {
        return Err(OperationError::NotAuthorized);
    } else if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.thumbnail)
        || !metadata.clone().frontend.map(validate_url).unwrap_or(true)
        || (metadata.details.len() != 1 && metadata.details[0].0 != String::from("category"))
    {
        return Err(OperationError::BadParameters);
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(&trusted_source.unwrap_or(caller), metadata)
}

#[update]
pub fn remove(
    trusted_source: Option<Principal>,
    canister: Principal,
) -> Result<(), OperationError> {
    let caller = ic::caller();
    if !is_admin(&caller) {
        return Err(OperationError::NotAuthorized);
    }
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.remove_canister(&trusted_source.unwrap_or(caller), &canister)
}

#[query]
pub fn get_all() -> Vec<&'static CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_all()
}
