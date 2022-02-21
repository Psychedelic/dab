use ic_cdk::export::candid::{CandidType, Principal};
use ic_kit::macros::*;
use ic_kit::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::validate_url;

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum DetailValue {
    True,
    False,
    U64(u64),
    I64(i64),
    Float(f64),
    Text(String),
    Principal(Principal),
    #[serde(with = "serde_bytes")]
    Slice(Vec<u8>),
    Vec(Vec<DetailValue>),
}

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
    pub name: String,
    pub description: String,
    pub thumbnail: String,
    pub frontend: Option<String>,
    pub principal_id: Principal,
    pub details: Vec<(String, DetailValue)>,
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

    pub fn get_info(&mut self, canister: Principal) -> Option<&CanisterMetadata> {
        self.0.get(&canister)
    }

    pub fn add_canister(&mut self, metadata: CanisterMetadata) -> Result<(), Failure> {
        let id: Principal = metadata.principal_id;
        self.0.insert(metadata.principal_id, metadata);
        if !self.0.contains_key(&id) {
            return Err(Failure::Unknown(String::from(
                "Something unexpected happend. Try again.",
            )));
        }
        Ok(())
    }

    pub fn remove_canister(&mut self, canister: &Principal) -> Result<(), Failure> {
        if !self.0.contains_key(canister) {
            return Err(Failure::NonExistentItem);
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
    NonExistentItem,
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
fn get(canister: Principal) -> Option<&'static CanisterMetadata> {
    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.get_info(canister)
}

#[update]
fn add(metadata: CanisterMetadata) -> Result<(), Failure> {
    if !is_fleek(&ic::caller()) {
        return Err(Failure::NotAuthorized);
    } else if &metadata.name.len() > &NAME_LIMIT
        || &metadata.description.len() > &DESCRIPTION_LIMIT
        || !validate_url(&metadata.thumbnail)
        || !metadata.clone().frontend.map(validate_url).unwrap_or(true)
    {
        return Err(Failure::BadParameters);
    }

    let canister_db = ic::get_mut::<CanisterDB>();
    canister_db.add_canister(metadata)
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
            principal_id: mock_principals::xtc(),
            details: vec![(String::from("category"), DetailValue::Text(String::from("service")))]
        };

        let addition = add(canister_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        // Bob is not an admin so the operation should not be successful
        ctx.update_caller(mock_principals::bob());
        let addition = add(canister_info.clone());
        assert!(addition.is_err());

        // Alice makes Bob an admin and now he can add/remove canisters
        ctx.update_caller(mock_principals::alice());
        let operation = set_admin(mock_principals::bob());
        assert!(operation.is_ok());

        ctx.update_caller(mock_principals::bob());
        let addition = add(canister_info);
        assert!(addition.is_ok());
    }

    #[test]
    fn test_add_canister_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

        let added_canister = get(mock_principals::xtc());
        assert!(added_canister.is_some());
    }

    #[test]
    fn test_add_canister_fails_because_of_bad_params() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("bad url"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);

        let added_canister = get(mock_principals::xtc());
        assert!(added_canister.is_none());
    }

    #[test]
    fn test_add_canister_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        context.update_caller(mock_principals::bob());

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), Failure::NotAuthorized);

        let added_canister = get(mock_principals::xtc());
        assert!(added_canister.is_none());
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
            principal_id: mock_principals::xtc(),
            details: vec![(String::from("category"), DetailValue::Text(String::from("service")))]
        };

        let addition = add(xtc_info.clone());
        assert!(addition.is_ok());

        let operation_get_info = get(mock_principals::xtc());
        let expected_response: Option<&CanisterMetadata> = Some(&xtc_info);
        assert_eq!(operation_get_info.unwrap(), expected_response.unwrap());

        // Users who are not admins should be able to access the information, too
        ctx.update_caller(mock_principals::bob());
        let operation_get_info = get(mock_principals::xtc());
        assert_eq!(operation_get_info.unwrap(), expected_response.unwrap());

        // users should be able to ask for multiple canisters
        // We switch back to alice to add another canister
        ctx.update_caller(mock_principals::alice());

        let nft_info = CanisterMetadata {
            name: String::from("NFT Registry"),
            description: String::from("DAB's NFT registry provides its users with information for every nft canister in the registry."),
            frontend: Some(String::from("https://frontend_url.com")),
            thumbnail: String::from("https://logo_url.com"),
            principal_id: nft_registry(),
            details: vec![(String::from("category"), DetailValue::Text(String::from("service")))]
        };

        let addition = add(nft_info.clone());
        assert!(addition.is_ok());

        // Now Bob should be able to ask for both xtc and nft registry canister
        ctx.update_caller(mock_principals::bob());
        let operation_get_info_xtc = get(mock_principals::xtc());
        let operation_get_info_nft = get(nft_registry());
        let expected_response_xtc: Option<&CanisterMetadata> = Some(&xtc_info);
        let expected_response_nft: Option<&CanisterMetadata> = Some(&nft_info);
        assert_eq!(operation_get_info_xtc, expected_response_xtc);
        assert_eq!(operation_get_info_nft, expected_response_nft);
    }

    #[test]
    fn test_remove_canister_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        add(canister_metadata.clone());

        let remove_result = remove(mock_principals::xtc());
        assert!(remove_result.is_ok());

        let removed_canister = get(mock_principals::xtc());
        assert!(removed_canister.is_none());
    }

    #[test]
    fn test_remove_canister_fails_because_of_inexistent_canister() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let remove_result = remove(mock_principals::xtc());
        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), Failure::NonExistentItem);
    }

    #[test]
    fn test_remove_canister_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        add(canister_metadata.clone());

        context.update_caller(mock_principals::bob());

        let remove_result = remove(mock_principals::xtc());
        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), Failure::NotAuthorized);
    }

    #[test]
    fn test_get_canister_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        add(canister_metadata);

        let get_response = get(mock_principals::xtc());
        assert!(get_response.is_some());
        assert_eq!(get_response.unwrap().principal_id, mock_principals::xtc());
    }

    fn test_get_canister_for_unauthorized_caller_successfully() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        add(canister_metadata);

        context.update_caller(mock_principals::bob());

        let get_response = get(mock_principals::xtc());
        assert!(get_response.is_some());
        assert_eq!(get_response.unwrap().principal_id, mock_principals::xtc());
    }

    #[test]
    fn test_get_canister_returns_none_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let get_response = get(mock_principals::xtc());
        assert!(get_response.is_none());
    }

    #[test]
    fn test_get_all_canisters_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        add(canister_metadata.clone());

        let get_all_response = get_all();

        assert_eq!(get_all_response.len(), 1);
    }

    #[test]
    fn test_get_all_canisters_for_unauthorized_caller_succesfully() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: None,
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        add(canister_metadata.clone());

        context.update_caller(mock_principals::bob());

        let get_all_response = get_all();

        assert_eq!(get_all_response.len(), 1);
    }

    #[test]
    fn test_get_all_canisters_returns_none_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let get_all_response = get_all();
        assert_eq!(get_all_response.len(), 0);
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
                principal_id: mock_principals::xtc(),
                details: vec![(String::from("category"), DetailValue::Text(String::from("service")))]
            };

        let addition = add(xtc_info.clone());
        assert!(addition.is_ok());

        let remove_operation = remove(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        // the canister should return an error if we try to remove a non-existent canister
        let remove_operation = remove(mock_principals::xtc());
        assert_eq!(remove_operation.err().unwrap(), Failure::NonExistentItem);

        // Bob should not be able to remove a canister because he is not an admin
        ctx.update_caller(mock_principals::bob());
        let remove_operation = remove(mock_principals::xtc());
        assert_eq!(remove_operation.err().unwrap(), Failure::NotAuthorized);
    }
}
