use crate::common_types::TrustedSource;
use crate::management::Admins;
use crate::proxy::*;
use ic_kit::candid::{CandidType, Deserialize};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
pub struct StableStorage {
    db: Vec<(Principal, TrustedSource)>,
    admins: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<TrustedSources>().archive();
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
        ic::get_mut::<TrustedSources>().load(stable.db);
        ic::store(Admins(stable.admins));
    }
}
