#[cfg(test)]
mod tests {
    use ic_kit::{MockContext, mock_principals};

    use crate::common_types::*;
    use crate::tokens::*;
    
    #[test]
    fn test_add_token_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());
    }

    #[test]
    fn test_add_token_fails_because_of_bad_params() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("bad logo url"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_err());
    }

    #[test]
    fn test_add_token_fails_because_of_unauthorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        context.update_caller(mock_principals::bob());

        assert!(add(token_info).is_err());
    }

    #[test]
    fn test_remove_token_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());

        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[test]
    fn test_remove_token_fails_because_of_unathorized_caller() {
        let context = MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());

        context.update_caller(mock_principals::bob());

        assert!(remove(mock_principals::xtc()).is_err());
    }

    #[test]
    fn test_get_all_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());

        let tokens = get_all();

        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn test_get_all_returns_none_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let tokens = get_all();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_get_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token_info = Token {
            name: String::from("Wrapped ICP"),
            description: String::from("Wrapped IPC description"),
            thumbnail: String::from("https://logo.com"),
            frontend: Some(String::from("https://website.com")),
            principal_id: mock_principals::xtc(),
            details: vec![
                (
                    String::from("symbol"),
                    DetailValue::Text(String::from("WICP")),
                ),
                (
                    String::from("standard"),
                    DetailValue::Text(String::from("DIP20")),
                ),
                (
                    String::from("total_supply"),
                    DetailValue::U64(1000),
                ),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).is_ok());

        let token = get(mock_principals::xtc());

        assert!(token.is_some());
    }

    #[test]
    fn test_get_returns_none_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token = get(mock_principals::xtc());

        assert!(token.is_none());
    }
}
