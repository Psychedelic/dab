extern crate unic;

use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use unic::emoji::char::{is_emoji, Emoji};
use unic::emoji::*;

/**
Every item in the map looks like this:
( Principal, ProfileMetadata  )
( UserID, UserProfileMetadata )
**/

// TODO: Every input should be checked. The length of description, the validity of avatar/banner url and ...
// TODO: Set a policy for version control

#[derive(Deserialize, CandidType, Clone)]
pub struct ProfileMetadata {
    display_name: Option<String>,
    description: Option<String>,
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
                        description: None,
                        emoji: None,
                        avatar: None,
                        banner: None,
                        version: 0,
                    },
                );
            }
        }
    }

    pub fn set_description(&mut self, account: Principal, description: String) {
        match self.0.get_mut(&account) {
            Some(x) => {
                x.description = Some(description);
            }
            None => {
                self.0.insert(
                    account,
                    ProfileMetadata {
                        display_name: None,
                        description: Some(description),
                        emoji: None,
                        avatar: None,
                        banner: None,
                        version: 0,
                    },
                );
            }
        }
    }

    pub fn set_emoji(&mut self, account: Principal, emoji: String) {
        match self.0.get_mut(&account) {
            Some(x) => {
                x.emoji = Some(emoji);
            }
            None => {
                self.0.insert(
                    account,
                    ProfileMetadata {
                        display_name: None,
                        description: None,
                        emoji: Some(emoji),
                        avatar: None,
                        banner: None,
                        version: 0,
                    },
                );
            }
        }
    }

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
fn set_description(description: String) {
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.set_description(caller(), description);
}

#[update]
fn set_emoji(input: String) {
    let emojis: Vec<char> = input.chars().take(1).collect();
    assert_eq!(is_emoji(emojis[0]), true);
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.set_emoji(caller(), input);
}

#[update]
fn set_avatar() {}
