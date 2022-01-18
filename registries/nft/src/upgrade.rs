use crate::nft::{Controller, NftCanister, Registry};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(String, NftCanister)>,
    controller: Principal,
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, NftCanister)>,
    controller: Principal,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<Registry>().archive();
    let controller = ic::get_mut::<Controller>().0;

    let stable = StableStorage { db, controller };

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
        let mut updated_nft_canisters = Vec::with_capacity(stable.db.len());

        for (_key, nft_canister) in stable.db.into_iter().enumerate() {
            let mut nft_canister_metadata: NftCanister = nft_canister.1.into();
            updated_nft_canisters.push((nft_canister_metadata.principal_id, nft_canister_metadata));
        }

        ic::get_mut::<Registry>().load(updated_nft_canisters);
        ic::store(Controller(stable.controller));
    }
}
