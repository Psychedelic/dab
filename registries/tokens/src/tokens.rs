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

    pub fn get_all_paginated(
        &self,
        offset: usize,
        _limit: usize,
    ) -> Result<Vec<&Token>, OperationError> {
        let tokens: Vec<&Token> = self.0.values().collect();

        if offset > tokens.len() {
            return Err(OperationError::BadParameters(String::from(
                "Offset out of bound.",
            )));
        }

        let mut limit = _limit;

        if offset + _limit > tokens.len() {
            limit = tokens.len() - offset;
        }

        return Ok(tokens[offset..(offset + limit)].to_vec());
    }

    pub fn get_amount(&self) -> usize {
        return self.0.values().len();
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
pub async fn add(token: Token) -> Result<(), OperationError> {
    // Check authorization
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if &token.name.len() > &NAME_LIMIT {
        return Err(OperationError::BadParameters(format!(
            "Name field has to be less than {} characters long.",
            NAME_LIMIT
        )));
    }

    if &token.description.len() > &DESCRIPTION_LIMIT {
        return Err(OperationError::BadParameters(format!(
            "Description field has to be less than {} characters long.",
            DESCRIPTION_LIMIT
        )));
    }

    if !validate_url(&token.thumbnail) {
        return Err(OperationError::BadParameters(String::from(
            "Thumbnail field has to be a url.",
        )));
    }

    if token.clone().frontend.is_some() && !validate_url(token.clone().frontend.unwrap()) {
        return Err(OperationError::BadParameters(String::from(
            "Frontend field has to be a url.",
        )));
    }

    if &token.details.len() < &4 {
        return Err(OperationError::BadParameters(String::from(
            "Details field has to specifiy: symbol, standard, total_supply and verified fields.",
        )));
    }

    if &token.details[0].0 != &String::from("symbol") {
        return Err(OperationError::BadParameters(String::from(
            "First detail field has to be symbol.",
        )));
    }

    if &token.details[1].0 != &String::from("standard") {
        return Err(OperationError::BadParameters(String::from(
            "Second detail field has to be standard.",
        )));
    }

    if &token.details[2].0 != &String::from("total_supply") {
        return Err(OperationError::BadParameters(String::from(
            "Third detail field has to be total_supply.",
        )));
    }

    if &token.details[0].0 != &String::from("verified")
        && &token.details[3].1 != &DetailValue::True
        && &token.details[3].1 != &DetailValue::False
    {
        return Err(OperationError::BadParameters(String::from(
            "Fourth detail field has to be verified (boolean).",
        )));
    }

    // Add the collection to the canister registry
    let mut call_arg: Token = token.clone();
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

#[query]
pub fn get_all_paginated(
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<GetAllPaginatedResponse, OperationError> {
    let db = ic::get_mut::<TokenRegistry>();
    let tokens = db.get_all_paginated(offset.unwrap_or(0), limit.unwrap_or(DEFAULT_LIMIT))?;
    let amount = db.get_amount();

    return Ok(GetAllPaginatedResponse { tokens, amount });
}
