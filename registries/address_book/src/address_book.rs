use ic_cdk::export::candid::{CandidType, Principal, Deserialize};
use ic_cdk::*;
use ic_cdk_macros::*;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

#[derive(CandidType, Deserialize, Clone)]
pub struct Address {
    pub name: String,
    pub principal_id: Principal,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
}

type Key = (Principal, String);
pub struct AddressBook(BTreeMap<Key, Address>);

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 24;

impl Default for AddressBook {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl AddressBook {
    pub fn archive(&mut self) -> Vec<(Key, Address)> {
        let map = std::mem::replace(&mut self.0, BTreeMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Key, Address)>) {
        self.0 = archive.into_iter().collect();
        // self.0.reserve(25_000 - self.0.len());
    }

    pub fn add(
        &mut self,
        account: Principal,
        address: Address,
    ) {
        let pointer: Key = (account, address.name.clone());
        self.0.insert(pointer.clone(), address);
    }

    pub fn remove(&mut self, account: Principal, canister_name: String) -> Result<(), Failure> {
        let pointer: Key = (account, canister_name);

        if !self.0.contains_key(&pointer) {
            return Err(Failure::NonExistentItem);
        }

        self.0.remove(&pointer);
        return Ok(());
    }

    pub fn get_all(&self, account: Principal) -> Vec<(&Key, &Address)> {
        let start: Key = (account.clone(), String::new());
        let end: Key = (account.clone(), unsafe {
            String::from(std::char::from_u32_unchecked(u32::MAX))
        });
        self.0.range((Included(start), Included(end))).collect()
    }
}

#[derive(CandidType, Debug, PartialEq)]
pub enum Failure {
    NotAuthorized,
    BadParameters,
    NonExistentItem,
    Unknown(String),
}

#[query]
fn name() -> String {
    String::from("Address Book")
}

#[update]
fn add(address: Address) -> Result<(), Failure> {
    if &address.name.len() > &NAME_LIMIT {
        return Err(Failure::BadParameters);
    }
    
    let address_book = storage::get_mut::<AddressBook>();
    address_book.add(caller(), address);
    return Ok(());
}

#[update]
fn remove(address_name: String) -> Result<(), Failure> {
    let address_book = storage::get_mut::<AddressBook>();
    return address_book.remove(caller(), address_name);
}

#[update]
fn get_all() -> Vec<&'static Address> {
    let address_book = storage::get_mut::<AddressBook>();
    address_book.get_all(caller()).iter().map(|entry| entry.1).collect()
}

