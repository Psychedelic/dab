use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 24;

pub struct Fleek(pub Vec<Principal>);

impl Default for Fleek {
    fn default() -> Self {
        panic!()
    }
}

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
    name: String,
    description: String,
    url: String,
    logo_url: String,
    version: u32,
}

#[derive(Deserialize, CandidType, Clone)]
pub struct InputCanisterMetadata {
    name: String,
    description: String,
    url: String,
    logo_url: String,
}

#[derive(Default)]
pub struct CanisterDB(HashMap<Principal, CanisterMetadata>);

impl CanisterDB {
    pub fn archive(&mut self) -> Vec<(Principal, CanisterMetadata)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, CanisterMetadata)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn get_info(&mut self, canisters: Vec<Principal>) -> Vec<Option<&CanisterMetadata>> {
        // self.0.get(canister).cloned()
        let mut list: Vec<Option<&CanisterMetadata>> = vec![];
        for canister in canisters {
            let item = self.0.get(&canister);
            list.push(item);
        }
        list
    }

    pub fn add_canister(
        &mut self,
        canister: Principal,
        metadata: InputCanisterMetadata,
    ) -> Result<(), Failure> {
        let canister_metadata = CanisterMetadata {
            name: metadata.name,
            description: metadata.description,
            url: metadata.url,
            logo_url: metadata.logo_url,
            version: 0,
        };
        self.0.insert(canister, canister_metadata);
        if !self.0.contains_key(&canister) {
            return Err(Failure::UnknownError);
        }
        Ok(())
    }

    pub fn remove_canister(&mut self, canister: &Principal) -> Result<(), Failure> {
        if !self.0.contains_key(canister) {
            return Err(Failure::NonExistentCanister);
        }
        self.0.remove(canister);
        Ok(())
    }

    /* pub fn set_description(&mut self, account: Principal, canister: &Principal, description: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.description = description;
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_url(&mut self, account: Principal, canister: &Principal, url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.url = url;
                x.version += 1;
            }
            None => return,
        }
    }

    pub fn set_logo(&mut self, account: Principal, canister: &Principal, logo_url: String) {
        match self.0.get_mut(canister) {
            Some(x) => {
                assert!(is_controller(canister, &account));
                x.logo_url = logo_url;
                x.version += 1;
            }
            None => return,
        }
    } **/
}

#[derive(CandidType, Debug, PartialEq)]
pub enum Failure {
    NotAuthorized,
    BadParameters,
    NonExistentCanister,
    InterCanisterCall(String),
    UnknownError,
}

#[init]
fn init() {
    ic::store(Fleek(vec![ic::caller()]));
}

fn is_fleek(account: &Principal) -> bool {
    ic::get::<Fleek>().0.contains(account)
}

#[update]
fn set_admin(new_admin: Principal) -> Result<(), Failure> {
    if is_fleek(&ic::caller()) {
        ic::get_mut::<Fleek>().0.push(new_admin);
        return Ok(());
    }
    Err(Failure::NotAuthorized)
}

#[query]
fn name() -> String {
    String::from("Canister Registry")
}

#[query]
fn get_info(canisters: Vec<Principal>) -> Vec<Option<&'static CanisterMetadata>> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_info(canisters)
}

#[update]
fn add_canister(canister: Principal, metadata: InputCanisterMetadata) -> Result<(), Failure> {
    if !is_fleek(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    } else if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.logo_url)
        || !validate_url(&metadata.url)
    {
        return Err(Failure::BadParameters);
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(canister, metadata)
}

#[update]
fn remove_canister(canister: Principal) -> Result<(), Failure> {
    if !is_fleek(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    }
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.remove_canister(&canister)
}

