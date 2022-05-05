use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::any::Any;
use std::collections::HashMap;
use std::str::FromStr;
use validator::validate_url;

use crate::common_types::*;
use crate::management::*;

pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
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
pub async fn add(add_token_input: AddTokenInput) -> Result<(), OperationError> {
    // Check authorization
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    // Check URLs
    if !validate_url(&add_token_input.thumbnail)
        || !add_token_input
            .clone()
            .frontend
            .map(validate_url)
            .unwrap_or(true)
    {
        return Err(OperationError::BadParameters);
    }

    // Check Character Limits
    let name = add_token_input.name.clone();
    if name.len() > 120 && add_token_input.description.len() > 1200 {
        return Err(OperationError::BadParameters);
    }

    // Check details
    if add_token_input.details.len() != 6
        || add_token_input.details[0].0 != String::from("symbol")
        || add_token_input.details[1].0 != String::from("standard")
        || add_token_input.details[2].0 != String::from("total_supply")
        || add_token_input.details[3].0 != String::from("verified")
        || (add_token_input.details[3].1 != DetailValue::True
            && add_token_input.details[3].1 != DetailValue::False)
        || add_token_input.details[4].0 != String::from("decimals")
        || add_token_input.details[5].0 != String::from("fee")
    {
        return Err(OperationError::BadParameters);
    }

    // Add the collection to the canister registry
    let mut call_arg = add_token_input.clone();
    call_arg.details = vec![(
        "category".to_string(),
        DetailValue::Text("Token".to_string()),
    )];

    let _registry_add_response: RegistryResponse = match ic::call(
        Principal::from_str(CANISTER_REGISTRY_ID).unwrap(),
        "add",
        (call_arg,),
    )
    .await
    {
        Ok((x,)) => x,
        Err((_code, msg)) => {
            return Err(OperationError::Unknown(msg));
        }
    };

    let db = ic::get_mut::<TokenRegistry>();
    return db.add(Token::from(add_token_input));
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
