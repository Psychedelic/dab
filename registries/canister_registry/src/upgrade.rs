// Project imports
use crate::registry::{CanisterDB, CanisterMetadata, Fleek};
use std::fmt::{self, Debug};

// IC imports
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub enum CanisterCategory {
    Tools,
    NFT,
    Service,
    Token,
    Social,
    Games,
}

impl fmt::Display for CanisterCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadataV0 {
    name: String,
    description: String,
    url: String,
    logo_url: String,
    category: CanisterCategory,
    version: u32,
}

impl From<CanisterMetadataV0> for CanisterMetadata {
    fn from(cs: CanisterMetadataV0) -> Self {
        CanisterMetadata {
            name: cs.name,
            description: cs.description,
            frontend: Some(cs.url),
            thumbnail: cs.logo_url,
            principal_id: Principal::management_canister(),
            details: vec![(String::from("category"), cs.category.to_string())],
        }
    }
}

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(Principal, CanisterMetadataV0)>,
    fleek: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, CanisterMetadata)>,
    fleek: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<CanisterDB>().archive();
    let fleek = ic::get_mut::<Fleek>().0.clone();

    let stable = StableStorage { db, fleek };

    match ic::stable_store((stable,)) {
        Ok(_) => (),
        Err(candid_err) => {
            trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
pub fn post_upgrade() {
    if let Ok((stable,)) = ic::stable_restore::<(StableStorageV0,)>() {
        let mut canister_list = Vec::with_capacity(stable.db.len());

        for (_key, canister_info) in stable.db.into_iter().enumerate() {
            let mut metadata_info: CanisterMetadata = canister_info.1.into();
            let principal_info: Principal = canister_info.0.into();
            metadata_info.principal_id = principal_info;

            canister_list.push((principal_info, metadata_info));
        }

        ic::get_mut::<CanisterDB>().load(canister_list);

        //ic::get_mut::<CanisterDB>().load(stable.db);
        ic::store(Fleek(stable.fleek));
    }
}
