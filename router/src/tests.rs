use crate::router::*;
use crate::common_types::*;
use crate::management::*;

use ic_kit::*;

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