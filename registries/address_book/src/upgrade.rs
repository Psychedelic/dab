use crate::address_book::{Address, AddressBook};

use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

type Key = (Principal, String);

#[derive(CandidType, Deserialize)]
struct StableStorage {
    address_book: Vec<(Key, Address)>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let address_book = ic::get_mut::<AddressBook>().archive();

    let stable = StableStorage { address_book };

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
        ic::get_mut::<AddressBook>().load(stable.address_book);
    }
}
