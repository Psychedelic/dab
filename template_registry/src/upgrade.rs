// Project imports
use crate::common_types::CanisterMetadata;
use crate::registry::CanisterDB;
use crate::management::Admins;

// IC imports
use ic_kit::*;
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, CanisterMetadata)>,
    admins: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<CanisterDB>().archive();
    let admins = ic::get_mut::<Admins>().0.clone();

    let stable = StableStorage { db, admins };

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
        ic::store(Admins(stable.admins));
    }
}
