use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::validate_url;

const DESCRIPTION_LIMIT: usize = 1200;
const NAME_LIMIT: usize = 24;

pub struct Fleek(pub Vec<Principal>);

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub enum CanisterCategory {
    Tools,
    NFT,
    Service,
    Token,
    Social,
    Games,
}

impl Default for Fleek {
    fn default() -> Self {
        panic!()
    }
}

#[derive(Deserialize, CandidType, Clone, PartialEq, Debug)]
pub struct CanisterMetadata {
    pub name: String,
    pub description: String,
    pub frontend: Option<String>,
    pub thumbnail: String,
    pub details: Vec<(String, String)>,
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
        metadata: CanisterMetadata,
    ) -> Result<(), Failure> {
        self.0.insert(canister, metadata);
        if !self.0.contains_key(&canister) {
            return Err(Failure::Unknown(String::from("Something unexpected happend. Try again.")));
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

    pub fn get_all(&self) -> Vec<&CanisterMetadata> {
        self.0.values().collect()
    }
}

#[derive(CandidType, Debug, PartialEq)]
pub enum Failure {
    NotAuthorized,
    BadParameters,
    NonExistentCanister,
    Unknown(String),
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
fn add(canister: Principal, metadata: CanisterMetadata) -> Result<(), Failure> {
    if !is_fleek(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    } else if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.thumbnail)
        || (metadata.frontend.is_some() && !validate_url(&metadata.frontend.clone().unwrap()))
    {
        return Err(Failure::BadParameters);
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(canister, metadata)
}

#[update]
fn remove(canister: Principal) -> Result<(), Failure> {
    if !is_fleek(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    }
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.remove_canister(&canister)
}

#[query]
fn get_all() -> Vec<&'static CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_all()
}

/* #[update]
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
} */

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

        let canister_info = CanisterMetadata {
            name: String::from("XTC"),
            description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
            frontend: Some(String::from("https://frontend_url.com")),
            thumbnail: String::from("https://logo_url.com"),
            details: vec![(String::from("category"), String::from("service"))]
        };

        let addition = add(mock_principals::xtc(), canister_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        // Bob is not an admin so the operation should not be successful
        ctx.update_caller(mock_principals::bob());
        let addition = add(mock_principals::xtc(), canister_info.clone());
        assert!(addition.is_err());

        // Alice makes Bob an admin and now he can add/remove canisters
        ctx.update_caller(mock_principals::alice());
        let operation = set_admin(mock_principals::bob());
        assert!(operation.is_ok());

        ctx.update_caller(mock_principals::bob());
        let addition = add(mock_principals::xtc(), canister_info);
        assert!(addition.is_ok());
    }

    #[test]
    fn get_information() {
        // Alice is an admin
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let xtc_info = CanisterMetadata {
            name: String::from("XTC"),
            description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
            frontend: Some(String::from("https://frontend_url.com")),
            thumbnail: String::from("https://logo_url.com"),
            details: vec![(String::from("category"), String::from("service"))]
        };

        let addition = add(mock_principals::xtc(), xtc_info.clone());
        assert!(addition.is_ok());

        let operation_get_info = get_info(vec![mock_principals::xtc()]);
        let expected_response: Vec<Option<&CanisterMetadata>> = vec![Some(&xtc_info)];
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

        let nft_info = CanisterMetadata {
            name: String::from("NFT Registry"),
            description: String::from("DAB's NFT registry provides its users with information for every nft canister in the registry."),
            frontend: Some(String::from("https://frontend_url.com")),
            thumbnail: String::from("https://logo_url.com"),
            details: vec![(String::from("category"), String::from("service"))]
        };

        let addition = add(nft_registry(), nft_info.clone());
        assert!(addition.is_ok());

        // Now Bob should be able to ask for both xtc and nft registry canister
        ctx.update_caller(mock_principals::bob());
        let operation_get_info = get_info(vec![mock_principals::xtc(), nft_registry()]);
        let expected_response: Vec<Option<&CanisterMetadata>> =
            vec![Some(&xtc_info), Some(&nft_info)];
        assert_eq!(operation_get_info, expected_response);
    }

    #[test]
    fn remove_test() {
        // Alice is an admin
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let xtc_info = CanisterMetadata {
                name: String::from("XTC"),
                description: String::from("XTC is one of Dank's products which allows its users manage their canisters and cycles."),
                frontend: Some(String::from("https://frontend_url.com")),
                thumbnail: String::from("https://logo_url.com"),
                details: vec![(String::from("category"), String::from("service"))]
            };

        let addition = add(mock_principals::xtc(), xtc_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        // the canister should return an error if we try to remove a non-existent canister
        let remove_operation = remove(mock_principals::xtc());
        assert_eq!(
            remove_operation.err().unwrap(),
            Failure::NonExistentCanister
        );

        // Bob should not be able to remove a canister because he is not an admin
        ctx.update_caller(mock_principals::bob());
        let remove_operation = remove(mock_principals::xtc());
        assert_eq!(remove_operation.err().unwrap(), Failure::NotAuthorized);
    }
}
