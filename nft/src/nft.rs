use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk_macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;

struct Controller(Principal);
impl Default for Controller {
    fn default() -> Self {
        panic!("Cannot set a default controller!")
    }
}

#[init]
fn init() {
    let ic = get_context();
    let controller = ic.caller();
    ic.store(Controller(controller));
}

fn is_controller(account: &Principal) -> bool {
    account == &get_context().get::<Controller>().0
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
    pub fn add(&mut self, name: String, canister_info: NftCanister) -> Result<(), OperationError> {
        self.0.insert(name, canister_info);
        Ok(())
    }

    pub fn remove(&mut self, name: &String) -> Result<(), OperationError> {
        if self.0.contains_key(name) {
            self.0.remove(name);
            return Ok(());
        }

        Err(OperationError::NonExistentCanister)
    }

    pub fn edit(
        &mut self,
        name: &String,
        principal_id: Option<Principal>,
        standard: Option<String>,
    ) -> Result<(), OperationError> {
        match self.0.get_mut(name) {
            None => return Err(OperationError::NonExistentCanister),
            Some(canister) => {
                if principal_id.is_some() {
                    canister.principal_id = principal_id.unwrap();
                } else {
                    canister.standard = standard.unwrap();
                }
                return Ok(());
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

#[update]
fn add(canister_info: NftCanister) -> Result<(), OperationError> {
    let ic = get_context();
    if !is_controller(&ic.caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let name = canister_info.name.clone();
    if name.len() <= 120 {
        let db = ic.get_mut::<Registry>();
        match db.add(name, canister_info) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    Err(OperationError::CharacterLimitation)
}

#[update]
fn remove(name: String) -> Result<(), OperationError> {
    let ic = get_context();
    if !is_controller(&ic.caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic.get_mut::<Registry>();
    db.remove(&name)
}

#[update]
fn edit(
    name: String,
    principal_id: Option<Principal>,
    standard: Option<String>,
) -> Result<(), OperationError> {
    let ic = get_context();
    if !is_controller(&ic.caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if principal_id.is_none() && standard.is_none() {
        return Err(OperationError::ParamatersNotPassed);
    } else {
        let db = ic.get_mut::<Registry>();
        return db.edit(&name, principal_id, standard);
    }
}

#[update]
fn get_canister(name: String) -> Option<&'static NftCanister> {
    let ic = get_context();
    let db = ic.get_mut::<Registry>();
    db.get_canister(&name)
}

#[update]
fn get_all() -> Vec<&'static NftCanister> {
    let ic = get_context();
    let db = ic.get_mut::<Registry>();
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
