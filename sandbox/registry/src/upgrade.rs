use crate::registry::{CanisterDB, CanisterMetadata};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::*;
use ic_kit::ic::*;
use ic_kit::macros::*;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, CanisterMetadata)>
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<CanisterDB>().archive();

    let stable = StableStorage { db };

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
        ic::get_mut::<CanisterDB>().load(stable.db);
    }
}
