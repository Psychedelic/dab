/*#[cfg(test)]
mod tests {
    use ic_kit::{mock_principals, MockContext};
    use tokio::*;

    use crate::common_types::*;
    use crate::tokens::*;

    #[tokio::test]
    async fn test_add_token_successfuly() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).await.is_ok());
    }

    #[tokio::test]
    async fn test_add_token_fails_because_of_bad_params() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).await.is_err());
    }

    #[tokio::test]
    async fn test_add_token_fails_because_of_unauthorized_caller() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        context.update_caller(mock_principals::bob());

        assert!(add(token_info).await.is_err());
    }

    #[tokio::test]
    async fn test_remove_token_successfuly() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).await.is_ok());

        assert!(remove(mock_principals::xtc()).is_ok());
    }

    #[tokio::test]
    async fn test_remove_token_fails_because_of_unathorized_caller() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).await.is_ok());

        context.update_caller(mock_principals::bob());

        assert!(remove(mock_principals::xtc()).is_err());
    }

    #[tokio::test]
    async fn test_get_all_successfuly() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).await.is_ok());

        let tokens = get_all();

        assert_eq!(tokens.len(), 1);
    }

    #[tokio::test]
    async fn test_get_all_returns_none_successfuly() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let tokens = get_all();

        assert_eq!(tokens.len(), 0);
    }

    #[tokio::test]
    async fn test_get_succesfully() {
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
                (String::from("total_supply"), DetailValue::U64(1000)),
                (String::from("verified"), DetailValue::True),
            ],
        };

        assert!(add(token_info).await.is_ok());

        let token = get(mock_principals::xtc());

        assert!(token.is_some());
    }

    #[tokio::test]
    async fn test_get_returns_none_succesfully() {
        MockContext::new()
            .with_caller(mock_principals::alice())
            .inject();

        init();

        let token = get(mock_principals::xtc());

        assert!(token.is_none());
    }
}
*/