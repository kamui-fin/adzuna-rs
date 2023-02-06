#[cfg(test)]
mod tests {
    use std::env;

    use adzuna::{Client, RequestBuilder};

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
    async fn it_fetches_histogram() {
        let client = get_client();
        let histogram = client.histogram().what("photoshop").fetch().await.unwrap();
        assert!(histogram.histogram.is_some());
        let histogram = histogram.histogram.unwrap();
        assert!(!histogram.is_empty());

        // make sure results are consistent for different keywords
        let histogram = client.histogram().what("excel").fetch().await.unwrap();
        assert!(histogram.histogram.is_some());
        let histogram = histogram.histogram.unwrap();
        assert!(!histogram.is_empty());
    }

    #[tokio::test]
    async fn it_fetches_top_companies() {
        let client = get_client();
        let companies = client
            .top_companies()
            .what("frontend")
            .fetch()
            .await
            .unwrap();
        assert!(companies.leaderboard.is_some());
        let companies = companies.leaderboard.unwrap();
        assert!(!companies.is_empty());
    }

    #[tokio::test]
    async fn it_fetches_geodata() {
        let client = get_client();
        let geodata = client.geodata().fetch().await.unwrap();
        assert!(geodata.locations.is_some());
        let geodata = geodata.locations.unwrap();
        assert!(!geodata.is_empty());
    }

    #[tokio::test]
    async fn it_fetches_history() {
        let client = get_client();
        let history = client.history().fetch().await.unwrap();
        assert!(history.month.is_some());
        let history = history.month.unwrap();
        assert!(!history.is_empty());
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
    async fn it_searches_with_place() {
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

    #[tokio::test]
    async fn it_fails_to_authorize() {
        let client = Client::new("fake".into(), "fake".into());
        let jobs = client.search().what("engineer").fetch().await;
        println!("{jobs:#?}");
        assert!(jobs.is_err());
        let error = jobs.unwrap_err();
        assert!(error.api_error.is_some());
        assert_eq!(error.http_status, 401);
    }

    #[tokio::test]
    async fn it_fails_with_invalid_category() {
        let client = get_client();
        let companies = client
            .top_companies()
            .what("frontend")
            .category("invalid")
            .fetch()
            .await;
        assert!(companies.is_err());
        assert_eq!(companies.unwrap_err().http_status, 400);
    }

    #[tokio::test]
    async fn it_fails_with_invalid_location() {
        let client = get_client();
        let companies = client
            .top_companies()
            .what("frontend")
            .location("somewhere")
            .fetch()
            .await;
        assert!(companies.is_err());
        assert_eq!(companies.unwrap_err().http_status, 400);
    }
}
