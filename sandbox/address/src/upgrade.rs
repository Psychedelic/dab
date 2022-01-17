use crate::ab::AddressBook;

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;

type Key = (Principal, String);

#[derive(CandidType, Deserialize)]
struct StableStorage {
    address_book: Vec<(Key, Principal)>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let address_book = storage::get_mut::<AddressBook>().archive();

    let stable = StableStorage { address_book };

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
        storage::get_mut::<AddressBook>().load(stable.address_book);
    }
}
