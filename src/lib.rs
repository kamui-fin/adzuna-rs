use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/* get jobs/{country}/search/{page}
Search the Adzuna job database

get jobs/{country}/categories
List available categories

get jobs/{country}/histogram
Provide histogram data of salary data

get jobs/{country}/top_companies
List the top employers for the search terms supplied

get jobs/{country}/geodata
Provides salary data for locations inside an area

get jobs/{country}/history
Provides historical average salary data

get version
Returns the current version of this API */

const ROOT_URL: &str = "https://api.adzuna.com/v1/api";

pub struct Client {
    app_id: String,
    app_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub exception: String,
    pub doc: String,
    pub display: String,
}

pub enum Country {
    UnitedKingdom,
    UnitedStates,
    Austria,
    Australia,
    Belgium,
    Brazil,
    Canada,
    Switzerland,
    Germany,
    Spain,
    France,
    India,
    Italy,
    Mexico,
    Netherlands,
    NewZealand,
    Poland,
    Russia,
    Singapore,
    SouthAfrica,
}

impl Country {
    fn to_code(&self) -> &'static str {
        match self {
            Country::UnitedKingdom => "gb",
            Country::UnitedStates => "us",
            Country::Austria => "at",
            Country::Australia => "au",
            Country::Belgium => "be",
            Country::Brazil => "br",
            Country::Canada => "ca",
            Country::Switzerland => "ch",
            Country::Germany => "de",
            Country::Spain => "es",
            Country::France => "fr",
            Country::India => "in",
            Country::Italy => "it",
            Country::Mexico => "mx",
            Country::Netherlands => "nl",
            Country::NewZealand => "nl",
            Country::Poland => "pl",
            Country::Russia => "ru",
            Country::Singapore => "sg",
            Country::SouthAfrica => "za",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionResponse {
    pub api_version: u8,
    pub software_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaderboardItem {
    count: usize,
    canonical_name: String,
    average_salary: usize,
    display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopCompanyResponse {
    pub leaderboard: Vec<LeaderboardItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    tag: String,
    label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoriesResponse {
    results: Vec<Category>,
}

pub enum SortDirection {
    Up,
    Down,
}

pub enum SortBy {
    Default,
    Hybrid,
    Date,
    Salary,
    Relevance,
}

pub struct Parameters {
    what: Option<String>,
    months: Option<usize>,

    // /search only
    what_and: Option<String>,
    what_phrase: Option<String>,
    what_or: Option<String>,
    what_exclude: Option<String>,
    title_only: Option<String>,
    r#where: Option<String>,
    distance: Option<usize>,
    results_per_page: Option<usize>,
    max_days_old: Option<usize>,
    sort_dir: Option<SortDirection>,
    sort_by: Option<SortBy>,
    salary_min: Option<usize>,
    salary_max: Option<usize>,
    salary_include_unknown: Option<String>,
    full_time: Option<String>,
    part_time: Option<String>,
    contract: Option<String>,
    permanent: Option<String>,
    company: Option<String>,

    locations: Option<Vec<String>>, // TODO: flatten out
    category: Option<String>,
}

pub struct RequestBuilder<'a, T> {
    client: &'a Client,
    endpoint: &'static str,
    search_country: Option<&'static str>,
    __phantom: PhantomData<T>,
}

impl<'a, T> RequestBuilder<'a, T> {
    pub fn new(client: &'a Client, endpoint: &'static str) -> Self {
        Self {
            client,
            endpoint,
            search_country: None,
            __phantom: PhantomData,
        }
    }

    pub fn country(mut self, country: Country) -> Self {
        self.search_country = Some(country.to_code());
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
        let response = client.get(url).query(&auth_params).send().await;

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

impl Client {
    pub fn new(app_id: String, app_key: String) -> Self {
        Self { app_id, app_key }
    }

    pub fn query<T>(&self, endpoint: &'static str) -> RequestBuilder<T> {
        RequestBuilder::<T>::new(self, endpoint)
    }

    pub fn api_version(&self) -> RequestBuilder<VersionResponse> {
        self.query("version")
    }

    pub fn categories(&self) -> RequestBuilder<CategoriesResponse> {
        self.query("categories").country(Country::UnitedStates) // default country
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::Client;

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
