#[cfg(test)]
mod tests {
    use ic_kit::mock_principals;
    use ic_kit::MockContext;
    use tokio::*;

    use crate::common_types::*;
    use crate::management::*;
    use crate::nft::*;

    #[tokio::test]
    async fn test_controller() {
        // alice is the controller
        let ctx = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_info = NftCanister {
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

        let mut addition = add(canister_info.clone());
        assert!(addition.await.is_ok());

        let remove_operation = remove(mock_principals::xtc());
        assert!(remove_operation.is_ok());

        ctx.update_caller(mock_principals::bob());
        addition = add(canister_info);
        assert!(addition.await.is_err());
    }

    #[tokio::test]
    async fn test_add() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
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

        assert!(add(canister_info).await.is_ok());
    }

    #[tokio::test]
    async fn test_add_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
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

        let add_result = add(canister_info).await;

        assert!(add_result.clone().is_err());
        assert_eq!(add_result.unwrap_err(), OperationError::NotAuthorized);
    }

    #[tokio::test]
    async fn test_add_fails_because_of_bad_name_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: std::iter::repeat("X")
                .take(NAME_LIMIT + 1)
                .collect::<String>(),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: Some(String::from("https://frontend_url.com")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info).await;

        assert!(addition_result.clone().is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[tokio::test]
    async fn test_add_fails_because_of_bad_descripion_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: std::iter::repeat("X")
                .take(DESCRIPTION_LIMIT + 1)
                .collect::<String>(),
            thumbnail: String::from("https://logo_url.com"),
            frontend: Some(String::from("https://frontend_url.com")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info).await;

        assert!(addition_result.clone().is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[tokio::test]
    async fn test_add_fails_because_of_bad_thumbnail_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("bad thumbnail"),
            frontend: Some(String::from("https://frontend_url.com")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info).await;

        assert!(addition_result.clone().is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[tokio::test]
    async fn test_add_fails_because_of_bad_frontend_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: Some(String::from("bad frontend")),
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        let addition_result = add(canister_info).await;

        assert!(addition_result.clone().is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[tokio::test]
    async fn test_add_fails_because_of_bad_standard_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: None,
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standards"),
                DetailValue::Text(String::from("bad standard")),
            )],
        };

        let addition_result = add(canister_info).await;

        assert!(addition_result.clone().is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[tokio::test]
    async fn test_add_fails_because_of_invalid_details_params_amount() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: None,
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP721")),
                ),
                (
                    String::from("extra field"),
                    DetailValue::Text(String::from("invalid field")),
                ),
            ],
        };

        let addition_result = add(canister_info).await;

        assert!(addition_result.clone().is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters);
    }

    #[tokio::test]
    async fn test_remove() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
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

        assert!(add(canister_info).await.is_ok());

        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[tokio::test]
    async fn test_remove_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
            name: String::from("name"),
            description: String::from("description"),
            thumbnail: String::from("https://logo_url.com"),
            frontend: None,
            principal_id: mock_principals::xtc(),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("DIP721")),
            )],
        };

        add(canister_info.clone());

        context.update_caller(mock_principals::bob());

        let remove_result = remove(canister_info.clone().principal_id);

        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), OperationError::NotAuthorized);
    }

    #[tokio::test]
    async fn test_remove_fails_because_of_non_existent_canister() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let remove_result = remove(mock_principals::xtc());

        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), OperationError::NonExistentItem);
    }

    #[tokio::test]
    async fn test_get() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
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

        assert!(add(canister_info.clone()).await.is_ok());

        assert_eq!(
            get(mock_principals::xtc()).unwrap().name,
            canister_info.name
        );
        assert!(get(mock_principals::alice()).is_none());
    }

    #[tokio::test]
    async fn test_get_returns_none_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let get_result = get(mock_principals::xtc());
        assert!(get_result.is_none());
    }

    #[tokio::test]
    async fn test_get_all() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let canister_info = NftCanister {
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

        assert!(add(canister_info.clone()).await.is_ok());
        assert_eq!(get_all()[0].name, canister_info.name);
    }

    #[tokio::test]
    async fn test_get_all_returns_no_canisters_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .with_data(Admins(vec![mock_principals::alice()]))
            .inject();

        let get_all_result = get_all();

        assert_eq!(get_all_result.len(), 0);
    }
}
