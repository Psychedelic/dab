use crate::nft::{Controller, NftCanister, NftCanisterV0, Registry};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(String, NftCanisterV0)>,
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
            let nft_canister_metadata: NftCanisterV0 = nft_canister.1.into();
            
            let updated_nft_canister_metadata: NftCanister = NftCanister {
                principal_id: nft_canister_metadata.principal_id,
                name: nft_canister_metadata.name.clone(),
                description: nft_canister_metadata.description.clone(),
                thumbnail: nft_canister_metadata.icon.clone(),
                frontend: None,
                details: vec![(String::from("standard"), nft_canister_metadata.standard)],
            };

            updated_nft_canisters.push((updated_nft_canister_metadata.principal_id, updated_nft_canister_metadata));
        }

        ic::get_mut::<Registry>().load(updated_nft_canisters);
        ic::store(Controller(stable.controller));
    }
}
