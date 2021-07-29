use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

type Key = (Principal, String);
pub struct AddressBook(BTreeMap<Key, Principal>);

impl Default for AddressBook {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl AddressBook {
    pub fn add_address(
        &mut self,
        account: Principal,
        canister_name: String,
        canister_id: Principal,
    ) {
        let pointer: Key = (account, canister_name);
        self.0.insert(pointer, canister_id);
    }

    pub fn remove_address(&mut self, account: Principal, canister_name: String) {
        let pointer: Key = (account, canister_name);
        self.0.remove(&pointer);
    }

    pub fn get_address(&self, account: Principal, canister_name: String) -> GetAddressResult {
        let pointer: Key = (account, canister_name.clone());
        let canister_id: Option<Principal> = self.0.get(&pointer).cloned();
        GetAddressResult {
            canister_name,
            canister_id,
        }
    }

    pub fn remove_all(&mut self, account: Principal) {
        // Unfortunately there is not a better native rust method to do this atm, see:
        // https://users.rust-lang.org/t/removing-range-of-elements-from-btreemap/51582/14

        let start: Key = (account.clone(), String::new());
        let end: Key = (account.clone(), unsafe {
            String::from(std::char::from_u32_unchecked(u32::MAX))
        });

        // Remove the data in the search range, and insert the remaining elements again.
        let mut rem = self.0.split_off(&start).split_off(&end);
        self.0.append(&mut rem);

        // We don't know what users are doing, so the final key might still be in the map.
        // (in theory it can be part of the second split_off)
        self.0.remove(&end);
    }

    pub fn get_all(&self, account: Principal) -> Vec<(&Key, &Principal)> {
        let start: Key = (account.clone(), String::new());
        let end: Key = (account.clone(), unsafe {
            String::from(std::char::from_u32_unchecked(u32::MAX))
        });
        self.0.range((Included(start), Included(end))).collect()
    }
}

#[query]
fn name() -> String {
    String::from("DAB")
}

#[update]
fn add_address(canister_name: String, canister_id: Principal) {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.add_address(caller(), canister_name, canister_id);
}

#[update]
fn remove_address(canister_name: String) {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.remove_address(caller(), canister_name);
}

#[derive(Deserialize, CandidType, Clone)]
pub struct GetAddressResult {
    canister_name: String,
    canister_id: Option<Principal>,
}

#[update]
fn get_address(canister_name: String) -> GetAddressResult {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_address(caller(), canister_name)
}

#[update]
fn remove_all() {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.remove_all(caller());
}

#[update]
fn get_all() -> Vec<(&'static Key, &'static Principal)> {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_all(caller())
}
