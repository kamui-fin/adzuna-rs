#[cfg(test)]
mod tests {
    use std::env;

    use adzuna_rs::client::Client;
    use adzuna_rs::request::RequestBuilder;

    fn get_client() -> Client {
        Client::new(env::var("API_ID").unwrap(), env::var("API_KEY").unwrap())
    }

    #[tokio::test]
    async fn it_is_version_1() {
        let client = get_client();
        let ver = client.api_version().fetch().await.unwrap();
        assert_eq!(ver.api_version, 1);
    }

    #[tokio::test]
    async fn it_fetches_categories() {
        let client = get_client();
        let categories = client.categories().fetch().await.unwrap();
        assert!(!categories.results.is_empty())
    }

    #[tokio::test]
    async fn it_searches_swe_jobs() {
        let client = get_client();
        let jobs = client
            .search()
            .what("software engineer")
            .fetch()
            .await
            .unwrap();
        assert!(jobs.count != 0)
    }

    #[tokio::test]
    async fn it_limits_search_results() {
        let client = get_client();
        let per_page = 7;
        let jobs = client
            .search()
            .what("frontend")
            .results_per_page(per_page)
            .fetch()
            .await
            .unwrap();
        assert_eq!(jobs.results.len(), per_page)
    }

    #[tokio::test]
    async fn it_searches_with_multiple_locations() {
        let client = get_client();
        let jobs = client
            .search()
            .what("backend")
            .place("austin")
            .results_per_page(1)
            .full_time()
            .fetch()
            .await
            .unwrap();

        let job = &jobs.results[0];
        let area = &job.location.area.as_ref().unwrap();
        assert!(area.contains(&"Austin".to_string()))
    }
}
