use crc32fast;
use hex::FromHex;
use ic_kit::candid::Principal;
use ic_kit::ic::call;
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use unic::emoji::char::is_emoji;
use unic::emoji::*;

use crate::common_types::*;

pub struct AddressBook(BTreeMap<Key, Address>);

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
    }

    fn validate_account_id(&mut self, account_id: String) -> Result<(), Failure> {
        if account_id.clone().len() != ACCOUNT_ID_LENGTH {
            return Err(Failure::BadParameters);
        }

        let crc = u32::from_str_radix(&account_id.clone()[..8], 16).unwrap();
        let checksum: u32 =
            crc32fast::hash(&(<[u8; 28]>::from_hex(&account_id.clone()[8..]).unwrap()));

        if crc != checksum {
            return Err(Failure::BadParameters);
        }

        return Ok(());
    }

    async fn validate_icns(&mut self, icns: String) -> Result<(), Failure> {
        let result: (Option<GetRecordResponse>,) = call(
            Principal::from_text(ICNS_REGISTRY_PRINCIPAL_ID).unwrap(),
            "getRecord",
            (icns.clone(),),
        )
        .await
        .unwrap();

        if result.0.is_none() {
            return Err(Failure::BadParameters);
        }

        return Ok(());
    }

    pub async fn validate_address_type(&mut self, address: AddressType) -> Result<(), Failure> {
        match address {
            AddressType::Icns(s) => self.validate_icns(s).await,
            AddressType::AccountId(s) => self.validate_account_id(s),
            AddressType::PrincipalId(_s) => Ok(()),
        }
    }

    pub fn add(&mut self, account: Principal, address: Address) -> Result<(), Failure> {
        let pointer: Key = (account, address.name.clone());

        self.0.insert(pointer.clone(), address);
        return Ok(());
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

    pub fn get_all_paginated(
        &self,
        account: Principal,
        offset: usize,
        _limit: usize,
    ) -> Result<Vec<(&Key, &Address)>, Failure> {
        let mut limit = _limit;

        if offset >= limit {
            return Err(Failure::BadParameters);
        }

        let start: Key = (account.clone(), String::new());
        let end: Key = (account.clone(), unsafe {
            String::from(std::char::from_u32_unchecked(u32::MAX))
        });
        let addresses: Vec<(&(ic_kit::Principal, std::string::String), &Address)> =
            self.0.range((Included(start), Included(end))).collect();

        if offset + limit > addresses.len() {
            limit = addresses.len() - offset;
        }

        return Ok(addresses[offset..limit].to_vec());
    }
}

#[query]
fn name() -> String {
    String::from("Address Book")
}

#[update]
pub async fn add(address: Address) -> Result<(), Failure> {
    if &address.name.len() > &NAME_LIMIT {
        return Err(Failure::BadParameters);
    } else if address.description.is_some() {
        let description = address.clone().description.unwrap();

        if &description.len() > &DESCRIPTION_LIMIT {
            return Err(Failure::BadParameters);
        }
    } else if address.emoji.is_some() {
        let emojis: Vec<char> = address.clone().emoji.unwrap().chars().take(1).collect();

        if !is_emoji(emojis[0]) {
            return Err(Failure::BadParameters);
        }
    }

    let caller = ic::caller();

    let address_book = ic::get_mut::<AddressBook>();
    address_book
        .validate_address_type(address.value.clone())
        .await?;
    address_book.add(caller.clone(), address.clone());
    return Ok(());
}

#[update]
pub fn remove(address_name: String) -> Result<(), Failure> {
    let address_book = ic::get_mut::<AddressBook>();
    return address_book.remove(ic::caller(), address_name);
}

#[update]
pub fn get_all() -> Vec<&'static Address> {
    let address_book = ic::get_mut::<AddressBook>();
    address_book
        .get_all(ic::caller())
        .iter()
        .map(|entry| entry.1)
        .collect()
}

#[update]
pub fn get_all_paginated(
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<Vec<&'static Address>, Failure> {
    let address_book = ic::get_mut::<AddressBook>();
    let addresses = address_book
        .get_all_paginated(
            ic::caller(),
            offset.unwrap_or(0),
            limit.unwrap_or(DEFAULT_LIMIT),
        )?
        .iter()
        .map(|entry| entry.1)
        .collect();

    return Ok(addresses);
}
