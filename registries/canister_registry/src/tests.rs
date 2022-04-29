#[cfg(test)]
mod tests {
    use crate::common_types::*;
    use crate::management::add_admin;
    use crate::registry::{add, get, get_all, init, remove};
    use ic_kit::*;

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
        let operation = add_admin(mock_principals::bob());
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
    fn test_add_canister_with_frontend_field_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: Some(String::from("https://google.com")),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

        let added_canister = get(mock_principals::xtc());
        assert!(added_canister.is_some());
        assert_eq!(
            added_canister.unwrap().frontend,
            Some(String::from("https://google.com"))
        );
    }

    #[test]
    fn test_add_canister_fails_because_of_thumbnail_param() {
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
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters(String::from("Thumbnail field has to be a url.")));

        let added_canister = get(mock_principals::xtc());
        assert!(added_canister.is_none());
    }

    #[test]
    fn test_add_canister_fails_because_of_bad_frontend_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let canister_metadata = CanisterMetadata {
            name: String::from("xtc"),
            principal_id: mock_principals::xtc(),
            description: String::from("XTC is your cycles wallet."),
            thumbnail: String::from("https://google.com"),
            frontend: Some(String::from("bad url")),
            details: vec![(
                String::from("standard"),
                DetailValue::Text(String::from("Dank")),
            )],
        };

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), OperationError::BadParameters(String::from(
            "Frontend field has to be a url.",
        )));

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
        assert_eq!(addition_result.unwrap_err(), OperationError::NotAuthorized);

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

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

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
        assert_eq!(remove_result.unwrap_err(), OperationError::NonExistentItem);
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

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

        context.update_caller(mock_principals::bob());

        let remove_result = remove(mock_principals::xtc());
        assert!(remove_result.is_err());
        assert_eq!(remove_result.unwrap_err(), OperationError::NotAuthorized);
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

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

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

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

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

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

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

        let addition_result = add(canister_metadata.clone());
        assert!(addition_result.is_ok());

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
        assert_eq!(
            remove_operation.err().unwrap(),
            OperationError::NonExistentItem
        );

        // Bob should not be able to remove a canister because he is not an admin
        ctx.update_caller(mock_principals::bob());
        let remove_operation = remove(mock_principals::xtc());
        assert_eq!(
            remove_operation.err().unwrap(),
            OperationError::NotAuthorized
        );
    }
}
