use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Controller(pub Principal);

impl Default for Controller {
    fn default() -> Self {
        panic!()
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Token {
    principal_id: Principal,
    name: String,
    symbol: String,
    description: String,
    standard: String,
    total_supply: Option<u64>,
    logo: String,
    website: String,
    timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputAddToken {
    principal_id: Principal,
    name: String,
    symbol: String,
    description: String,
    standard: String,
    total_supply: Option<u64>,
    logo: String,
    website: String,
}

#[derive(Default)]
pub struct TokenRegistry(HashMap<String, Token>);

impl TokenRegistry {
    pub fn archive(&mut self) -> Vec<(String, Token)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }
    pub fn load(&mut self, archive: Vec<(String, Token)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(
        &mut self,
        name: String,
        token_info: InputAddToken,
    ) -> Result<OperationSuccessful, OperationError> {
        let token = Token {
            principal_id: token_info.principal_id,
            name: token_info.name,
            symbol: token_info.symbol,
            description: token_info.description,
            standard: token_info.standard,
            total_supply: token_info.total_supply,
            logo: token_info.logo,
            website: token_info.website,
            timestamp: ic::time(),
        };

        self.0.insert(name, token);
        Ok(true)
    }

    pub fn get_all(&self) -> Vec<&Token> {
        self.0.values().collect()
    }
}

#[init]
fn init() {
    ic::store(Controller(ic::caller()));
}

fn is_controller(account: &Principal) -> bool {
    account == &ic::get::<Controller>().0
}

#[update]
fn set_controller(new_controller: Principal) -> Result<OperationSuccessful, OperationError>{
    if is_controller(&ic::caller()) {
        ic::store(Controller(new_controller));   
        return Ok(true);
    }
    Err(OperationError::NotAuthorized)
}

#[query]
fn name() -> String {
    String::from("Token Registry Canister")
}

#[derive(CandidType)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentCanister,
    BadParameters,
}

pub type OperationSuccessful = bool;

#[update]
fn add(token_info: InputAddToken) -> Result<OperationSuccessful, OperationError> {
    // if !is_controller(&ic::caller()) {
    //     return Err(OperationError::NotAuthorized);
    // } else if !validate_url(&canister_info.icon) {
    //     return Err(OperationError::BadParameters);
    // }

    let name = token_info.name.clone();
    if name.len() <= 120 && &token_info.description.len() <= &1200 {
        let db = ic::get_mut::<TokenRegistry>();
        return db.add(name, token_info);
    }

    Err(OperationError::BadParameters)
}

#[query]
fn get_all() -> Vec<&'static Token> {
    let db = ic::get_mut::<TokenRegistry>();
    db.get_all()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_controller() {
//         // alice is the controller
//         let ctx = MockContext::new()
//             .with_caller(mock_principals::alice())
//             .inject();

//         init();

//         let canister_info = InputNftCanister {
//             name: String::from("xtc"),
//             principal_id: mock_principals::xtc(),
//             standard: String::from("Dank"),
//             description: String::from("XTC is your cycles wallet."),
//             icon: String::from("https://google.com"),
//         };

//         let mut addition = add(canister_info.clone());
//         assert!(addition.is_ok());

//         let remove_operation = remove(String::from("xtc"));
//         assert!(remove_operation.is_ok());

//         ctx.update_caller(mock_principals::bob());
//         addition = add(canister_info);
//         assert!(addition.is_err());
//     }

//     #[test]
//     fn test_add() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(Controller(mock_principals::alice()))
//             .inject();

//         let canister_info = InputNftCanister {
//             name: String::from("xtc"),
//             principal_id: mock_principals::xtc(),
//             standard: String::from("Dank"),
//             description: String::from("XTC is your cycles wallet."),
//             icon: String::from("https://google.com"),
//         };

//         assert!(add(canister_info).is_ok());
//     }

//     #[test]
//     fn test_remove() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(Controller(mock_principals::alice()))
//             .inject();

//         let canister_info = InputNftCanister {
//             name: String::from("xtc"),
//             principal_id: mock_principals::xtc(),
//             standard: String::from("Dank"),
//             description: String::from("XTC is your cycles wallet."),
//             icon: String::from("https://google.com"),
//         };

//         assert!(add(canister_info).is_ok());

//         assert!(remove(String::from("xtc")).is_ok());
//     }

//     #[test]
//     fn test_get_canister() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(Controller(mock_principals::alice()))
//             .inject();

//         let canister_info = InputNftCanister {
//             name: String::from("xtc"),
//             principal_id: mock_principals::xtc(),
//             standard: String::from("Dank"),
//             description: String::from("XTC is your cycles wallet."),
//             icon: String::from("https://google.com"),
//         };

//         assert!(add(canister_info.clone()).is_ok());

//         assert_eq!(
//             get_canister(String::from("xtc")).unwrap().name,
//             canister_info.name
//         );
//         assert!(get_canister(String::from("dab")).is_none());
//     }

//     #[test]
//     fn test_get_all() {
//         MockContext::new()
//             .with_caller(mock_principals::alice())
//             .with_data(Controller(mock_principals::alice()))
//             .inject();

//         let canister_info = InputNftCanister {
//             name: String::from("xtc"),
//             principal_id: mock_principals::xtc(),
//             standard: String::from("Dank"),
//             description: String::from("XTC is your cycles wallet."),
//             icon: String::from("https://google.com"),
//         };

//         assert!(add(canister_info.clone()).is_ok());
//         assert_eq!(get_all()[0].name, canister_info.name);
//     }
// }
