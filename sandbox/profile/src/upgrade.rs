use crate::profile::{ProfileDB, ProfileMetadata};

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    profile_db: Vec<(Principal, ProfileMetadata)>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let profile_db = storage::get_mut::<ProfileDB>().archive();

    let stable = StableStorage {
        profile_db,
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
        storage::get_mut::<ProfileDB>().load(stable.profile_db);
    }
}
