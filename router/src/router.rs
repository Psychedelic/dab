use crate::common_types::*;
use crate::management::*;

use ic_kit::candid::Principal;
use ic_kit::macros::*;
use ic_kit::*;
use std::collections::HashMap;
use validator::validate_url;

#[derive(Default)]
pub struct Registries(HashMap<Principal, Registry>);

impl Registries {
    pub fn archive(&mut self) -> Vec<(Principal, Registry)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, Registry)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(
        &mut self,
        registry_info: Registry,
    ) -> Result<(), OperationError> {
        self.0.insert(registry_info.principal_id, registry_info);
        Ok(())
    }

    pub fn remove(&mut self, principal_id: &Principal) -> Result<(), OperationError> {
        if self.0.contains_key(principal_id) {
            self.0.remove(principal_id);
            return Ok(());
        }

        Err(OperationError::NonExistentItem)
    }

    pub fn get(&self, principal_id: &Principal) -> Option<&Registry> {
        self.0.get(principal_id)
    }

    pub fn get_all(&self) -> Vec<&Registry> {
        self.0.values().collect()
    }
}

#[query]
fn name() -> String {
    String::from("Router Canister")
}

#[update]
fn add(registry_info: Registry) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    if !validate_url(&registry_info.thumbnail) {
        return Err(OperationError::BadParameters);
    }

    let name = registry_info.name.clone();
    if name.len() <= 120
        && &registry_info.description.len() <= &1200
        && registry_info.details.len() == 0
    {
        let db = ic::get_mut::<Registries>();
        return db.add(registry_info);
    }

    Err(OperationError::BadParameters)
}

#[update]
fn remove(principal_id: Principal) -> Result<(), OperationError> {
    if !is_admin(&ic::caller()) {
        return Err(OperationError::NotAuthorized);
    }

    let db = ic::get_mut::<Registries>();
    db.remove(&principal_id)
}

#[query]
fn get(principal_id: Principal) -> Option<&'static Registry> {
    let db = ic::get_mut::<Registries>();
    db.get(&principal_id)
}

#[query]
fn get_all() -> Vec<&'static Registry> {
    let db = ic::get_mut::<Registries>();
    db.get_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_registry_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        assert!(add(registry_info).is_ok());
    }

    #[test]
    fn test_add_registry_fails_because_of_bad_params() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("bad thumbnail :("),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        assert!(add(registry_info).is_err());
    }

    #[test]
    fn test_add_registry_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        context.update_caller(mock_principals::bob());

        assert!(add(registry_info).is_err());
    }

    #[test]
    fn test_remove_registry_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        assert!(add(registry_info).is_ok());

        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[test]
    fn test_remove_token_fails_because_of_unathorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        assert!(add(registry_info).is_ok());

        context.update_caller(mock_principals::bob());

        assert!(remove(mock_principals::xtc()).is_err());
    }

    #[test]
    fn test_get_all_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        assert!(add(registry_info).is_ok());

        let registries = get_all();

        assert_eq!(registries.len(), 1);
    }

    #[test]
    fn test_get_all_returns_none_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registries = get_all();

        assert_eq!(registries.len(), 0);
    }

    #[test]
    fn test_get_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry_info = Registry {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![],
        };

        assert!(add(registry_info).is_ok());

        let registry = get(mock_principals::xtc());

        assert!(registry.is_some());
    }

    #[test]
    fn test_get_returns_none_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let registry = get(mock_principals::xtc());

        assert!(registry.is_none());
    }
}

