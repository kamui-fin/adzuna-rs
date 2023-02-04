use crate::models::*;
use std::marker::PhantomData;

const ROOT_URL: &str = "https://api.adzuna.com/v1/api";

pub struct RequestBuilder<'a, T> {
    client: &'a Client,
    endpoint: &'static str,
    search_country: Option<&'static str>,
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

    pub fn api_version(&self) -> RequestBuilder<ApiVersion> {
        self.query("version")
    }

    pub fn categories(&self) -> RequestBuilder<CategoriesResponse> {
        self.query("categories").country(Country::UnitedStates) // default country
    }

    pub fn history(&self) -> RequestBuilder<HistoryResponse> {
        self.query("history").country(Country::UnitedStates)
    }

    pub fn geodata(&self) -> RequestBuilder<GeodataResponse> {
        self.query("geodata").country(Country::UnitedStates)
    }

    pub fn top_companies(&self) -> RequestBuilder<TopCompaniesResponse> {
        self.query("top_companies").country(Country::UnitedStates)
    }

    pub fn histogram(&self) -> RequestBuilder<HistogramResponse> {
        self.query("histogram").country(Country::UnitedStates)
    }

    pub fn search(&self) -> RequestBuilder<SearchResponse> {
        self.query("search").country(Country::UnitedStates)
    }
}

impl<'a, T> RequestBuilder<'a, T> {
    pub fn new(client: &'a Client, endpoint: &'static str) -> Self {
        Self {
            client,
            endpoint,
            search_country: None,
            parameters: Default::default(),
            __phantom: PhantomData,
        }
    }

    pub fn what(mut self, what: &str) -> Self {
        self.parameters.what = Some(what.into());
        self
    }

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

    pub fn r#where(mut self, r#where: &str) -> Self {
        self.parameters.r#where = Some(r#where.into());
        self
    }

    pub fn title_only(mut self, title_only: &str) -> Self {
        self.parameters.title_only = Some(title_only.into());
        self
    }

    pub fn full_time(mut self, full_time: &str) -> Self {
        self.parameters.full_time = Some(full_time.into());
        self
    }

    pub fn part_time(mut self, part_time: &str) -> Self {
        self.parameters.part_time = Some(part_time.into());
        self
    }

    pub fn contract(mut self, contract: &str) -> Self {
        self.parameters.contract = Some(contract.into());
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
        self.parameters.results_per_page = Some(results_per_page);
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

    pub fn permanent(mut self, permanent: &str) -> Self {
        self.parameters.permanent = Some(permanent.into());
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

    pub fn salary_include_unknown(mut self, salary_include_unknown: &str) -> Self {
        self.parameters.salary_include_unknown = Some(salary_include_unknown.into());
        self
    }

    pub fn months(mut self, months: usize) -> Self {
        self.parameters.months = Some(months);
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.parameters.category = Some(category.into());
        self
    }

    pub fn country(mut self, country: Country) -> Self {
        self.search_country = Some(country.into());
        self
    }

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

    pub async fn fetch(&self) -> Result<T, StatusCode>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let url: String = ROOT_URL.to_string()
            + &match self.search_country {
                Some(country) => format!("/jobs/{}/{}", country, self.endpoint),
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
                    return Err(r.status());
                }
            }
            Err(e) => {
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
