use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::validate_url;

pub struct Admins(pub Vec<Principal>);

impl Default for Admins {
    fn default() -> Self {
        panic!()
    }
}

#[init]
fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

fn is_admin(account: &Principal) -> bool {
    account == &ic::get::<Admins>().0
}

#[update]
fn add_admin(new_admin: Principal) -> Result<(), Failure> {
    if is_admin(&ic::caller()) {
        ic::get_mut::<Admins>().0.push(new_admin);
        return Ok(());
    }
    Err(Failure::NotAuthorized)
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum DetailValue {
    True,
    False,
    U64(u64),
    I64(i64),
    Float(f64),
    Text(String),
    Principal(Principal),
    #[serde(with = "serde_bytes")]
    Slice(Vec<u8>),
    Vec(Vec<DetailValue>),
}

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 120;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct NftCanister {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(Default)]
pub struct Registry(HashMap<Principal, NftCanister>);

impl Registry {
    pub fn archive(&mut self) -> Vec<(Principal, NftCanister)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, NftCanister)>) {
        assert!(self.0.is_empty());
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, canister_info: NftCanister) -> Result<(), OperationError> {
        self.0.insert(canister_info.principal_id, canister_info);
        Ok(())
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if self.0.remove(&principal_id).is_some() {
            return Ok(());
        }

        Err(OperationError::NonExistentItem)
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

#[derive(CandidType, Debug, PartialEq)]
pub enum OperationError {
    NotAuthorized,
    NonExistentItem,
    BadParameters,
    Unknown(String),
}

#[update]
fn add(canister_info: NftCanister) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    } else if !validate_url(&canister_info.thumbnail) {
        return Err(OperationError::BadParameters);
    } else if canister_info.frontend.is_some()
        && !validate_url(&canister_info.frontend.clone().unwrap())
    {
        return Err(OperationError::BadParameters);
    } else if canister_info.details[0].0 != String::from("standard") {
        return Err(OperationError::BadParameters);
    } else if canister_info.details.len() != 1 {
        return Err(OperationError::BadParameters);
    }

    let name = canister_info.name.clone();
    if name.len() <= NAME_LIMIT && &canister_info.description.len() <= &DESCRIPTION_LIMIT {
        let db = ic::get_mut::<Registry>();
        return db.add(canister_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registry>();
    db.remove(&principal_id)
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

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
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

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        assert!(add(canister_info).is_ok());
    }

    #[test]
    fn test_add_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        context.update_caller(mock_principals::bob());

        let add_result = add(canister_info);

        assert!(add_result.is_err());
        assert_eq!(add_result.unwrap_err(), OperationError::NotAuthorized);
    }

    #[test]
    fn test_add_fails_because_of_bad_name_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: std::iter::repeat("X")
                .take(NAME_LIMIT + 1)
                .collect::<String>(),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: Some(String::from("https://frontend_url.com")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info);

        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[test]
    fn test_add_fails_because_of_bad_descripion_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: std::iter::repeat("X")
                .take(DESCRIPTION_LIMIT + 1)
                .collect::<String>(),
            thumbnail: String::from("https://logo_url.com"),
            frontend: Some(String::from("https://frontend_url.com")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info);

        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[test]
    fn test_add_fails_because_of_bad_thumbnail_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("bad thumbnail"),
            frontend: Some(String::from("https://frontend_url.com")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info);

        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[test]
    fn test_add_fails_because_of_bad_frontend_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: Some(String::from("bad frontend")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info);

        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[test]
    fn test_add_fails_because_of_bad_standard_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: None,
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standards"),
                DetailValue::Text(String::from("bad standard")),
            )],
        };

        let addition_result = add(canister_info);

        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[test]
    fn test_add_fails_because_of_invalid_details_params_amount() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: None,
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP721")),
                ),
                (
                    String::from("extra field"),
                    DetailValue::Text(String::from("invalid field")),
                ),
            ],
        };

        let addition_result = add(canister_info);

        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
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
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        assert!(add(canister_info).is_ok());

        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[test]
    fn test_remove_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: None,
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        add(canister_info.clone());

        context.update_caller(mock_principals::bob());

        let remove_result = remove(canister_info.clone().principal_id);

        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), OperationError::NotAuthorized);
    }

    #[test]
    fn test_remove_fails_because_of_non_existent_canister() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let remove_result = remove(mock_principals::xtc());

        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), OperationError::NonExistentItem);
    }

    #[test]
    fn test_get() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let canister_info = NftCanister {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        assert!(add(canister_info.clone()).is_ok());

        assert_eq!(
            get(mock_principals::xtc()).unwrap().name,
            canister_info.name
        );
        assert!(get(mock_principals::alice()).is_none());
    }

    #[test]
    fn test_get_returns_none_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let get_result = get(mock_principals::xtc());
        assert!(get_result.is_none());
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
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        assert!(add(canister_info.clone()).is_ok());
        assert_eq!(get_all()[0].name, canister_info.name);
    }

    #[test]
    fn test_get_all_returns_no_canisters_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Controller(mock_principals::alice()))
            .inject();

        let get_all_result = get_all();

        assert_eq!(get_all_result.len(), 0);
    }
}
