use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception {
    pub exception: String,
    pub doc: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub api_version: u8,
    pub software_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub count: Option<usize>,
    pub canonical_name: Option<String>,
    pub average_salary: Option<usize>,
    pub display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopCompanies {
    pub leaderboard: Option<Vec<Company>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub tag: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Categories {
    pub results: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalSalary {
    pub month: Option<HashMap<String, f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SalaryHistogram {
    pub histogram: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationDetail {
    pub area: Option<Vec<String>>,
    pub display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub count: Option<usize>,
    pub location: Option<LocationDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobGeoData {
    pub locations: Option<Vec<Location>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub adref: String,
    pub created: String,
    pub title: String,
    pub description: String,
    pub redirect_url: String,
    pub latitude: f64,
    pub longitude: f64,
    pub category: Category,
    pub location: LocationDetail,
    pub salary_min: f64,
    pub salary_max: f64,
    pub salary_is_predicted: String,
    pub company: Company,
    pub contract_type: Option<String>,
    pub contract_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobSearchResults {
    pub results: Vec<Job>,
    pub count: usize,
    pub mean: f64,
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
    pub location0: Option<String>,
    pub location1: Option<String>,
    pub location2: Option<String>,
    pub location3: Option<String>,
    pub location4: Option<String>,
    pub location5: Option<String>,
    pub location6: Option<String>,
    pub location7: Option<String>,
    pub category: Option<String>,

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
