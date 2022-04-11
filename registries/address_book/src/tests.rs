/*#[cfg(test)]
mod tests {
    use tokio::*;
    use ic_kit::{MockContext, mock_principals};

    use crate::common_types::*;
    use crate::address_book::*;

    #[tokio::test]
    async fn test_add_address_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        let address_info = Address {
            name: String::from("Bob"),
            description: Some(String::from("Friend")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let addition_result = add(address_info.clone()).await;
        assert!(addition_result.is_ok());

        let addresses = get_all();
        assert_eq!(addresses.len(), 1);
        assert_eq!(
            addresses[0].value,
            address_info.value
        );
    }

    #[tokio::test]
    async fn test_add_address_fails_because_of_long_description_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        let address_info = Address {
            name: String::from("Bob"),
            description: Some(
                std::iter::repeat("X")
                    .take(DESCRIPTION_LIMIT + 1)
                    .collect::<String>(),
            ),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let addition_result = add(address_info.clone()).await;
        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);
    }

    #[tokio::test]
    async fn test_add_address_fails_because_of_long_name_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        let address_info = Address {
            name: std::iter::repeat("X").take(25).collect::<String>(),
            description: Some(String::from("description")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let addition_result = add(address_info.clone()).await;
        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);
    }

    #[tokio::test]
    async fn test_add_address_fails_because_of_bad_emoji_param() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        let address_info = Address {
            name: String::from("Bob"),
            description: Some(String::from("description")),
            emoji: Some(String::from("a")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let addition_result = add(address_info.clone()).await;
        assert!(addition_result.is_err());
        assert_eq!(addition_result.unwrap_err(), Failure::BadParameters);
    }

    #[tokio::test]
    async fn test_remove_address_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        let address_info = Address {
            name: String::from("Bob"),
            description: Some(String::from("Friend")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let addition_result = add(address_info.clone()).await;
        assert!(addition_result.is_ok());

        let removal_result = remove(String::from("Bob"));
        assert!(removal_result.is_ok());

        let get_all_result = get_all();
        assert_eq!(get_all_result.len(), 0);
    }

    #[tokio::test]
    async fn test_users_get_their_own_addresses() {
        let context = MockContext::new().inject();

        let bob_address_info = Address {
            name: String::from("Bob"),
            description: Some(String::from("Friend")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let alice_address_info = Address {
            name: String::from("Alice"),
            description: Some(String::from("Friend")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::alice())
        };

        // Alice adds Bob as her contact
        context.update_caller(mock_principals::alice());
        add(bob_address_info);

        let alice_addresses = get_all();

        assert_eq!(alice_addresses.len(), 1);
        assert_eq!(alice_addresses[0].name, String::from("Bob"));

        // Bob adds Alice as his contact
        context.update_caller(mock_principals::bob());
        add(alice_address_info);

        let bob_addresses = get_all();

        assert_eq!(bob_addresses.len(), 1);
        assert_eq!(bob_addresses[0].name, String::from("Alice"));
    }

    #[tokio::test]
    async fn test_addresses_are_added_alphabetically_successfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        let bob_address_info = Address {
            name: String::from("Bob"),
            description: Some(String::from("Friend")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::bob()),
        };

        let andrew_address_info = Address {
            name: String::from("Andrew"),
            description: Some(String::from("Friend")),
            emoji: Some(String::from("😚")),
            value: AddressType::PrincipalId(mock_principals::alice())
        };

        add(bob_address_info);
        add(andrew_address_info);

        let addresses = get_all();

        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].name, String::from("Andrew"));
        assert_eq!(
            addresses[0].value,
            AddressType::PrincipalId(mock_principals::alice())
        );
        assert_eq!(addresses[1].name, String::from("Bob"));
        assert_eq!(
            addresses[1].value,
            AddressType::PrincipalId(mock_principals::bob())
        );
    }
}*/