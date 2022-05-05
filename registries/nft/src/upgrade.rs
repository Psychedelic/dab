use crate::common_types::{NftCanister, DetailValue};
use crate::management::Admins;
use crate::nft::Registry;

use ic_kit::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
pub struct NftCanisterV0 {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Deserialize)]
pub struct StableStorageV0 {
    db: Vec<(Principal, NftCanisterV0)>,
    admins: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, NftCanister)>,
    admins: Vec<Principal>,
}

impl From<NftCanisterV0> for NftCanister {
    fn from(wrapper: NftCanisterV0) -> NftCanister {
        return NftCanister {
            name: wrapper.name,
            description: wrapper.description,
            thumbnail: wrapper.thumbnail,
            frontend: wrapper.frontend,
            principal_id: wrapper.principal_id,
            details: wrapper.details,
            submitter: ic::id(),
            last_updated_by: ic::id(),
            last_updated_at: ic::time(),
        };
    }
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<Registry>().archive();
    let admins = &ic::get_mut::<Admins>().0;

    let stable = StableStorage {
        db,
        admins: admins.to_vec(),
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
    if let Ok((stable,)) = ic::stable_restore::<(StableStorageV0,)>() {
        let mut updated_nft_canisters: Vec<(Principal, NftCanister)> = vec![];
        for entry in stable.db {
            let updated_nft_canister: NftCanister = NftCanister::from(entry.1);
            updated_nft_canisters.push((entry.0, updated_nft_canister));
        }
        ic::get_mut::<Registry>().load(updated_nft_canisters);
        ic::store(Admins(stable.admins));
    }
}
