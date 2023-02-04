use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub exception: String,
    pub doc: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiVersion {
    pub api_version: u8,
    pub software_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    count: Option<usize>,
    canonical_name: Option<String>,
    average_salary: Option<usize>,
    display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopCompaniesResponse {
    pub leaderboard: Option<Vec<Company>>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryResponse {
    month: Option<HashMap<String, f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramResponse {
    histogram: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationDetail {
    area: Option<Vec<String>>,
    display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    count: Option<usize>,
    location: Option<LocationDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeodataResponse {
    locations: Option<Vec<Location>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    id: String,
    adref: String,
    created: String,
    title: String,
    description: String,
    redirect_url: String,
    latitude: f64,
    longitude: f64,
    category: Category,
    location: LocationDetail,
    salary_min: f64,
    salary_max: f64,
    salary_is_predicted: String,
    company: Company,
    contract_type: Option<String>,
    contract_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    results: Vec<Job>,
    count: usize,
    mean: f64,
}

pub enum SortDirection {
    Up,
    Down,
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SortDirection::Up => {
                write!(f, "up")
            }
            SortDirection::Down => {
                write!(f, "down")
            }
        }
    }
}

pub enum SortBy {
    Default,
    Hybrid,
    Date,
    Salary,
    Relevance,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SortBy::Default => {
                write!(f, "default")
            }
            SortBy::Hybrid => {
                write!(f, "hybrid")
            }
            SortBy::Date => {
                write!(f, "date")
            }
            SortBy::Salary => {
                write!(f, "salary")
            }
            SortBy::Relevance => {
                write!(f, "relevance")
            }
        }
    }
}

#[derive(Default, Serialize)]
pub struct Parameters {
    pub what: Option<String>,
    pub months: Option<usize>,

    // /search only
    pub what_and: Option<String>,
    pub what_phrase: Option<String>,
    pub what_or: Option<String>,
    pub what_exclude: Option<String>,
    pub title_only: Option<String>,
    pub r#where: Option<String>,
    pub salary_include_unknown: Option<String>,
    pub full_time: Option<String>,
    pub part_time: Option<String>,
    pub contract: Option<String>,
    pub permanent: Option<String>,
    pub company: Option<String>,
    pub distance: Option<usize>,
    pub results_per_page: Option<usize>,
    pub max_days_old: Option<usize>,
    pub salary_min: Option<usize>,
    pub salary_max: Option<usize>,
    pub sort_dir: Option<String>,
    pub sort_by: Option<String>,

    pub location0: Option<String>,
    pub location1: Option<String>,
    pub location2: Option<String>,
    pub location3: Option<String>,
    pub location4: Option<String>,
    pub location5: Option<String>,
    pub location6: Option<String>,
    pub location7: Option<String>,

    pub category: Option<String>,
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
    pub fn to_code(&self) -> &'static str {
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

impl Display for Country {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_code())
    }
}
