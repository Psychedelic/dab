use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;
use std::any::Any;
use validator::validate_url;

use crate::common_types::*;
use crate::management::*;

pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
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
pub fn init() {
    ic::store(Admins(vec![ic::caller()]));
}

#[query]
pub fn name() -> String {
    String::from("Token Registry Canister")
}

#[update]
pub fn add(token: Token) -> Result<(), OperationError> {
    // Check authorization
    if !is_admin(&ic::caller()) {
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
pub fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<TokenRegistry>();
    db.remove(&principal_id)
}

#[query]
pub fn get(principal_id: Principal) -> Option<&'static Token> {
    let db = ic::get_mut::<TokenRegistry>();
    db.get_info(&principal_id)
}

#[query]
pub fn get_all() -> Vec<&'static Token> {
    let db = ic::get_mut::<TokenRegistry>();
    db.get_all()
}