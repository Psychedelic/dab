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

    pub fn add_canister(&mut self, metadata: CanisterMetadata) -> Result<(), OperationError> {
        let id: Principal = metadata.principal_id;
        self.0.insert(metadata.principal_id, metadata);
        if !self.0.contains_key(&id) {
            return Err(OperationError::Unknown(String::from(
                "Something unexpected happend. Try again.",
            )));
        }
        Ok(())
    }

    pub fn remove_canister(&mut self, canister: &Principal) -> Result<(), OperationError> {
        if !self.0.contains_key(canister) {
            return Err(OperationError::NonExistentItem);
        }
        self.0.remove(canister);
        Ok(())
    }

    pub fn get_all(&self) -> Vec<&CanisterMetadata> {
        self.0.values().collect()
    }

    pub fn get_all_paginated(
        &self,
        offset: usize,
        _limit: usize,
    ) -> Result<Vec<&CanisterMetadata>, OperationError> {
        let canisters: Vec<&CanisterMetadata> = self.0.values().collect();

        if offset > canisters.len() {
            return Err(OperationError::BadParameters(String::from(
                "Offset out of bound.",
            )));
        }

        let mut limit = _limit;

        if offset + _limit > canisters.len() {
            limit = canisters.len() - offset;
        }

        return Ok(canisters[offset..(offset + limit)].to_vec());
    }

    pub fn get_amount(&self) -> usize {
        return self.0.values().len();
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

#[update]
pub fn add(metadata: CanisterMetadata) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if metadata.name.len() > NAME_LIMIT {
        return Err(OperationError::BadParameters(format!(
            "Name field has to be less than {} characters long.",
            NAME_LIMIT
        )));
    }

    if metadata.description.len() > DESCRIPTION_LIMIT {
        return Err(OperationError::BadParameters(format!(
            "Description field has to be less than {} characters long.",
            DESCRIPTION_LIMIT
        )));
    }

    if !validate_url(&metadata.thumbnail) {
        return Err(OperationError::BadParameters(String::from(
            "Thumbnail field has to be a url.",
        )));
    }

    if metadata.frontend.is_some() && !validate_url(metadata.clone().frontend.unwrap()) {
        return Err(OperationError::BadParameters(String::from(
            "Frontend field has to be a url.",
        )));
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(metadata)
}

#[update]
pub fn remove(canister: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.remove_canister(&canister)
}

#[query]
pub fn get_all() -> Vec<&'static CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_all()
}

#[query]
pub fn get_all_paginated(
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<GetAllPaginatedResponse, OperationError> {
    let db = ic::get_mut::<CanisterDB>();
    let canisters = db.get_all_paginated(offset.unwrap_or(0), limit.unwrap_or(DEFAULT_LIMIT))?;
    let amount = db.get_amount();

    return Ok(GetAllPaginatedResponse { canisters, amount });
}
