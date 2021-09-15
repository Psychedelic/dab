use crate::nft::{Registry, NftCanister, Controller};
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::*;
use ic_kit::ic::*;
use ic_kit::macros::*;

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(String, NftCanister)>,
    controller: Principal
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<Registry>().archive();
    let controller = ic::get_mut::<Controller>().0;

    let stable = StableStorage {
        db,
        controller
    };

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
        ic::get_mut::<Registry>().load(stable.db);
        ic::store(Controller(stable.controller));
    }
}
