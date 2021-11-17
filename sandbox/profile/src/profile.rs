use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::ic::*;
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use unic::emoji::char::is_emoji;
use unic::emoji::*;
use validator::validate_url;

const MAX_DESCRIPTION_LIMIT  : usize = 1201;
const MAX_DISPLAY_NAME_LIMIT : usize = 25;
const MAX_USERNAME_LIMIT : u32 = 9999;

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq)]
pub struct ProfileMetadata {
    username: Option<String>,
    user_id: Option<u32>,
    display_name: Option<String>,
    description: Option<String>,
    emoji: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    version: u32,
}

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq)]
pub struct InputProfileMetadata {
    username: Option<String>,
    user_id: Option<u32>,
    display_name: Option<String>,
    description: Option<String>,
    emoji: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,   
}

pub struct ProfileDB(BTreeMap<Principal, ProfileMetadata>);


pub type OperationSuccessful = Option<String>;

#[derive(Deserialize, CandidType, Clone)]
pub enum OperationError {
    BadParameters,
    NotFound,
}

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq)]
pub struct UserInfo {
    user_id: u32, 
    username: String,
}

pub struct UsersDB(HashMap<u32, UserInfo>);

impl Default for ProfileDB {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl Default for UsersDB {
    fn default() -> Self {
        Self(HashMap::new())
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
                        username: None,
                        user_id: None,
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

    pub fn set_username(&mut self, account: Principal, username: String, user_id: u32) {
        match self.0.contains_key(&account) {
            false => match self.0.get_mut(&account) {
                Some(x) => {
                    x.username = Some(username);
                    x.user_id = Some(user_id);
                    x.version += 1;
                }
                None => {
                    self.0.insert(
                        account,
                        ProfileMetadata {
                            username: Some(username),
                            user_id: Some(user_id),
                            display_name: None,
                            description: None,
                            emoji: None,
                            avatar: None,
                            banner: None,
                            version: 0,
                        },
                    );
                }
            },
            true => panic!("Account already exists"),
        };
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
                        username: None,
                        user_id: None,
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
                        username: None,
                        user_id: None,
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
                        username: None,
                        user_id: None,
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
                        username: None,
                        user_id: None,
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

    pub fn set_profile(&mut self, account: Principal, metadata: InputProfileMetadata) -> bool{
        let profile_metadata = ProfileMetadata {
            username: metadata.username,
            user_id: metadata.user_id,
            display_name: metadata.display_name,
            description: metadata.description,
            emoji: metadata.emoji,
            avatar: metadata.avatar,
            banner: metadata.banner,
            version: 0,
        };
        self.0.insert(account, profile_metadata);
        if !self.0.contains_key(&account) {
            return false;
        }
        true 
    }

}

impl UsersDB {
    pub fn set_username_id(&mut self, user_id: u32, username: String) -> bool {
        if !self.0.contains_key(&user_id) {
            self.0.insert(user_id, UserInfo{ user_id: user_id, username: username });
            return true
        }
        false
    }

    pub fn get_user_info(&mut self, user_id: &u32) -> Option<UserInfo> {
        self.0.get(user_id).cloned()
    }
}

#[query]
fn name() -> String {
    String::from("Profile Canister")
}

#[query]
fn get_username_id(user_id: u32) -> Option<UserInfo> {
    let users_db = ic::get_mut::<UsersDB>();
    users_db.get_user_info(&user_id)
}

#[update]
fn get_profile(account: Option<Principal>) -> Option<ProfileMetadata> {
    let profile_db = ic::get_mut::<ProfileDB>();
    profile_db.get_profile(&account.unwrap_or_else(|| ic::caller()))
}

#[update]
fn set_display_name(name: String) -> Result<OperationSuccessful, OperationError> {
    if &name.len() < &MAX_DISPLAY_NAME_LIMIT && &name.len() > &2 {
        let profile_db = ic::get_mut::<ProfileDB>();
        profile_db.set_display_name(ic::caller(), name);
        return Ok(None)
    }
    return Err(OperationError::BadParameters);
}

#[update]
fn set_description(description: String) -> Result<OperationSuccessful ,OperationError>{
    if &description.len() < &MAX_DESCRIPTION_LIMIT {
        let profile_db = ic::get_mut::<ProfileDB>();
        profile_db.set_description(ic::caller(), description);
        return Ok(None);
    }
    return Err(OperationError::BadParameters)
}

#[update]
fn set_emoji(input: String) -> Result<OperationSuccessful, OperationError> {
    let emojis: Vec<char> = input.chars().take(1).collect();
    if is_emoji(emojis[0]) {
        let profile_db = ic::get_mut::<ProfileDB>();
        profile_db.set_emoji(ic::caller(), input);
        return Ok(None);  
    }
    return Err(OperationError::BadParameters);
}

#[update]
fn set_avatar(url: String) -> Result<OperationSuccessful, OperationError> {
    if validate_url(&url) {
        let profile_db = ic::get_mut::<ProfileDB>();
        profile_db.set_avatar(ic::caller(), url);
        return Ok(None);
    }
    return Err(OperationError::BadParameters);
}

#[update]
fn set_banner(url: String) -> Result<OperationSuccessful, OperationError>{
    if validate_url(&url) {
        let profile_db = ic::get_mut::<ProfileDB>();
        profile_db.set_banner(ic::caller(), url);
        return Ok(None);
    }
    return Err(OperationError::BadParameters);
}

#[update]
fn set_profile(metadata: InputProfileMetadata) {
    let profile_db = ic::get_mut::<ProfileDB>();
    profile_db.set_profile(ic::caller(), metadata);
}

#[update]
fn set_username(username: String, user_id: u32) {
    if &username.len() < &(*&MAX_USERNAME_LIMIT as usize) && &username.len() > &2 {
        let profile_db = ic::get_mut::<ProfileDB>();
        let users_db = ic::get_mut::<UsersDB>();
        if users_db.set_username_id(user_id, username.clone()) {
            profile_db.set_username(caller(), username, user_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_display_name_for_non_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_display_name(String::from("Correct display name")).is_ok());
        assert!(get_profile(Some(mock_principals::alice())).is_some());
    }

    #[test]
    fn test_set_display_name_for_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        let alice_profile_metadata: InputProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: Some(String::from("Original display name")), description: None, emoji: None, avatar: None, banner: None };

        set_profile(alice_profile_metadata.clone());

        assert!(get_profile(Some(mock_principals::alice())).is_some());

        let edited_display_name = String::from("Edited display name");

        assert!(set_display_name(edited_display_name).is_ok());

        let alice_profile = get_profile(Some(mock_principals::alice())).unwrap();

        assert_eq!(alice_profile.display_name.unwrap(), String::from("Edited display name"));
        assert_eq!(alice_profile.version.clone(), 1);
    }

    #[test]
    fn test_set_display_name_with_large_input_throws_an_error() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());

