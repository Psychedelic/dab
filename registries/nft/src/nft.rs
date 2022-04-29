use ic_cdk::export::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::{collections::HashMap, str::FromStr};
use validator::validate_url;

use crate::common_types::*;
use crate::management::*;

#[init]
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

#[derive(Default)]
pub struct Registry(HashMap<Principal, NftCanister>);

impl Registry {
    pub fn archive(&mut self) -> Vec<(Principal, NftCanister)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, NftCanister)>) {
        assert!(self.0.is_empty());
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, canister_info: NftCanister) -> Result<(), OperationError> {
        self.0.insert(canister_info.principal_id, canister_info);
        Ok(())
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if self.0.remove(&principal_id).is_some() {
            return Ok(());
        }

        Err(OperationError::NonExistentItem)
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&NftCanister> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&NftCanister> {
        self.0.values().collect()
    }

    pub fn get_all_paginated(
        &self,
        offset: usize,
        _limit: usize,
    ) -> Result<Vec<&NftCanister>, OperationError> {
        let nfts: Vec<&NftCanister> = self.0.values().collect();

        if offset > nfts.len() {
            return Err(OperationError::BadParameters(String::from(
                "Offset out of bound.",
            )));
        }

        let mut limit = _limit;

        if offset + _limit > nfts.len() {
            limit = nfts.len() - offset;
        }

        return Ok(nfts[offset..(offset + limit)].to_vec());
    }

    pub fn get_amount(&self) -> usize {
        return self.0.values().len();
    }
}

#[query]
fn name() -> String {
    String::from("NFT Registry Canister")
}

#[update]
pub async fn add(canister_info: NftCanister) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if canister_info.name.len() > NAME_LIMIT {
        return Err(OperationError::BadParameters(
            format!(
                "Name field has to be less than {} characters long.",
                NAME_LIMIT
            )
            .to_string(),
        ));
    }

    if canister_info.description.len() > DESCRIPTION_LIMIT {
        return Err(OperationError::BadParameters(
            format!(
                "Description field has to be less than {} characters long.",
                DESCRIPTION_LIMIT
            )
            .to_string(),
        ));
    }

    if !validate_url(canister_info.clone().thumbnail) {
        return Err(OperationError::BadParameters(String::from(
            "Thumbnail field has to be a url.",
        )));
    }

    if canister_info.frontend.is_some()
        && !validate_url(canister_info.clone().frontend.unwrap())
    {
        return Err(OperationError::BadParameters(String::from(
            "Frontend field has to be a url.",
        )));
    }

    if canister_info.details.len() < 1 {
        return Err(OperationError::BadParameters(String::from("Details has to have standard field.")));
    }

    if canister_info.details[0].0 != String::from("standard") {
        return Err(OperationError::BadParameters(String::from(
            "First detail field has to be standard.",
        )));
    }

    // Add the collection to the canister registry
    let mut call_arg: NftCanister = canister_info.clone();
    call_arg.details = vec![("category".to_string(), DetailValue::Text("NFT".to_string()))];

    let _registry_add_response: RegistryResponse = match ic::call(
        Principal::from_str(CANISTER_REGISTRY_ID).unwrap(),
        "add",
        (call_arg,),
    )
    .await
    {
        Ok((x,)) => x,
        Err((_code, msg)) => {
            return Err(OperationError::Unknown(msg));
        }
    };

    let db = ic::get_mut::<Registry>();
    return db.add(canister_info);
}

#[update]
pub fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registry>();
    db.remove(&principal_id)
}

#[query]
pub fn get(principal_id: Principal) -> Option<&'static NftCanister> {
    let db = ic::get_mut::<Registry>();
    db.get(&principal_id)
}

#[query]
pub fn get_all() -> Vec<&'static NftCanister> {
    let db = ic::get_mut::<Registry>();
    db.get_all()
}

#[query]
pub fn get_all_paginated(
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<GetAllPaginatedResponse, OperationError> {
    let db = ic::get_mut::<Registry>();
    let nfts = db.get_all_paginated(offset.unwrap_or(0), limit.unwrap_or(DEFAULT_LIMIT))?;
    let amount = db.get_amount();

    return Ok(GetAllPaginatedResponse { nfts, amount });
}
