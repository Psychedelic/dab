use crc32fast;
use hex::FromHex;
use ic_kit::candid::{CandidType, Principal};
use ic_kit::ic::call;
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use unic::emoji::char::is_emoji;
use unic::emoji::*;

/**
Every item in the map looks like this:
( ( Principal,  String       ), Principal  )
( ( UserID,     CanisterName ), CanisterID )
**/

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum AddressType {
    PrincipalId(Principal),
    AccountId(String),
    Icns(String),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Address {
    pub name: String,
    pub value: AddressType,
    pub description: Option<String>,
    pub emoji: Option<String>,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct GetRecordResponse {
    pub ttl: u64,
    controller: Principal,
    resolver: Principal,
    owner: Principal,
    operator: Principal,
    name: String,
    expiry: i64,
}

type Key = (Principal, String);
pub struct AddressBook(BTreeMap<Key, Address>);

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 24;
const ACCOUNT_ID_LENGTH: usize = 64;
const ICNS_SUFFIX: &str = ".icp";
const ICNS_REGISTRY_PRINCIPAL_ID: &str = "e5kvl-zyaaa-aaaan-qabaq-cai";

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

    fn validate_account_id(&mut self, account_id: String) -> bool {
        if account_id.clone().len() != ACCOUNT_ID_LENGTH {
            return false;
        }

        let crc = u32::from_str_radix(&account_id.clone()[..8], 16).unwrap();
        let checksum: u32 =
            crc32fast::hash(&(<[u8; 28]>::from_hex(&account_id.clone()[8..]).unwrap()));

        if crc != checksum {
            return false;
        }

        return true;
    }

    async fn validate_icns(&mut self, icns: String) -> bool {
        let result: (Option<GetRecordResponse>,) = call(
            Principal::from_text(ICNS_REGISTRY_PRINCIPAL_ID).unwrap(),
            "getRecord",
            (&icns,),
        )
        .await
        .unwrap();
        ic_cdk::println!("{:?}", result);
        return result.0.is_some();
    }

    pub async fn validate_address_type(&mut self, address: AddressType) -> Result<(), Failure> {
        match address {
            AddressType::Icns(s) => match self.validate_icns(s).await {
                true => return Ok(()),
                false => return Err(Failure::BadParameters),
            },
            AddressType::AccountId(s) => match self.validate_account_id(s) {
                true => return Ok(()),
                false => return Err(Failure::BadParameters),
            },
            AddressType::PrincipalId(s) => Ok(()),
            _ => Err(Failure::BadParameters),
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
async fn add(address: Address) -> Result<(), Failure> {
    if &address.name.len() > &NAME_LIMIT {
        return Err(Failure::BadParameters);
    }

    if address.description.is_some() {
        let description = address.clone().description.unwrap();

        if &description.len() > &DESCRIPTION_LIMIT {
            return Err(Failure::BadParameters);
        }
    }

    if address.emoji.is_some() {
        let emojis: Vec<char> = address.clone().emoji.unwrap().chars().take(1).collect();

        if !is_emoji(emojis[0]) {
            return Err(Failure::BadParameters);
        }
    }

    let address_book = ic::get_mut::<AddressBook>();
    address_book
        .validate_address_type(address.value.clone())
        .await?;
    address_book.add(ic::caller(), address.clone());
    return Ok(());
}

#[update]
fn remove(address_name: String) -> Result<(), Failure> {
    let address_book = ic::get_mut::<AddressBook>();
    return address_book.remove(ic::caller(), address_name);
}

#[update]
fn get_all() -> Vec<&'static Address> {
    let address_book = ic::get_mut::<AddressBook>();
    address_book
        .get_all(ic::caller())
        .iter()
        .map(|entry| entry.1)
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add_address_successfully() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         let address_info = Address {
//             name: String::from("Bob"),
//             description: Some(String::from("Friend")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let addition_result = add(address_info.clone()).await;
//         assert!(addition_result.is_ok());

//         let addresses = get_all();
//         assert_eq!(addresses.len(), 1);
//         assert_eq!(
//             addresses[0].value,
//             address_info.value
//         );
//     }

//     #[test]
//     fn test_add_address_fails_because_of_long_description_param() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         let address_info = Address {
//             name: String::from("Bob"),
//             description: Some(
//                 std::iter::repeat("X")
//                     .take(DESCRIPTION_LIMIT + 1)
//                     .collect::<String>(),
//             ),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let addition_result = add(address_info.clone());
//         assert!(addition_result.is_err());
//         assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);
//     }

//     #[test]
//     fn test_add_address_fails_because_of_long_name_param() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         let address_info = Address {
//             name: std::iter::repeat("X").take(25).collect::<String>(),
//             description: Some(String::from("description")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let addition_result = add(address_info.clone());
//         assert!(addition_result.is_err());
//         assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);
//     }

//     #[test]
//     fn test_add_address_fails_because_of_bad_emoji_param() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         let address_info = Address {
//             name: String::from("Bob"),
//             description: Some(String::from("description")),
//             emoji: Some(String::from("a")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let addition_result = add(address_info.clone());
//         assert!(addition_result.is_err());
//         assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);
//     }

//     #[test]
//     fn test_remove_address_successfully() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         let address_info = Address {
//             name: String::from("Bob"),
//             description: Some(String::from("Friend")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let addition_result = add(address_info.clone());
//         assert!(addition_result.is_ok());

//         let removal_result = remove(String::from("Bob"));
//         assert!(removal_result.is_ok());

//         let get_all_result = get_all();
//         assert_eq!(get_all_result.len(), 0);
//     }

//     #[test]
//     fn test_users_get_their_own_addresses() {
//         let context = MockContext::new().inject();

//         let bob_address_info = Address {
//             name: String::from("Bob"),
//             description: Some(String::from("Friend")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let alice_address_info = Address {
//             name: String::from("Alice"),
//             description: Some(String::from("Friend")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::alice().to_text()))
//         };

//         //Alice adds Bob as her contact
//         context.update_caller(mock_principals::alice());
//         add(bob_address_info);

//         let alice_addresses = get_all();

//         assert_eq!(alice_addresses.len(), 1);
//         assert_eq!(alice_addresses[0].name, String::from("Bob"));

//         //Bob adds Alison as his contact
//         context.update_caller(mock_principals::bob());
//         add(alice_address_info);

//         let bob_addresses = get_all();

//         assert_eq!(bob_addresses.len(), 1);
//         assert_eq!(bob_addresses[0].name, String::from("Alice"));
//     }

//     #[test]
//     fn test_addresses_are_added_alphabetically_successfully() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         let bob_address_info = Address {
//             name: String::from("Bob"),
//             description: Some(String::from("Friend")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::bob().to_text())),
//         };

//         let andrew_address_info = Address {
//             name: String::from("Andrew"),
//             description: Some(String::from("Friend")),
//             emoji: Some(String::from("😚")),
//             value: AddressType::Principal(String::from(mock_principals::alice().to_text()))
//         };

//         add(bob_address_info);
//         add(andrew_address_info);

//         let addresses = get_all();

//         assert_eq!(addresses.len(), 2);
//         assert_eq!(addresses[0].name, String::from("Andrew"));
//         assert_eq!(
//             addresses[0].value,
//             AddressType::Principal(mock_principals::alice().to_text())
//         );
//         assert_eq!(addresses[1].name, String::from("Bob"));
//         assert_eq!(
//             addresses[1].value,
//             AddressType::Principal(String::from(mock_principals::bob().to_text()))
//         );
//     }
// }