        let display_name = String::from("Veeeeeryyyyy laaaaargeeeeee displaaaayyy naaaaaaaameeeeeeeeee");

        assert!(set_display_name(display_name).is_err());
        assert!(get_profile(Some(mock_principals::alice())).is_none());
    }

    #[test]
    fn test_set_avatar_for_non_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_avatar(String::from("http://image.jpeg")).is_ok());
        assert!(get_profile(Some(mock_principals::alice())).is_some());
    }

    #[test]
    fn test_set_avatar_for_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        let alice_profile_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: None, description: None, emoji: None, avatar: Some(String::from("http://pre-image.jpeg")), banner: None };

        set_profile(alice_profile_metadata.clone());

        assert!(get_profile(Some(mock_principals::alice())).is_some());

        let edited_avatar = String::from("http://image.jpeg");

        assert!(set_avatar(edited_avatar).is_ok());

        let alice_profile = get_profile(Some(mock_principals::alice())).unwrap();

        assert_eq!(alice_profile.avatar.unwrap(), String::from("http://image.jpeg"));
        assert_eq!(alice_profile.version.clone(), 1);
    }

    #[test]
    fn test_set_avatar_with_invalid_url_throws_an_error() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_avatar(String::from("123")).is_err());
        assert!(get_profile(Some(mock_principals::alice())).is_none());
    }

    #[test]
    fn test_set_banner_for_non_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_banner(String::from("http://image.jpeg")).is_ok());
        assert!(get_profile(Some(mock_principals::alice())).is_some());
    }

    #[test]
    fn test_set_banner_for_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        let alice_profile_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: None, description: None, emoji: None, avatar: None, banner: Some(String::from("http://pre-banner.jpeg")) };

        set_profile(alice_profile_metadata.clone());

        assert!(get_profile(Some(mock_principals::alice())).is_some());

        let edited_banner = String::from("http://image.jpeg");

        assert!(set_banner(edited_banner).is_ok());

        let alice_profile = get_profile(Some(mock_principals::alice())).unwrap();

        assert_eq!(alice_profile.banner.unwrap(), String::from("http://image.jpeg"));
        assert_eq!(alice_profile.version.clone(), 1);
    }

    #[test]
    fn test_set_banner_with_invalid_url_throws_an_error() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_banner(String::from("123")).is_err());
        assert!(get_profile(Some(mock_principals::alice())).is_none());
    }

    #[test]
    fn test_get_profile_returns_profile() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        let mut alice_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: None, description: None, emoji: None, avatar: None, banner: None };

        set_profile(alice_metadata.clone());
        assert_eq!(get_profile(Some(mock_principals::alice())).unwrap(), alice_metadata);
    }

    #[test]
    fn test_get_profile_returns_none_for_non_existent_profile() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
    }

    #[test]
    fn test_set_emoji_for_non_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_emoji(String::from("⚡️")).is_ok());
        assert!(get_profile(Some(mock_principals::alice())).is_some());
    }

    #[test]
    fn test_set_emoji_for_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        let alice_profile_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: None, description: None, emoji: Some(String::from("⚡️")), avatar: None, banner: None };

        set_profile(alice_profile_metadata.clone());

        assert!(get_profile(Some(mock_principals::alice())).is_some());

        let edited_emoji = String::from("😚");

        assert!(set_emoji(edited_emoji).is_ok());

        let alice_profile = get_profile(Some(mock_principals::alice())).unwrap();

        assert_eq!(alice_profile.emoji.unwrap(), String::from("😚"));
        assert_eq!(alice_profile.version.clone(), 1);
    }

    #[test]
    fn test_set_emoji_with_invalid_input_throws_an_error() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_banner(String::from("1")).is_err());
        assert!(get_profile(Some(mock_principals::alice())).is_none());
    }

    fn test_set_description_for_non_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());
        assert!(set_description(String::from("Correct description")).is_ok());
        assert!(get_profile(Some(mock_principals::alice())).is_some());
    }

    #[test]
    fn test_set_description_for_existent_profile_runs_succesfully() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        let alice_profile_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, description: Some(String::from("Original description")), display_name: None, emoji: None, avatar: None, banner: None };

        set_profile(alice_profile_metadata.clone());

        assert!(get_profile(Some(mock_principals::alice())).is_some());

        let edited_description = String::from("Edited description");

        assert!(set_description(edited_description).is_ok());

        let alice_profile = get_profile(Some(mock_principals::alice())).unwrap();

        assert_eq!(alice_profile.description.unwrap(), String::from("Edited description"));
        assert_eq!(alice_profile.version.clone(), 1);
    }

    #[test]
    fn test_set_description_with_large_input_throws_an_error() {
        MockContext::new()
        .with_caller(mock_principals::alice())
        .inject();

        assert!(get_profile(Some(mock_principals::alice())).is_none());

        let description = String::from("Very large descriptioooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");

        assert!(set_description(description).is_err());
        assert!(get_profile(Some(mock_principals::alice())).is_none());
    }

    #[test]
    fn set_username() {
        let mut profile_db = ProfileDB::default();
        let mut alice_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };
        assert_eq!(profile_db.set_username(mock_principals::alice(), String::from("Alice"), 1111), ());
        alice_metadata.username = Some(String::from("Alice"));
    }

    #[test] 
    fn null_case() {
        let mut profile_db = ProfileDB::default();

        // Testing to see what happens if the profile doesn't exist
        assert_eq!(profile_db.get_profile(&mock_principals::alice()), None);
    }

    #[test]
    fn partial_case() {
        let mut profile_db = ProfileDB::default();
        let mut alice_metadata: ProfileMetadata = ProfileMetadata { user_id: None, username: None, display_name: None, description: None, emoji: None, avatar: None, banner: None, version: 0 };

        assert_eq!(profile_db.set_display_name(mock_principals::alice(), String::from("Alice")), ());
        alice_metadata.display_name = Some(String::from("Alice"));
    
        assert_eq!(profile_db.get_profile(&mock_principals::alice()).unwrap(), alice_metadata);
    }
}
