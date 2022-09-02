use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::{collections::HashMap, str::FromStr};
use validator::validate_url;

use crate::common_types::*;
use crate::management::*;

#[init]
pub fn init(canister_registry: Option<Principal>) {
    ic::store(Admins(vec![ic::caller()]));
    ic::store(Registry(
        HashMap::new(),
        canister_registry.unwrap_or(CANISTER_REGISTRY_ID.try_into().unwrap()),
    ));
}

// (registry map, canister registry id)
pub struct Registry(HashMap<Principal, NftCanister>, Principal);
impl Default for Registry {
    fn default() -> Self {
        Registry(
            HashMap::new(),
            Principal::from_str(CANISTER_REGISTRY_ID).unwrap(),
        )
    }
}

impl Registry {
    pub fn archive(&mut self) -> Vec<(Principal, NftCanister)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, NftCanister)>) {
        assert!(self.0.is_empty());
        self.0 = archive.into_iter().collect();
    }

    pub fn add(
        &mut self,
        caller: &Principal,
        canister_info: AddNftInput,
    ) -> Result<(), OperationError> {
        let nft = self.0.get(&canister_info.principal_id);

        // If its an update, check if the caller matches the submitter or if its an admin
        if nft.is_some() && !is_admin(caller) && nft.unwrap().submitter != *caller {
            return Err(OperationError::NotAuthorized);
        }

        // An admin can update any entry
        if nft.is_some() && is_admin(caller) {
            let updated_nft = NftCanister {
                name: canister_info.name,
                description: canister_info.description,
                thumbnail: canister_info.thumbnail,
                frontend: canister_info.frontend,
                principal_id: canister_info.principal_id,
                submitter: nft.unwrap().submitter,
                last_updated_by: *caller,
                last_updated_at: ic::time(),
                details: canister_info.details.clone(),
            };

            self.0.insert(canister_info.principal_id, updated_nft);
        }
        // Its a new entry
        else {
            let new_nft = NftCanister {
                name: canister_info.name,
                description: canister_info.description,
                thumbnail: canister_info.thumbnail,
                frontend: canister_info.frontend,
                principal_id: canister_info.principal_id,
                submitter: *caller,
                last_updated_by: *caller,
                last_updated_at: ic::time(),
                details: canister_info.details.clone(),
            };

            self.0.insert(canister_info.principal_id, new_nft);
        }

        Ok(())
    }

    pub fn remove(
        &mut self,
        caller: &Principal,
        principal_id: &Principal,
    ) -> Result<(), OperationError> {
        if !self.0.contains_key(principal_id) {
            return Err(OperationError::NonExistentItem);
        }

        let nft = self.0.get(principal_id).unwrap();

        if nft.submitter != *caller && !is_admin(caller) {
            return Err(OperationError::NotAuthorized);
        }

        self.0.remove(principal_id);

        return Ok(());
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
pub async fn add(
    trusted_source: Option<Principal>,
    canister_info: AddNftInput,
) -> Result<(), OperationError> {
    let caller = ic::caller();
    if !is_admin(&caller) {
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
        let mut call_arg = canister_info.clone();
        call_arg.details = vec![("category".to_string(), DetailValue::Text("NFT".to_string()))];
        let canister_registry = ic::get::<Registry>().1;

        // set canister registry to `aaaaa-aa` to skip canister registry insertion
        if canister_registry != Principal::management_canister() {
            let _registry_add_response: RegistryResponse = match ic::call(
                canister_registry,
                "add",
                (trusted_source.unwrap_or(ic::id()), call_arg),
            )
            .await
            {
                Ok((x,)) => x,
                Err((_code, msg)) => {
                    return Err(OperationError::Unknown(msg));
                }
            };
        }

        let db = ic::get_mut::<Registry>();
        return db.add(&trusted_source.unwrap_or(caller), canister_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
pub fn remove(
    trusted_source: Option<Principal>,
    principal_id: Principal,
) -> Result<(), OperationError> {
    let caller = ic::caller();
    if !is_admin(&caller) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registry>();
    db.remove(&trusted_source.unwrap_or(caller), &principal_id)
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
