use crate::registry::{CanisterDB, CanisterMetadata};

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    canister_db: Vec<(String, CanisterMetadata)>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let canister_db = storage::get_mut::<CanisterDB>().archive();

    let stable = StableStorage {
        canister_db,
    };

    match storage::stable_save((stable,)) {
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
    if let Ok((stable,)) = storage::stable_restore::<(StableStorage,)>() {
        storage::get_mut::<CanisterDB>().load(stable.canister_db);
    }
}
