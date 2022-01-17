use crate::profile::{ProfileDB, ProfileMetadata};

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    profile_db: Vec<(Principal, ProfileMetadata)>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let profile_db = ic::get_mut::<ProfileDB>().archive();

    let stable = StableStorage { profile_db };

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
        ic::get_mut::<ProfileDB>().load(stable.profile_db);
    }
}
