use crate::common_types::{CanisterMetadata, DetailValue};
use crate::management::Admins;
use crate::registry::CanisterDB;

// IC imports
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
pub struct CanisterMetadataV0 {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(Principal, CanisterMetadataV0)>,
    admins: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, CanisterMetadata)>,
    admins: Vec<Principal>,
}

impl From<CanisterMetadataV0> for CanisterMetadata {
    fn from(wrapper: CanisterMetadataV0) -> CanisterMetadata {
        return CanisterMetadata {
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
    if let Ok((stable,)) = ic::stable_restore::<(StableStorageV0,)>() {
        let mut updated_canisters: Vec<(Principal, CanisterMetadata)> = vec![];
        for entry in stable.db {
            let updated_canister: CanisterMetadata = CanisterMetadata::from(entry.1);
            updated_canisters.push((entry.0, updated_canister));
        }
        ic::get_mut::<CanisterDB>().load(updated_canisters);
        ic::store(Admins(stable.admins));
    }
}
