use crate::client::Client;
use crate::models::*;
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const ROOT_URL: &str = "https://api.adzuna.com/v1/api";

#[async_trait]
trait AdzunaRequestBuilder {
    type Response: DeserializeOwned + std::fmt::Debug;

    fn get_request_url(&self) -> String;
    fn get_client(&self) -> &Client;
    fn get_parameters(&self) -> &Parameters;

    async fn fetch(&self) -> Result<Self::Response, StatusCode> {
        let url = format!("{}{}", ROOT_URL, self.get_request_url());
        let auth_params: Vec<(String, &String)> = vec![
            ("app_id".into(), &self.get_client().app_id),
            ("app_key".into(), &self.get_client().app_key),
        ];

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .query(&auth_params)
            .query(&self.get_parameters())
            .send()
            .await;

        match &response {
            Ok(r) => {
                if r.status() != StatusCode::OK {
                    println!("{r:#?}");
                    return Err(r.status());
                }
            }
            Err(e) => {
                println!("{e:#?}");
                if e.is_status() {
                    return Err(e.status().unwrap());
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }

        let response = response.unwrap().json::<Self::Response>().await;
        match response {
            Ok(response) => {
                println!("{response:#?}");
                Ok(response)
            }
            Err(e) => {
                println!("{e:#?}");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}

macro_rules! create_endpoint {
    ($name: ident) => {
        pub struct $name<'a> {
            client: &'a Client,
            parameters: Parameters,
            search_country: &'static str,
            search_page: usize,
        }
        impl<'a> $name<'a> {
            pub fn new(client: &'a Client) -> Self {
                Self {
                    client,
                    parameters: Default::default(),
                    search_country: Country::UnitedStates.to_code(),
                    search_page: 1,
                }
            }
        }
    };
}

create_endpoint!(VersionRequest);

impl AdzunaRequestBuilder for VersionRequest<'_> {
    type Response = Version;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        "/version".into()
    }
}

create_endpoint!(CategoriesRequest);

impl AdzunaRequestBuilder for CategoriesRequest<'_> {
    type Response = Categories;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/jobs/{}/categories", self.search_country)
    }
}

impl CategoriesRequest<'_> {
    /// Filter with a country of interest.
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = country.to_code();
        self
    }
}

create_endpoint!(HistogramRequest);

impl AdzunaRequestBuilder for HistogramRequest<'_> {
    type Response = SalaryHistogram;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/jobs/{}/histogram", self.search_country)
    }
}

impl HistogramRequest<'_> {
    /// Filter by keywords. Multiple terms may be space separated.
    pub fn what(mut self, what: &str) -> Self {
        self.parameters.what = Some(what.into());
        self
    }
    /// Filter with a country of interest.
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = country.to_code();
        self
    }

    /// Filter by a location, in a similar form to that returned in a LocationDetail object.
    pub fn location(mut self, location: &str) -> Self {
        if self.parameters.locations.len() < 8 {
            self.parameters.locations.push(location.to_string());
        }
        self
    }

    /// Filter with a category tag, as returned by the "category" endpoint.
    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }
}

create_endpoint!(HistoryRequest);

impl AdzunaRequestBuilder for HistoryRequest<'_> {
    type Response = HistoricalSalary;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/jobs/{}/history", self.search_country)
    }
}

impl HistoryRequest<'_> {
    /// Set the number of months back for which to retrieve data.
    pub fn months(mut self, months: usize) -> Self {
        self.parameters.months = Some(months);
        self
    }
    /// Filter with a country of interest.
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = country.to_code();
        self
    }

    /// Filter by a location, in a similar form to that returned in a LocationDetail object.
    pub fn location(mut self, location: &str) -> Self {
        if self.parameters.locations.len() < 8 {
            self.parameters.locations.push(location.to_string());
        }
        self
    }

    /// Filter with a category tag, as returned by the "category" endpoint.
    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }
}

create_endpoint!(TopCompaniesRequest);

impl AdzunaRequestBuilder for TopCompaniesRequest<'_> {
    type Response = TopCompanies;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/jobs/{}/top_companies", self.search_country)
    }
}

impl TopCompaniesRequest<'_> {
    /// Filter by keywords. Multiple terms may be space separated.
    pub fn what(mut self, what: &str) -> Self {
        self.parameters.what = Some(what.into());
        self
    }
    /// Filter with a country of interest.
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = country.to_code();
        self
    }

    /// Filter by a location, in a similar form to that returned in a LocationDetail object.
    pub fn location(mut self, location: &str) -> Self {
        if self.parameters.locations.len() < 8 {
            self.parameters.locations.push(location.to_string());
        }
        self
    }

    /// Filter with a category tag, as returned by the "category" endpoint.
    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }
}

create_endpoint!(GeodataRequest);

impl AdzunaRequestBuilder for GeodataRequest<'_> {
    type Response = JobGeoData;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/jobs/{}/geodata", self.search_country)
    }
}

