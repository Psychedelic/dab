use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

pub struct Controller(pub Principal);

impl Default for Controller {
    fn default() -> Self {
        panic!()
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
fn set_controller(new_controller: Principal) -> Result<OperationSuccessful, OperationError> {
    if is_controller(&ic::caller()) {
        ic::store(Controller(new_controller));
        return Ok(true);
    }
    Err(OperationError::NotAuthorized)
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct NftCanister {
    pub principal_id: Principal,
    pub name: String,
    pub standard: String,
    pub description: String,
    pub icon: String,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputNftCanister {
    principal_id: Principal,
    name: String,
    standard: String,
    description: String,
    icon: String,
}

#[derive(Default)]
pub struct Registry(HashMap<Principal, NftCanister>);

impl Registry {
    pub fn archive(&mut self) -> Vec<(Principal, NftCanister)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }
    pub fn load(&mut self, archive: Vec<(Principal, NftCanister)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(
        &mut self,
        principal_id: Principal,
        canister_info: InputNftCanister,
    ) -> Result<OperationSuccessful, OperationError> {
        let nft_canister = NftCanister {
            principal_id: principal_id,
            name: canister_info.name,
            standard: canister_info.standard,
            description: canister_info.description,
            icon: canister_info.icon,
            timestamp: ic::time(),
        };

        self.0.insert(principal_id, nft_canister);
        Ok(true)
    }

    pub fn remove(
        &mut self,
        principal_id: &Principal,
    ) -> Result<OperationSuccessful, OperationError> {
        if self.0.contains_key(principal_id) {
            self.0.remove(principal_id);
            return Ok(true);
        }

        Err(OperationError::NonExistentCanister)
    }

    pub fn edit(
        &mut self,
        principal_id: Principal,
        name: Option<String>,
        standard: Option<String>,
        icon: Option<String>,
        description: Option<String>,
    ) -> Result<OperationSuccessful, OperationError> {
        match self.0.get_mut(&principal_id) {
            None => return Err(OperationError::NonExistentCanister),
            Some(canister) => {
                if name.is_some() {
                    canister.name = name.unwrap();
                }

                if standard.is_some() {
                    canister.standard = standard.unwrap();
                }

                if icon.is_some() {
                    canister.icon = icon.unwrap();
                }

                if description.is_some() {
                    canister.description = description.unwrap();
                }

                return Ok(true);
            }
        }
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&NftCanister> {
        self.0.get(principal_id)
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
    BadParameters,
}

pub type OperationSuccessful = bool;

#[update]
fn add(canister_info: InputNftCanister) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    } else if !validate_url(&canister_info.icon) {
        return Err(OperationError::BadParameters);
    }

    let name = canister_info.name.clone();
    if name.len() <= 120 && &canister_info.description.len() <= &1200 {
        let db = ic::get_mut::<Registry>();
        return db.add(canister_info.principal_id, canister_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
fn remove(principal_id: Principal) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registry>();
    db.remove(&principal_id)
}

#[update]
fn edit(
    principal_id: Principal,
    name: Option<String>,
    standard: Option<String>,
    icon: Option<String>,
    description: Option<String>,
) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    } else if name.is_none() && standard.is_none() && icon.is_none() && description.is_none() {
        return Err(OperationError::ParamatersNotPassed);
    } else if !validate_url(&icon.clone().unwrap()) {
        return Err(OperationError::BadParameters);
    } else if &description.clone().unwrap().len() > &1200 {
        return Err(OperationError::BadParameters);
    } else {
        let db = ic::get_mut::<Registry>();
        return db.edit(principal_id, name, standard, icon, description);
    }
}

#[query]
fn get(principal_id: Principal) -> Option<&'static NftCanister> {
    let db = ic::get_mut::<Registry>();
    db.get(&principal_id)
}

#[query]
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

        let canister_info = InputNftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
            description: String::from("XTC is your cycles wallet."),
            icon: String::from("https://google.com"),
        };

        let mut addition = add(canister_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove(mock_principals::xtc());
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

        let canister_info = InputNftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
            description: String::from("XTC is your cycles wallet."),
            icon: String::from("https://google.com"),
        };

        assert!(add(canister_info).is_ok());
    }

    #[test]
    fn test_remove() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = InputNftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
            description: String::from("XTC is your cycles wallet."),
            icon: String::from("https://google.com"),
        };

        assert!(add(canister_info).is_ok());

        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[test]
    fn test_get() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = InputNftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
            description: String::from("XTC is your cycles wallet."),
            icon: String::from("https://google.com"),
        };

        assert!(add(canister_info.clone()).is_ok());

        assert_eq!(
            get(mock_principals::xtc()).unwrap().name,
            canister_info.name
        );
        assert!(get(mock_principals::alice()).is_none());
    }

    #[test]
    fn test_get_all() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = InputNftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            standard: String::from("Dank"),
            description: String::from("XTC is your cycles wallet."),
            icon: String::from("https://google.com"),
        };

        assert!(add(canister_info.clone()).is_ok());
        assert_eq!(get_all()[0].name, canister_info.name);
    }
}
