use crate::common_types::{Event, TrustedSource};
use crate::history::*;
use crate::management::Admins;
use crate::trusted_sources::*;
use ic_kit::candid::{CandidType, Deserialize};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
pub struct StableStorage {
    trusted_sources: Vec<(Principal, TrustedSource)>,
    history: Vec<Event>,
    admins: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let trusted_sources = ic::get_mut::<TrustedSources>().archive();
    let history = ic::get_mut::<History>().archive();
    let admins = ic::get_mut::<Admins>().0.clone();

    let stable = StableStorage {
        trusted_sources,
        history,
        admins,
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
        ic::get_mut::<TrustedSources>().load(stable.trusted_sources);
        ic::get_mut::<History>().load(stable.history);
        ic::store(Admins(stable.admins));
    }
}
