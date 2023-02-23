use crate::request::*;

/// The main client of the wrapper to access API routes.
pub struct Client {
    pub app_id: String,
    pub app_key: String,
    pub req_client: reqwest::Client,
}

impl Client {
    /// Create a new client with API credentials.
    pub fn new(app_id: String, app_key: String) -> Self {
        let req_client = reqwest::Client::new();
        Self {
            app_id,
            app_key,
            req_client,
        }
    }

    /// Return the current version of this API
    pub fn api_version(&self) -> VersionRequest {
        VersionRequest::new(self)
    }

    /// List available job categories
    pub fn categories(&self) -> CategoriesRequest {
        CategoriesRequest::new(self)
    }

    /// Provides historical average salary data
    pub fn history(&self) -> HistoryRequest {
        HistoryRequest::new(self)
    }

    /// Provides salary data for locations inside an area
    pub fn geodata(&self) -> GeodataRequest {
        GeodataRequest::new(self)
    }

    /// List the top employers for the search terms supplied
    pub fn top_companies(&self) -> TopCompaniesRequest {
        TopCompaniesRequest::new(self)
    }

    /// Provide histogram data of salary data
    pub fn histogram(&self) -> HistogramRequest {
        HistogramRequest::new(self)
    }

    /// Search the Adzuna job database
    pub fn search(&self) -> SearchRequest {
        SearchRequest::new(self)
    }
}
