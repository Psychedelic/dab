use crate::nft::{Controller, DetailValue, NftCanister, Registry };
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

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
    if let Ok((stable,)) = ic::stable_restore::<(StableStorage,)>() {
        let mut updated_nft_canisters = Vec::with_capacity(stable.db.len());

        for (_key, nft_canister) in stable.db.into_iter() {
            let updated_nft_canister = NftCanister {
                name: nft_canister.name.clone(),
                description: nft_canister.description.clone(),
                thumbnail: nft_canister.thumbnail.clone(),
                frontend: nft_canister.frontend.clone(),
                principal_id: nft_canister.principal_id.clone(),
                details: vec![
                    nft_canister.details[0].clone(),
                    (String::from("asset_type"), DetailValue::Text(String::from("ASSET TYPE HERE"))),
                ],
            };

            updated_nft_canisters.push((updated_nft_canister.principal_id, updated_nft_canister));
        }
        ic::get_mut::<Registry>().load(updated_nft_canisters);
        ic::store(Controller(stable.controller));
    }
}