#[update]
async fn update_canister(canister: Principal) -> Result<(), Failure> {
    let metadata: InputCanisterMetadata =
        match ic::call(canister, String::from("dab_registry"), ((),)).await {
            Ok((x,)) => x,
            Err((_code, msg)) => {
                return Err(Failure::InterCanisterCall(msg));
            }
        };

    if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.logo_url)
        || !validate_url(&metadata.url)
    {
        return Err(Failure::BadParameters);
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(canister, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn nft_registry() -> Principal {
        Principal::from_text("aipdg-waaaa-aaaah-aaq5q-cai").unwrap()
    }

    #[test]
    fn test_controller() {
        // Alice is an admin
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_info = InputCanisterMetadata {
            name: String::from("XTC"),
            description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
            url: String::from("https://frontend_url.com"),
            logo_url: String::from("https://logo_url.com"),
        };

        let addition = add_canister(mock_principals::xtc(), canister_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove_canister(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        // Bob is not an admin so the operation should not be successful
        ctx.update_caller(mock_principals::bob());
        let addition = add_canister(mock_principals::xtc(), canister_info.clone());
        assert!(addition.is_err());

        // Alice makes Bob an admin and now he can add/remove canisters
        ctx.update_caller(mock_principals::alice());
        let operation = set_admin(mock_principals::bob());
        assert!(operation.is_ok());

        ctx.update_caller(mock_principals::bob());
        let addition = add_canister(mock_principals::xtc(), canister_info);
        assert!(addition.is_ok());
    }

    #[test]
    fn get_information() {
        // Alice is an admin
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let xtc_info = InputCanisterMetadata {
            name: String::from("XTC"),
            description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
            url: String::from("https://frontend_url.com"),
            logo_url: String::from("https://logo_url.com"),
        };

        let xtc_metadata = CanisterMetadata {
            name: String::from("XTC"),
            description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
            url: String::from("https://frontend_url.com"),
            logo_url: String::from("https://logo_url.com"),
            version: 0
        };

        let addition = add_canister(mock_principals::xtc(), xtc_info.clone());
        assert!(addition.is_ok());

        let operation_get_info = get_info(vec![mock_principals::xtc()]);
        let expected_response: Vec<Option<&CanisterMetadata>> = vec![Some(&xtc_metadata)];
        assert_eq!(
            operation_get_info[0].unwrap(),
            expected_response[0].unwrap()
        );

        // Users who are not admins should be able to access the information, too
        ctx.update_caller(mock_principals::bob());
        let operation_get_info = get_info(vec![mock_principals::xtc()]);
        assert_eq!(
            operation_get_info[0].unwrap(),
            expected_response[0].unwrap()
        );

        // users should be able to ask for multiple canisters
        // We switch back to alice to add another canister
        ctx.update_caller(mock_principals::alice());

        let nft_info = InputCanisterMetadata {
            name: String::from("NFT Registry"),
            description: String::from("DAB's NFT registry provides its users with information for every nft canister in the registry."),
            url: String::from("https://frontend_url.com"),
            logo_url: String::from("https://logo_url.com"),
        };

        let nft_metadata = CanisterMetadata {
            name: String::from("NFT Registry"),
            description: String::from("DAB's NFT registry provides its users with information for every nft canister in the registry."),
            url: String::from("https://frontend_url.com"),
            logo_url: String::from("https://logo_url.com"),
            version: 0
        };

        let addition = add_canister(nft_registry(), nft_info);
        assert!(addition.is_ok());

        // Now Bob should be able to ask for both xtc and nft registry canister
        ctx.update_caller(mock_principals::bob());
        let operation_get_info = get_info(vec![mock_principals::xtc(), nft_registry()]);
        let expected_response: Vec<Option<&CanisterMetadata>> =
            vec![Some(&xtc_metadata), Some(&nft_metadata)];
        assert_eq!(operation_get_info, expected_response);
    }

    #[test]
    fn remove() {
        // Alice is an admin
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let xtc_info = InputCanisterMetadata {
                name: String::from("XTC"),
                description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
                url: String::from("https://frontend_url.com"),
                logo_url: String::from("https://logo_url.com"),
            };

        let addition = add_canister(mock_principals::xtc(), xtc_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove_canister(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        // the canister should return an error if we try to remove a non-existent canister
        let remove_operation = remove_canister(mock_principals::xtc());
        assert_eq!(
            remove_operation.err().unwrap(),
            Failure::NonExistentCanister
        );

        // Bob should not be able to remove a canister because he is not an admin
        ctx.update_caller(mock_principals::bob());
        let remove_operation = remove_canister(mock_principals::xtc());
        assert_eq!(remove_operation.err().unwrap(), Failure::NotAuthorized);
    }
}

/*#[update]
fn set_url(canister: Principal, url: String) {
    if validate_url(&url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_url(caller(), &canister, url);
    }
}

#[update]
fn set_description(canister: Principal, description: String) {
    if &description.len() < &DESCRIPTION_LIMIT {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_description(caller(), &canister, description);
    }
}

#[update]
fn set_logo(canister: Principal, logo_url: String) {
    if validate_url(&logo_url) {
        let canister_db = ic::get_mut::<CanisterDB>();
        canister_db.set_logo(caller(), &canister, logo_url);
    }
} **/
