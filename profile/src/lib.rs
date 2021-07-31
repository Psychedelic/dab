use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ops::Bound::Included;

/**
Every item in the map looks like this:
( Principal, ProfileMetadata  )
( UserID, UserProfileMetadata )
**/

struct ProfileMetadata {
    display_name: String,
    biography: String,
    emoji: String,
    avatar: String,
    banner: String
}

pub struct ProfileDB(BTreeMap<Principal, ProfileMetadata>);

impl Default for ProfileDB {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl ProfileDB {}

#[query]
fn name() -> String {
    String::from("Profile Canister")
}
