use crate::nft::{Admins, DetailValue, NftCanister, Registry};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, NftCanister)>,
    admins: Vec<Principal>,
}

// This struct should be removed in the next release.
#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(Principal, NftCanister)>,
    controller: Principal,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<Registry>().archive();
    let admins = ic::get_mut::<Admins>().0;

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
    // In the line below, StableStorageV0 should be replaced with StableStorage in the next release.
    if let Ok((stable,)) = ic::stable_restore::<(StableStorageV0,)>() {
        ic::get_mut::<Registry>().load(stable.db);

        // The line below should be replace with line 46 in the next release.
        ic::store(Admins(vec![stable.controller]));
        //ic::store(Admins(stable.admins));
    }
}
