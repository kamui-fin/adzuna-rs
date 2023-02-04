use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::models::*;
use std::marker::PhantomData;

const ROOT_URL: &str = "https://api.adzuna.com/v1/api";

pub struct RequestBuilder<'a, T> {
    client: &'a Client,
    endpoint: &'static str,
    search_country: Option<&'static str>,
    search_page: Option<usize>,
    parameters: Parameters,
    __phantom: PhantomData<T>,
}

pub struct Client {
    app_id: String,
    app_key: String,
}

impl Client {
    pub fn new(app_id: String, app_key: String) -> Self {
        Self { app_id, app_key }
    }

    pub fn query<T>(&self, endpoint: &'static str) -> RequestBuilder<T> {
        RequestBuilder::<T>::new(self, endpoint)
    }

    pub fn api_version(&self) -> RequestBuilder<Version> {
        self.query("version")
    }

    pub fn categories(&self) -> RequestBuilder<Categories> {
        self.query("categories").country(Country::UnitedStates) // default country
    }

    pub fn history(&self) -> RequestBuilder<HistoricalSalary> {
        self.query("history").country(Country::UnitedStates)
    }

    pub fn geodata(&self) -> RequestBuilder<JobGeoData> {
        self.query("geodata").country(Country::UnitedStates)
    }

    pub fn top_companies(&self) -> RequestBuilder<TopCompanies> {
        self.query("top_companies").country(Country::UnitedStates)
    }

    pub fn histogram(&self) -> RequestBuilder<SalaryHistogram> {
        self.query("histogram").country(Country::UnitedStates)
    }

    pub fn search(&self) -> RequestBuilder<JobSearchResults> {
        self.query("search").country(Country::UnitedStates)
    }
}

impl<'a, T> RequestBuilder<'a, T> {
    pub fn new(client: &'a Client, endpoint: &'static str) -> Self {
        Self {
            client,
            endpoint,
            search_country: None,
            search_page: Some(1),
            parameters: Default::default(),
            __phantom: PhantomData,
        }
    }

    // both of these are shared between all routes except version
    pub fn location(mut self, location: &str) -> Self {
        if self.parameters.location0.is_none() {
            self.parameters.location0 = Some(location.to_string());
        } else if self.parameters.location1.is_none() {
            self.parameters.location1 = Some(location.to_string());
        } else if self.parameters.location2.is_none() {
            self.parameters.location2 = Some(location.to_string());
        } else if self.parameters.location3.is_none() {
            self.parameters.location3 = Some(location.to_string());
        } else if self.parameters.location4.is_none() {
            self.parameters.location4 = Some(location.to_string());
        } else if self.parameters.location5.is_none() {
            self.parameters.location5 = Some(location.to_string());
        } else if self.parameters.location6.is_none() {
            self.parameters.location6 = Some(location.to_string());
        } else if self.parameters.location7.is_none() {
            self.parameters.location7 = Some(location.to_string());
        } else {
            // location already full
        }
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }

    // path parameters
    pub fn country(mut self, country: Country) -> Self {
        self.search_country = Some(country.to_code());
        self
    }

    pub fn page(mut self, page: usize) -> Self {
        if page > 0 {
            self.search_page = Some(page);
        }
        self
    }

    // used for search, top companies, histogram
    pub fn what(mut self, what: &str) -> Self {
        self.parameters.what = Some(what.into());
        self
    }

    // used for history
    pub fn months(mut self, months: usize) -> Self {
        self.parameters.months = Some(months);
        self
    }

    // rest only for search
    pub fn what_and(mut self, what_and: &str) -> Self {
        self.parameters.what_and = Some(what_and.into());
        self
    }

    pub fn what_phrase(mut self, what_phrase: &str) -> Self {
        self.parameters.what_phrase = Some(what_phrase.into());
        self
    }

    pub fn what_or(mut self, what_or: &str) -> Self {
        self.parameters.what_or = Some(what_or.into());
        self
    }

    pub fn what_exclude(mut self, what_exclude: &str) -> Self {
        self.parameters.what_exclude = Some(what_exclude.into());
        self
    }

    pub fn place(mut self, r#where: &str) -> Self {
        self.parameters.r#where = Some(r#where.into());
        self
    }

    pub fn title_only(mut self, title_only: &str) -> Self {
        self.parameters.title_only = Some(title_only.into());
        self
    }

    pub fn salary_include_unknown(mut self) -> Self {
        self.parameters.salary_include_unknown = Some("1".into());
        self
    }

    pub fn full_time(mut self) -> Self {
        self.parameters.full_time = Some("1".into());
        self
    }

    pub fn part_time(mut self) -> Self {
        self.parameters.part_time = Some("1".into());
        self
    }

    pub fn contract(mut self) -> Self {
        self.parameters.contract = Some("1".into());
        self
    }

    pub fn permanent(mut self) -> Self {
        self.parameters.permanent = Some("1".into());
        self
    }

    pub fn company(mut self, company: &str) -> Self {
        self.parameters.company = Some(company.into());
        self
    }

    pub fn distance(mut self, distance: usize) -> Self {
        self.parameters.distance = Some(distance);
        self
    }

    pub fn results_per_page(mut self, results_per_page: usize) -> Self {
        if results_per_page > 0 {
            self.parameters.results_per_page = Some(results_per_page);
        }
        self
    }

    pub fn max_days_old(mut self, max_days_old: usize) -> Self {
        self.parameters.max_days_old = Some(max_days_old);
        self
    }

    pub fn salary_min(mut self, distance: usize) -> Self {
        self.parameters.salary_min = Some(distance);
        self
    }

    pub fn salary_max(mut self, salary_max: usize) -> Self {
        self.parameters.salary_max = Some(salary_max);
        self
    }

    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.parameters.sort_by = Some(sort_by.to_string());
        self
    }

    pub fn sort_dir(mut self, sort_dir: SortDirection) -> Self {
        self.parameters.sort_dir = Some(sort_dir.to_string());
        self
    }

    pub async fn fetch(&self) -> Result<T, StatusCode>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let url: String = ROOT_URL.to_string()
            + &match self.search_country {
                Some(country) => {
                    format!("/jobs/{}/{}", country, self.endpoint)
                        + &self
                            .search_page
                            .map_or_else(|| "".to_string(), |page| format!("/{page}"))
                }
                None => format!("/{}", self.endpoint),
            };
        let auth_params: Vec<(String, String)> = vec![
            ("app_id".into(), self.client.app_id.clone()),
            ("app_key".into(), self.client.app_key.clone()),
        ];

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .query(&auth_params)
            .query(&self.parameters)
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

        let response = response.unwrap().json::<T>().await;
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
