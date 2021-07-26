use std::collections::HashMap;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use ic_cdk::*;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

type Key = (Principal, String);
pub struct AddressBook(HashMap<Key, Principal>);

impl Default for AddressBook {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl AddressBook {
    pub fn add_address(&mut self, account: Principal, item: <Key, Principal>) {}
    pub fn remove_address(&mut self, account: Principal, canister_name: String) {}
    pub fn get_address(&mut self, account: Principal, canister_name: String) {}
    pub fn remove_all(&mut self, account: Principal) {};
    pub fn get_all(&mut self, account: Principal) {};
}

#[query]
fn name() -> String {
    String::from("Decentralised Address Book")
}

#[update]
fn add_address(canister_name: String, canister_id: Principal) {
    let pointer: Key = (caller(), canister_name);
    let item = (pointer, canister_id);

    let AddressBook = storage::get_mut::<AddressBook>();
    AddressBook.add_address(caller(), item);
}

#[update]
fn remove_address(canister_name: String) {
    let AddressBook = storage::get_mut::<AddressBook>();
    AddressBook.add_address(caller(), canister_name);
}

#[update]
fn get_address(canister_name: String) {
    let AddressBook = storage::get_mut::<AddressBook>();
    AddressBook.add_address(caller(), canister_name);
}

#[update]
fn remove_all() {
    let AddressBook = storage::get_mut::<AddressBook>();
    AddressBook.add_address(caller());
}

#[update]
fn get_all() {
    let AddressBook = storage::get_mut::<AddressBook>();
    AddressBook.add_address(caller());
}

/** TODO (@Nima-Ra):
        1. Write unfinished methods.
        2. Fix the candid file.
        3. Update README.md
**/
