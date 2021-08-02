use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;

/**
Every item in the map looks like this:
( Principal, ProfileMetadata  )
( UserID, UserProfileMetadata )
**/

#[derive(Deserialize, CandidType, Clone)]
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
    pub fn get_public_profile(&mut self, account: &Principal) -> Option<ProfileMetadata> {
        self.0.get(account).cloned()
    }

    pub fn set_display_name(&mut self, account: Principal, name: String) {
        match self.0.get_mut(&account) {
            Some(x) => {
                x.display_name = Some(name);
            }
            None => {
                    self.0.insert(
                        account,
                        ProfileMetadata {
                            display_name: Some(name),
                            biography: None,
                            emoji: None,
                            avatar: None,
                            banner: None,
                            version: 0,
                        },
                    );
                }
        }
    }

    pub fn set_biography(&mut self, account: Principal, biography: String) {}

    pub fn set_emoji(&mut self, account: Principal, emoji: String) {}

    pub fn set_avatar(&mut self, account: Principal, avatar: String) {}
}

#[query]
fn name() -> String {
    String::from("Profile Canister")
}

#[update]
fn get_public_profile(account: Option<Principal>) -> Option<ProfileMetadata> {
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
