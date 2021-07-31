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

#[derive(Deserialize, CandidType)]
pub struct ProfileMetadata {
    display_name: Option<String>,
    biography: Option<String>,
    emoji: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    version: u32,
}

pub struct ProfileDB(BTreeMap<Principal, ProfileMetadata>);

impl Default for ProfileDB {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl ProfileDB {
    pub fn get_public_profile(&mut self, account: &Principal) -> GetPublicProfileResult {
        if self.0.contains_key(account) {
            let profile = self.0.get(account).clone();
            return GetPublicProfileResult {
                profile_exists: true,
                profile: profile,
            };
        }

        GetPublicProfileResult {
            profile_exists: false,
            profile: None,
        }
    }

    pub fn set_display_name(&mut self, account: Principal, name: String) {
        assert_eq!(self.0.contains_key(&account), true);
        ic_cdk::api::print(String::from("Works."));
    }
}

#[query]
fn name() -> String {
    String::from("Profile Canister")
}

#[derive(Deserialize, CandidType)]
pub struct GetPublicProfileResult {
    profile_exists: bool,
    profile: Option<ProfileMetadata>,
}

#[update]
fn get_public_profile(account: Option<Principal>) -> GetPublicProfileResult {
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.get_public_profile(&account.unwrap_or_else(|| caller()))
}

#[update]
fn set_display_name(name: String) {
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.set_display_name(caller(), name);
}

#[update]
fn set_biography() {}

#[update]
fn set_emoji() {}

#[update]
fn set_avatar() {}
