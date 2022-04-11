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
}

#[query]
fn name() -> String {
    String::from("NFT Registry Canister")
}

#[update]
pub async fn add(canister_info: NftCanister) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    } else if !validate_url(&canister_info.thumbnail) {
        return Err(OperationError::BadParameters);
    } else if canister_info.frontend.is_some()
        && !validate_url(&canister_info.frontend.clone().unwrap())
    {
        return Err(OperationError::BadParameters);
    } else if canister_info.details[0].0 != String::from("standard") {
        return Err(OperationError::BadParameters);
    } else if canister_info.details.len() != 1 {
        return Err(OperationError::BadParameters);
    }

    let name = canister_info.name.clone();
    if name.len() <= NAME_LIMIT && &canister_info.description.len() <= &DESCRIPTION_LIMIT {
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

    Err(OperationError::BadParameters)
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
