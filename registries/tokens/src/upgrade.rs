use crate::tokens::{Controllers, Token, TokenRegistry};
use std::fmt::{self, Debug};

use ic_kit::candid::{CandidType, Deserialize, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenV0 {
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

impl fmt::Display for TokenV0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<TokenV0> for Token {
    fn from(cs: TokenV0) -> Self {
        Token {
            name: cs.name,
            description: cs.description,
            frontend: Some(cs.website),
            thumbnail: cs.logo,
            details: vec![
                (String::from("symbol"), cs.symbol),
                (String::from("standard"), cs.standard),
                (
                    String::from("total_supply"),
                    cs.total_supply.unwrap_or_else(|| 0).to_string(),
                ),
                (String::from("verified"), cs.verified.to_string()),
            ],
        }
    }
}

#[derive(CandidType, Deserialize)]
struct StableStorageV0 {
    db: Vec<(Principal, TokenV0)>,
    controllers: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    db: Vec<(Principal, Token)>,
    controllers: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let db = ic::get_mut::<TokenRegistry>().archive();
    let controllers = ic::get_mut::<Controllers>().0.clone();

    let stable = StableStorage { db, controllers };

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
        let mut token_list = Vec::with_capacity(stable.db.len());

        for (_key, token_info) in stable.db.into_iter().enumerate() {
            let metadata_info: Token = token_info.1.into();
            let principal_info: Principal = token_info.0.into();

            token_list.push((principal_info, metadata_info));
        }

        ic::get_mut::<TokenRegistry>().load(token_list);

        //ic::get_mut::<TokenRegistry>().load(stable.db);
        ic::store(Controllers(stable.controllers));
    }
}
