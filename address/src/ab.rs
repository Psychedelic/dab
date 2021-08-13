use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

#[derive(CandidType)]
pub struct CanisterAddress {
    canister_name: String,
    canister_id: Principal,
}

type Key = (Principal, String);
pub struct AddressBook(BTreeMap<Key, Principal>);

impl Default for AddressBook {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl AddressBook {
    pub fn archive(&mut self) -> Vec<(Key, Principal)> {
        let map = std::mem::replace(&mut self.0, BTreeMap::new());
        map.into_iter()
            .collect()
    }
    
    pub fn load(&mut self, archive: Vec<(Key, Principal)>) {
        self.0 = archive.into_iter().collect();
        // self.0.reserve(25_000 - self.0.len());
    }

    pub fn add_address(
        &mut self,
        account: Principal,
        canister_name: String,
        canister_id: Principal,
    ) -> bool {
        let pointer: Key = (account, canister_name);
        self.0.insert(pointer.clone(), canister_id);
        self.0.contains_key(&pointer)
    }

    pub fn remove_address(&mut self, account: Principal, canister_name: String) -> bool {
        let pointer: Key = (account, canister_name);
        self.0.remove(&pointer);
        !self.0.contains_key(&pointer)
    }

    pub fn get_address(
        &self,
        account: Principal,
        canister_name: String,
    ) -> Option<CanisterAddress> {
        let pointer: Key = (account, canister_name.clone());
        if self.0.contains_key(&pointer) {
            let canister_id: Principal = self.0.get(&pointer).unwrap().clone();
            // return Some((canister_name, canister_id));
            return Some(CanisterAddress {
                canister_name: canister_name,
                canister_id: canister_id
            });
        }
        return None;
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
    String::from("Address Book")
}

#[update]
fn add_address(canister_name: String, canister_id: Principal) -> bool {
    if canister_name.len() < 120 {
        let address_book = storage::get_mut::<AddressBook>();
        return address_book.add_address(caller(), canister_name, canister_id);
    }
    false
}

#[update]
fn remove_address(canister_name: String) -> bool {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.remove_address(caller(), canister_name)
}

#[update]
fn get_address(canister_name: String) -> Option<CanisterAddress> {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_address(caller(), canister_name)
}

#[derive(CandidType)]
pub struct GetAllResult {
    total_addresses: u64,
    list: Vec<(&'static Key, &'static Principal)>,
}

#[update]
fn get_all() -> GetAllResult {
    let address_book = storage::get_mut::<AddressBook>();
    let canisters_list = address_book.get_all(caller());
    GetAllResult {
        total_addresses: canisters_list.len() as u64,
        list: canisters_list,
    }
}
