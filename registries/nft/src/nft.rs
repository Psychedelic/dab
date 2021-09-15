use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::*;
use ic_kit::ic::*;
use ic_kit::macros::*;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Controller(Principal);

impl Default for Controller {
    fn default() -> Self { panic!() }
}

impl Controller {
    pub fn archive(&mut self) -> Principal {
        self.0
    }
    
    pub fn load(&mut self, id: Principal) {
        self.0 = id;
    }
}

#[init]
fn init() {
    ic::store(Controller(ic::caller()));
}

fn is_controller(account: &Principal) -> bool {
    account == &ic::get::<Controller>().0
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct NftCanister {
    principal_id: Principal,
    name: String,
    standard: String,
}

#[derive(Default)]
pub struct Registry(HashMap<String, NftCanister>);

impl Registry {
    pub fn archive(&mut self) -> Vec<(String, NftCanister)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter()
            .collect()
    }
    
    pub fn load(&mut self, archive: Vec<(String, NftCanister)>) {
        self.0 = archive.into_iter().collect();
        // self.0.reserve(25_000 - self.0.len());
    }

    pub fn add(&mut self, name: String, canister_info: NftCanister) -> Result<OperationSuccessful, OperationError> {
        self.0.insert(name, canister_info);
        Ok(true)
    }

    pub fn remove(&mut self, name: &String) -> Result<OperationSuccessful, OperationError> {
        if self.0.contains_key(name) {
            self.0.remove(name);
            return Ok(true);
        }

        Err(OperationError::NonExistentCanister)
    }

    pub fn edit(
        &mut self,
        name: &String,
        principal_id: Option<Principal>,
        standard: Option<String>,
    ) -> Result<OperationSuccessful, OperationError> {
        match self.0.get_mut(name) {
            None => return Err(OperationError::NonExistentCanister),
            Some(canister) => {
                if principal_id.is_some() {
                    canister.principal_id = principal_id.unwrap();
                } else {
                    canister.standard = standard.unwrap();
                }
                return Ok(true);
            }
        }
    }

    pub fn get_canister(&self, name: &String) -> Option<&NftCanister> {
        self.0.get(name)
    }

    pub fn get_all(&self) -> Vec<&NftCanister> {
        self.0.values().collect()
    }
}

#[query]
fn name() -> String {
    String::from("NFT Registry Canister")
}

#[derive(CandidType)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentCanister,
    CharacterLimitation,
}

pub type OperationSuccessful = bool;

#[update]
fn add(canister_info: NftCanister) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let name = canister_info.name.clone();
    if name.len() <= 120 {
        let db = ic::get_mut::<Registry>();
        return db.add(name, canister_info);
    }

    Err(OperationError::CharacterLimitation)
}

#[update]
fn remove(name: String) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registry>();
    db.remove(&name)
}

#[update]
fn edit(
    name: String,
    principal_id: Option<Principal>,
    standard: Option<String>,
) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if principal_id.is_none() && standard.is_none() {
        return Err(OperationError::ParamatersNotPassed);
    } else {
        let db = ic::get_mut::<Registry>();
        return db.edit(&name, principal_id, standard);
    }
}

#[update]
fn get_canister(name: String) -> Option<&'static NftCanister> {
    let db = ic::get_mut::<Registry>();
    db.get_canister(&name)
}

#[update]
fn get_all() -> Vec<&'static NftCanister> {
    let db = ic::get_mut::<Registry>();
    db.get_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller() {
        // alice is the controller
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        let mut addition = add(canister_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove(String::from("xtc"));
        assert!(remove_operation.is_ok());
        
        ctx.update_caller(mock_principals::bob());
        addition = add(canister_info);
        assert!(addition.is_err());
    }

    #[test]
    fn test_add() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert!(add(canister_info).is_ok());
    }

    #[test]
    fn test_remove() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert!(add(canister_info).is_ok());

        assert!(remove(String::from("xtc")).is_ok());
    }

    #[test]
    fn test_get_canister() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert!(add(canister_info.clone()).is_ok());

        assert_eq!(get_canister(String::from("xtc")).unwrap(), &canister_info);
        assert!(get_canister(String::from("dab")).is_none());
    }

    #[test]
    fn test_get_all() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
        };

        assert!(add(canister_info.clone()).is_ok());
        assert_eq!(get_all(), vec![&canister_info]);
    }
}