#[cfg(test)]
mod tests {
    use std::env;

    use adzuna_rs::client::Client;

    fn get_client() -> Client {
        Client::new(env::var("API_ID").unwrap(), env::var("API_KEY").unwrap())
    }

    #[tokio::test]
    async fn is_version_1() {
        let client = get_client();
        let ver = client.api_version().fetch().await.unwrap();
        assert_eq!(ver.api_version, 1);
    }

    #[tokio::test]
    async fn does_fetch_categories() {
        let client = get_client();
        let categories = client.categories().fetch().await.unwrap();
        assert!(!categories.results.is_empty())
    }
}
