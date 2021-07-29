

use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::BTreeMap;
use serde::{Deserialize};
use ic_cdk_macros::*;
use ic_cdk::*;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

type Key = (Principal, String);
pub struct AddressBook(BTreeMap<Key, Principal>);

fn binary_search(map: Vec<(Principal, String)>, target: Principal, low: usize, high: usize) -> (Key, Key) {
    //ic_cdk::api::print(map.to_string());
    let highest_principal = map[high].0;
    let lowest_principal = map[low].0;
    let middle = low + high / 2;
    let middle_principal = map[middle].0;

    if highest_principal == target {
        if lowest_principal == target {
            return (map[low], map[high]);
        }
        return binary_search(map, target, low + 1, high);
    } else if lowest_principal == target {
        return binary_search(map, target, low, high - 1);
    } else {
        if middle_principal  == target {
            loop {
                if map[middle - 1].0 != target && map[middle + 1].0 != target {
                    return (map[middle], map[middle]);
                } else if map[middle - 1].0 != target {
                    return binary_search(map, target, middle, high - 1);
                }
            }
        } else if middle_principal > target {
            return binary_search(map, target, low + 1, middle - 1);
        } else {
            return binary_search(map, target, middle + 1, high - 1);
        }
    }
}

impl Default for AddressBook {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl AddressBook {
    pub fn add_address(&mut self, account: Principal, canister_name: String, canister_id: Principal) {
        let pointer: Key = (account, canister_name);
        self.0.insert(
            pointer,
            canister_id
        );
    }

    pub fn remove_address(&mut self, account: Principal, canister_name: String) {
        let pointer: Key = (account, canister_name);
        self.0.remove(&pointer);
    }
    
    pub fn get_address(&mut self, account: Principal, canister_name: String) -> GetAddressResult {
        let pointer: Key = (account, canister_name.clone());
        let canister_id: Option<Principal> = self.0.get(&pointer).cloned();
        GetAddressResult { canister_name: canister_name, canister_id: canister_id }
    }

    pub fn remove_all(&mut self, account: Principal) {
        // binary_search
    }

    pub fn get_all(&mut self, account: Principal) -> Vec<(Key, Principal)> {
        let keys: Vec<_> = self.0.keys().cloned().collect();
        
        // "2" should be changed to the length of the keys vector.
        let range = binary_search(keys, account, 0, 2);

        // ic_cdk::api::print(range.0.to_string());
        // ic_cdk::api::print(range.1.to_string());

        self.0.range(range.0..range.1).cloned().collect()
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
fn get_all() -> Vec<(Key, Principal)> {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_all(caller())
}
