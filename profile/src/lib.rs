extern crate unic;

use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use unic::emoji::char::is_emoji;
use unic::emoji::*;
use validator::validate_url;

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
    pub fn get_profile(&mut self, account: &Principal) -> Option<ProfileMetadata> {
        self.0.get(account).cloned()
    }

    pub fn set_display_name(&mut self, account: Principal, name: String) {
        match self.0.get_mut(&account) {
            Some(x) => {
                x.display_name = Some(name);
                x.version += 1;
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
                x.version += 1;
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
                x.version += 1;
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

    pub fn set_avatar(&mut self, account: Principal, avatar: String) {
        match self.0.get_mut(&account) {
            Some(x) => {
                x.avatar = Some(avatar);
                x.version += 1;
            }
            None => {
                self.0.insert(
                    account,
                    ProfileMetadata {
                        display_name: None,
                        description: None,
                        emoji: None,
                        avatar: Some(avatar),
                        banner: None,
                        version: 0,
                    },
                );
            }
        }
    }

    pub fn set_banner(&mut self, account: Principal, banner: String) {
        match self.0.get_mut(&account) {
            Some(x) => {
                x.banner = Some(banner);
                x.version += 1;
            }
            None => {
                self.0.insert(
                    account,
                    ProfileMetadata {
                        display_name: None,
                        description: None,
                        emoji: None,
                        avatar: None,
                        banner: Some(banner),
                        version: 0,
                    },
                );
            }
        }
    }

    pub fn set_profile(&mut self, account: Principal, profile_data: ProfileMetadata) {
        self.0.insert(account, profile_data);
    }
}

#[query]
fn name() -> String {
    String::from("Profile Canister")
}

#[update]
fn get_profile(account: Option<Principal>) -> Option<ProfileMetadata> {
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.get_profile(&account.unwrap_or_else(|| caller()))
}

#[update]
fn set_display_name(name: String) {
    if &name.len() < &25 && &name.len() > &2 {
        let profile_db = storage::get_mut::<ProfileDB>();
        profile_db.set_display_name(caller(), name);
    }
}

#[update]
fn set_description(description: String) {
    if &description.len() < &1200 {
        let profile_db = storage::get_mut::<ProfileDB>();
        profile_db.set_description(caller(), description);
    }
}

#[update]
fn set_emoji(input: String) {
    let emojis: Vec<char> = input.chars().take(1).collect();
    assert_eq!(is_emoji(emojis[0]), true);
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.set_emoji(caller(), input);
}

#[update]
fn set_avatar(url: String) {
    if validate_url(&url) {
        let profile_db = storage::get_mut::<ProfileDB>();
        profile_db.set_avatar(caller(), url);
    }
}

#[update]
fn set_banner(url: String) {
    if validate_url(&url) {
        let profile_db = storage::get_mut::<ProfileDB>();
        profile_db.set_banner(caller(), url);
    }
}

#[update]
fn set_profile(profile_data: ProfileMetadata) {
    let profile_db = storage::get_mut::<ProfileDB>();
    profile_db.set_profile(caller(), profile_data);
}
