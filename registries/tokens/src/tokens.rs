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

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct InputEditToken {
    principal_id: Option<Principal>,
    name: String,
    symbol: Option<String>,
    description: Option<String>,
    standard: Option<String>,
    total_supply: Option<u64>,
    logo: Option<String>,
    website: Option<String>,
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
            timestamp: ic::time(),
        };

        self.0.insert(token_info.name.clone(), token);
        Ok(true)
    }

    pub fn remove(&mut self, name: &String) -> Result<OperationSuccessful, OperationError> {
        if self.0.contains_key(name) {
            self.0.remove(name);
            return Ok(true);
        }

        Err(OperationError::NonExistentToken)
    }

    pub fn edit(
        &mut self,
        token_info: InputEditToken
    ) -> Result<OperationSuccessful, OperationError> {
        match self.0.get_mut(&token_info.name) {
            None => return Err(OperationError::NonExistentToken),
            Some(token) => {
                if token_info.principal_id.is_some() {
                    token.principal_id = token_info.principal_id.unwrap();
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

#[derive(CandidType)]
pub enum OperationError {
    NotAuthorized,
    ParamatersNotPassed,
    NonExistentToken,
    BadParameters,
}

pub type OperationSuccessful = bool;

#[update]
fn add(token_info: InputAddToken) -> Result<OperationSuccessful, OperationError> {
    let name = token_info.name.clone();
    if name.len() <= 120 && &token_info.description.len() <= &1200 {
        let db = ic::get_mut::<TokenRegistry>();
        return db.add(token_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
fn remove(name: String) -> Result<OperationSuccessful, OperationError> {
    if !is_controller(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TokenRegistry>();
    db.remove(&name)
}

#[update]
fn edit(
    token_info: InputEditToken
) -> Result<OperationSuccessful, OperationError> {
    let db = ic::get_mut::<TokenRegistry>();
    return db.edit(token_info);
}

#[query]
fn get_all() -> Vec<&'static Token> {
    let db = ic::get_mut::<TokenRegistry>();
    db.get_all()
}