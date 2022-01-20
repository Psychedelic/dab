use crate::nft::{Controller, NftCanister, Registry};
use ic_kit::ic::*;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct NftCanisterV0 {
    pub principal_id: Principal,
    pub name: String,
    pub description: String,
    pub standard: String,
    pub icon: String,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(String, NftCanisterV0)>,
    controller: Principal,
}

impl From<NftCanisterV0> for NftCanister {
    fn from(nft_canister: NftCanisterV0) -> Self {
        NftCanister {
            principal_id: nft_canister.principal_id, 
            name: nft_canister.name,
            description: nft_canister.description,
            frontend: None,
            thumbnail: nft_canister.icon,
            details: vec![
                (String::from("standard"), nft_canister.standard),
            ],
        }
    }
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
            let nft_canister_metadata: NftCanister = nft_canister.1.into();

            updated_nft_canisters.push((nft_canister_metadata.principal_id, nft_canister_metadata));
        }

        ic::get_mut::<Registry>().load(updated_nft_canisters);
        ic::store(Controller(stable.controller));
    }
}
