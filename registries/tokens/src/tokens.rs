use ic_kit::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::validate_url;

pub struct Controllers(pub Vec<Principal>);

impl Default for Controllers {
    fn default() -> Self {
        panic!()
    }
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

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Token {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
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

    pub fn add(&mut self, token_info: Token) -> Result<(), OperationError> {
        self.0.insert(token_info.principal_id, token_info);
        Ok(())
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if self.0.contains_key(principal_id) {
            self.0.remove(principal_id);
            return Ok(());
        }

        Err(OperationError::NonExistentItem)
    }

    pub fn get_info(&self, principal_id: &Principal) -> Option<&Token> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&Token> {
        self.0.values().collect()
    }
}

#[init]
fn init() {
    ic::store(Controllers(vec![ic::caller()]));
}

fn is_controller(account: &Principal) -> bool {
    ic::get::<Controllers>().0.contains(account)
}

#[update]
fn set_controller(new_controller: Principal) -> Result<(), OperationError> {
    if is_controller(&ic::caller()) {
        ic::get_mut::<Controllers>().0.push(new_controller);
        return Ok(());
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
    NonExistentItem,
    BadParameters,
    Unknown(String),
}

#[update]
fn add(token: Token) -> Result<(), OperationError> {
    // Check authorization
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    // Check URLs
    if !validate_url(&token.thumbnail) || !token.clone().frontend.map(validate_url).unwrap_or(true)
    {
        return Err(OperationError::BadParameters);
    }

    // Check Character Limits
    let name = token.name.clone();
    if name.len() > 120 && &token.description.len() > &1200 {
        return Err(OperationError::BadParameters);
    }

    // Check details
    if token.details.len() != 4
        || token.details[0].0 != String::from("symbol")
        || token.details[1].0 != String::from("standard")
        || token.details[2].0 != String::from("total_supply")
        || token.details[3].0 != String::from("verified")
        || (token.details[3].1 != DetailValue::True && token.details[3].1 != DetailValue::False)
    {
        return Err(OperationError::BadParameters);
    }

    let db = ic::get_mut::<TokenRegistry>();
    return db.add(token);
}

#[update]
fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TokenRegistry>();
    db.remove(&principal_id)
}

#[query]
fn get(principal_id: Principal) -> Option<&'static Token> {
    let db = ic::get_mut::<TokenRegistry>();
    db.get_info(&principal_id)
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

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());
    }

    #[test]
    fn test_add_token_fails_because_of_bad_params() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("bad logo url"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_err());
    }

    #[test]
    fn test_add_token_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        context.update_caller(mock_principals::bob());

        assert!(add(token_info).is_err());
    }

    #[test]
    fn test_remove_token_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
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

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
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

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());

        let tokens = get_all();

        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn test_get_all_returns_none_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let tokens = get_all();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_get_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::Text(String::from("1000")),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());

        let token = get(mock_principals::xtc());

        assert!(token.is_some());
    }

    #[test]
    fn test_get_returns_none_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token = get(mock_principals::xtc());

        assert!(token.is_none());
    }
}
