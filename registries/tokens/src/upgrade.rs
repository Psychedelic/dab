use crate::common_types::*;
use crate::management::Admins;
use crate::tokens::TokenRegistry;
use ic_kit::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize)]
pub struct TokenV0 {
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
}

#[derive(CandidType, Deserialize)]
pub struct StableStorageV0 {
    db: Vec<(Principal, TokenV0)>,
    admins: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct StableStorage {
    db: Vec<(Principal, Token)>,
    admins: Vec<Principal>,
}

impl From<TokenV0> for Token {
    fn from(wrapper: TokenV0) -> Token {
        return Token {
            name: wrapper.name,
            description: wrapper.description,
            thumbnail: wrapper.thumbnail,
            frontend: wrapper.frontend,
            principal_id: wrapper.principal_id,
            details: wrapper.details,
            submitter: ic::id(),
            last_updated_by: ic::id(),
            last_updated_at: ic::time(),
        };
    }
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<TokenRegistry>().archive();
    let admins = ic::get_mut::<Admins>().0.clone();

    let stable = StableStorage { db, admins };

    match ic::stable_store((stable,)) {
        Ok(_) => (),
        Err(candid_err) => {
            trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
pub fn post_upgrade() {
    if let Ok((stable,)) = ic::stable_restore::<(StableStorageV0,)>() {
        let mut updated_tokens: Vec<(Principal, Token)> = vec![];
        for entry in stable.db {
            let updated_token: Token = Token::from(entry.1);
            updated_tokens.push((entry.0, updated_token));
        }
        ic::get_mut::<TokenRegistry>().load(updated_tokens);
        //ic::store(Admins(stable.controllers));
        ic::store(Admins(stable.admins));
    }
}