impl GeodataRequest<'_> {
    /// Filter with a country of interest.
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = country.to_code();
        self
    }

    /// Filter by a location, in a similar form to that returned in a LocationDetail object.
    pub fn location(mut self, location: &str) -> Self {
        if self.parameters.locations.len() < 8 {
            self.parameters.locations.push(location.to_string());
        }
        self
    }

    /// Filter with a category tag, as returned by the "category" endpoint.
    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }
}

create_endpoint!(SearchRequest);

impl AdzunaRequestBuilder for SearchRequest<'_> {
    type Response = JobSearchResults;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/jobs/{}/search/{}", self.search_country, self.search_page)
    }
}

impl SearchRequest<'_> {
    /// Filter with a country of interest.
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = country.to_code();
        self
    }

    /// Filter by a location, in a similar form to that returned in a LocationDetail object.
    pub fn location(mut self, location: &str) -> Self {
        if self.parameters.locations.len() < 8 {
            self.parameters.locations.push(location.to_string());
        }
        self
    }

    /// Filter with a category tag, as returned by the "category" endpoint.
    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }

    /// Set the page for search results.
    pub fn page(mut self, page: usize) -> Self {
        if page > 0 {
            self.search_page = page;
        }
        self
    }

    /// Filter by keywords. Multiple terms may be space separated.
    pub fn what(mut self, what: &str) -> Self {
        self.parameters.what = Some(what.into());
        self
    }

    /// Filter by keywords. All keywords must be found.
    pub fn what_and(mut self, what_and: &str) -> Self {
        self.parameters.what_and = Some(what_and.into());
        self
    }

    /// Filter by an entire phrase which must be found in the description or title.
    pub fn what_phrase(mut self, what_phrase: &str) -> Self {
        self.parameters.what_phrase = Some(what_phrase.into());
        self
    }

    /// Filter by keywords. Any keywords may be found.
    pub fn what_or(mut self, what_or: &str) -> Self {
        self.parameters.what_or = Some(what_or.into());
        self
    }

    /// Filter out jobs with certain keywords.
    pub fn what_exclude(mut self, what_exclude: &str) -> Self {
        self.parameters.what_exclude = Some(what_exclude.into());
        self
    }

    /// Filter by the geographic center. Place names, postal codes, etc. may be used.
    pub fn place(mut self, r#where: &str) -> Self {
        self.parameters.r#where = Some(r#where.into());
        self
    }

    /// Filter by keywords. Only searches the title.
    pub fn title_only(mut self, title_only: &str) -> Self {
        self.parameters.title_only = Some(title_only.into());
        self
    }

    /// Include jobs with unknown salaries.
    pub fn salary_include_unknown(mut self) -> Self {
        self.parameters.salary_include_unknown = Some("1".into());
        self
    }

    /// Only full time jobs will be returned.
    pub fn full_time(mut self) -> Self {
        self.parameters.full_time = Some("1".into());
        self
    }

    /// Only part time jobs will be returned.
    pub fn part_time(mut self) -> Self {
        self.parameters.part_time = Some("1".into());
        self
    }

    /// Only contract jobs will be returned.
    pub fn contract(mut self) -> Self {
        self.parameters.contract = Some("1".into());
        self
    }

    /// Only permanent jobs will be returned.
    pub fn permanent(mut self) -> Self {
        self.parameters.permanent = Some("1".into());
        self
    }

    /// Filter by the canonical company name.
    /// This may be contained in a Company object when a job is returned.
    /// A full list of allowed terms in not available through the API.
    pub fn company(mut self, company: &str) -> Self {
        self.parameters.company = Some(company.into());
        self
    }

    /// Filter using a distance in kilometers from the centre of the place described by the 'where' parameter. Defaults to 5km.
    pub fn distance(mut self, distance: usize) -> Self {
        self.parameters.distance = Some(distance);
        self
    }

    /// Set a number of results to include on a page of search results.
    pub fn results_per_page(mut self, results_per_page: usize) -> Self {
        if results_per_page > 0 {
            self.parameters.results_per_page = Some(results_per_page);
        }
        self
    }

    /// Set an upper bound to the age of the oldest advertisment in days that will be returned.
    pub fn max_days_old(mut self, max_days_old: usize) -> Self {
        self.parameters.max_days_old = Some(max_days_old);
        self
    }

    /// Set a minimum salary we wish to get results for.
    pub fn salary_min(mut self, distance: usize) -> Self {
        self.parameters.salary_min = Some(distance);
        self
    }

    /// Set a maximum salary we wish to get results for.
    pub fn salary_max(mut self, salary_max: usize) -> Self {
        self.parameters.salary_max = Some(salary_max);
        self
    }

    /// Specify the ordering of search results.
    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.parameters.sort_by = Some(sort_by.to_string());
        self
    }

    /// Specify a direction to order the search results.
    pub fn sort_dir(mut self, sort_dir: SortDirection) -> Self {
        self.parameters.sort_dir = Some(sort_dir.to_string());
        self
    }
}