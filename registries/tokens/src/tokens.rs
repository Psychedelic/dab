use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::ic::*;
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
    verified: bool,
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

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputEditToken {
    principal_id: Principal,
    name: Option<String>,
    symbol: Option<String>,
    description: Option<String>,
    standard: Option<String>,
    total_supply: Option<u64>,
    logo: Option<String>,
    website: Option<String>,
    verified: Option<bool>,
}

#[derive(Default)]
pub struct TokenRegistry(HashMap<Principal, Token>);

impl TokenRegistry {
    pub fn archive(&mut self) -> Vec<(Principal, Token)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }
    pub fn load(&mut self, archive: Vec<(Principal, Token)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(
        &mut self,
        token_info: InputAddToken,
    ) -> Result<OperationSuccessful, OperationError> {
        let token = Token {
            principal_id: token_info.principal_id,
            name: token_info.name.clone(),
            symbol: token_info.symbol,
            description: token_info.description,
            standard: token_info.standard,
            total_supply: token_info.total_supply,
            logo: token_info.logo,
            website: token_info.website,
            verified: false,
            timestamp: ic::time(),
        };

        self.0.insert(token_info.principal_id.clone(), token);
        Ok(true)
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<OperationSuccessful, OperationError> {
        if self.0.contains_key(principal_id) {
            self.0.remove(principal_id);
            return Ok(true);
        }

        Err(OperationError::NonExistentToken)
    }

    pub fn edit(
        &mut self,
        token_info: InputEditToken
    ) -> Result<OperationSuccessful, OperationError> {
        match self.0.get_mut(&token_info.principal_id) {
            None => return Err(OperationError::NonExistentToken),
            Some(token) => {
                if token_info.name.is_some() {
                    token.name = token_info.name.unwrap();
                }

                if token_info.symbol.is_some() {
                    token.symbol = token_info.symbol.unwrap();
                }

                if token_info.description.is_some() {
                    token.description = token_info.description.unwrap();
                }

                if token_info.standard.is_some() {
                    token.standard = token_info.standard.unwrap();
                }

                if token_info.total_supply.is_some() {
                    token.total_supply = Some(token_info.total_supply.unwrap());
                }

                if token_info.logo.is_some() {
                    token.logo = token_info.logo.unwrap();
                }

                if token_info.website.is_some() {
                    token.website = token_info.website.unwrap();
                }

                if token_info.verified.is_some() {
                    token.verified = token_info.verified.unwrap();
                }

                return Ok(true);
            }
        }
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

#[derive(CandidType, Debug)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentToken,
    BadParameters,
}

pub type OperationSuccessful = bool;

#[update]
fn add(token_info: InputAddToken) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if !validate_url(&token_info.logo) || !validate_url(&token_info.website) {
        return Err(OperationError::BadParameters);
    }

    let name = token_info.name.clone();
    if name.len() <= 120 && &token_info.description.len() <= &1200 {
        let db = ic::get_mut::<TokenRegistry>();
        return db.add(token_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
fn remove(principal_id: Principal) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TokenRegistry>();
    db.remove(&principal_id)
}

#[update]
fn edit(
    token_info: InputEditToken
) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if token_info.logo.is_some() && !validate_url(&token_info.logo.clone().unwrap()) {
        return Err(OperationError::BadParameters);
    }

    if token_info.website.is_some() && !validate_url(&token_info.website.clone().unwrap()) {
        return Err(OperationError::BadParameters);
    }

    let db = ic::get_mut::<TokenRegistry>();
    return db.edit(token_info);
}

#[query]
fn get_all() -> Vec<&'static Token> {
    let db = ic::get_mut::<TokenRegistry>();
    db.get_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_token_successfuly() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());
    }

    #[test]
    fn test_add_token_fails_because_of_bad_params() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("bad logo url"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_err());
    }

    #[test]
    fn test_add_token_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        context.update_caller(mock_principals::bob());

        assert!(add(token_info).is_err());
    }

    #[test]
    fn test_edit_token_successfuly() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());

        let token_new_info = InputEditToken {
            principal_id: mock_principals::xtc(),
            name: Some(String::from("Wrapped ICP")),
            symbol: None,
            description: None,
            standard: None,
            logo: None,
            website: None,
            total_supply: None,
            verified: None,
        };
        
        assert!(edit(token_new_info).is_ok());
    }

    #[test]
    fn test_edit_token_fails_because_of_bad_params() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());

        let token_new_info = InputEditToken {
            principal_id: mock_principals::bob(),
            name: Some(String::from("Wrapped ICP")),
            symbol: None,
            description: None,
            standard: None,
            logo: Some(String::from("bad logo url")),
            website: None,
            total_supply: None,
            verified: None,
        };
        
        assert!(edit(token_new_info).is_err());
    }

    #[test]
    fn test_edit_token_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());

        let token_new_info = InputEditToken {
            principal_id: mock_principals::bob(),
            name: Some(String::from("Wrapped ICP")),
            symbol: None,
            description: None,
            standard: None,
            logo: None,
            website: None,
            total_supply: None,
            verified: None,
        };

        context.update_caller(mock_principals::bob());
        
        assert!(edit(token_new_info).is_err());
    }

    #[test]
    fn test_remove_token_successfuly() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());
        
        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[test]
    fn test_remove_token_fails_because_of_unathorized_caller() {
        let context = MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());

        context.update_caller(mock_principals::bob());

        assert!(remove(mock_principals::xtc()).is_err());
    }

    #[test]
    fn test_get_all_successfuly() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        init();

        let token_info = InputAddToken {
            principal_id: mock_principals::xtc(),
            name: String::from("Wrapped ICP"),
            symbol: String::from("WICP"),
            description: String::from("Wrapped IPC description"),
            standard: String::from("DIP20"),
            logo: String::from("https://logo.com"),
            website: String::from("https://website.com"),
            total_supply: Some(1000),
        };

        assert!(add(token_info).is_ok());

        let tokens = get_all();

        assert_eq!(tokens.len(), 1);
    }
}