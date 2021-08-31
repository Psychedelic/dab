extern crate unic;

use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use unic::emoji::char::is_emoji;
use unic::emoji::*;
use validator::validate_url;

const MAX_DESCRIPTION_LIMIT  : usize = 1201;
const MAX_DISPLAY_NAME_LIMIT : usize = 25;

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq)]
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
    pub fn archive(&mut self) -> Vec<(Principal, ProfileMetadata)> {
        let map = std::mem::replace(&mut self.0, BTreeMap::new());
        map.into_iter()
            .collect()
    }
    
    pub fn load(&mut self, archive: Vec<(Principal, ProfileMetadata)>) {
        self.0 = archive.into_iter().collect();
        // self.0.reserve(25_000 - self.0.len());
    }

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
    if &name.len() < &MAX_DISPLAY_NAME_LIMIT && &name.len() > &2 {
        let profile_db = storage::get_mut::<ProfileDB>();
        profile_db.set_display_name(caller(), name);
    }
}

#[update]
fn set_description(description: String) {
    if &description.len() < &MAX_DESCRIPTION_LIMIT {
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

#[cfg(test)]
mod tests {
    use super::{ProfileDB, ProfileMetadata};
    use ic_cdk::export::candid::Principal;

    fn barry() -> Principal {
        Principal::from_text("fterm-bydaq-aaaaa-aaaaa-c").unwrap()
    }

    fn alec() -> Principal {
        Principal::from_text("hozae-racaq-aaaaa-aaaaa-c").unwrap()
    }

    #[test]
    fn display_name() {
        let mut profile_db = ProfileDB::default();
        let mut barry_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_display_name(barry(), String::from("Barry Allen")), ());
        barry_metadata.display_name = Some(String::from("Barry Allen"));

        let profile_metadata = profile_db.get_profile(&barry());

        assert_eq!(profile_metadata.unwrap(), barry_metadata);
    }

    #[test]
    fn set_avatar() {
        let mut profile_db = ProfileDB::default();
        let mut barry_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_avatar(barry(), String::from("Avatar Link")), ());
        barry_metadata.avatar = Some(String::from("Avatar Link"));
    }

    #[test]
    fn set_banner() {
        let mut profile_db = ProfileDB::default();
        let mut barry_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_banner(barry(), String::from("Banner Link")), ());
        barry_metadata.banner = Some(String::from("Banner Link"));
    }

    #[test]
    fn get_profile() {
        let mut profile_db = ProfileDB::default();
        let mut barry_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_avatar(barry(), String::from("Avatar Link")), ());
        barry_metadata.avatar = Some(String::from("Avatar Link"));

        let profile_metadata = profile_db.get_profile(&barry());
        assert_eq!(profile_metadata.unwrap(), barry_metadata);

    }

    #[test]
    fn set_emoji() {
        let mut profile_db = ProfileDB::default();
        let mut barry_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_emoji(barry(), String::from("⚡️")), ());
        barry_metadata.emoji = Some(String::from("⚡️"));
    }

    #[test]
    fn set_description() {
        let mut profile_db = ProfileDB::default();
        let mut barry_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_description(barry(), String::from("Fastest man alive!")), ());
        barry_metadata.description = Some(String::from("Fastest man alive!"));
    }

    #[test]
    fn null_case() {
        let mut profile_db = ProfileDB::default();

        // Testing to see what happens if the profile doesn't exist
        assert_eq!(profile_db.get_profile(&alec()), None);
    }

    #[test]
    fn partial_case() {
        let mut profile_db = ProfileDB::default();
        let mut alec_metadata: ProfileMetadata = ProfileMetadata { display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_display_name(alec(), String::from("Alec Holland")), ());
        alec_metadata.display_name = Some(String::from("Alec Holland"));
    
        assert_eq!(profile_db.get_profile(&alec()).unwrap(), alec_metadata);
    }
}