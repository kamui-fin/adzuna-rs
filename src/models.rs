use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiException {
    /// A string representing the class of exception.
    pub exception: String,
    /// A human readable error message in English.
    pub doc: String,
    /// A URL linking to hopefully the relevant documentation.
    pub display: String,
}

/// The object is returned by the version endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    /// The major version of the API
    pub api_version: u8,
    /// The version of the software providing the API
    pub software_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// The name of the company, in the form provided by the advertiser.
    /// A company name is not always available.
    pub display_name: Option<String>,
    /// A normalised string of the company name. This is not always available.
    /// When available, it can be supplied to the search endpoint in the company query string parameter.
    pub canonical_name: Option<String>,
    /// The total number of job advertisements posted by this company.
    /// This will normally only be provided by statistics queries, not search queries.
    pub count: Option<usize>,
    /// The average salary in job advertisements posted by this company.
    /// This will normally only be provided by statistics queries, not search queries.
    /// The data may be provided with up to two decimal places, and will have no currency symbol.
    pub average_salary: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopCompanies {
    /// A list of Company objects, ordered by the number of advertisements they have in the database.
    pub leaderboard: Option<Vec<Company>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    /// The string which should be passed to search endpoint using the category query parameter.
    pub tag: String,
    /// A text string describing the category, suitable for display.
    pub label: String,
}

/// The object is returned by the categories endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Categories {
    /// An array of all the categories discovered as Category objects.
    pub results: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalSalary {
    /// A hash of salary figures. The key is an ISO 8601 date, omitting the day.
    /// For example, 2013-09 indicates September 2013.
    /// The salary is averaged and supplied with up to two decimal places but no currency symbol, for example 20000.00, 20000.0 or 20000.
    pub month: Option<HashMap<String, f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SalaryHistogram {
    /// Returns the current distribution of jobs by salary.
    /// Results are returned as an associative array of salaries and vacancies.
    ///  - Each salary number indicates the lower end of a range.
    ///  - Each vacancy number is the number of live job ads with a salary in range.
    /// It can be used to generate a "histogram distribution" of salaries.
    /// This is a hashmap containing the histogram data. The buckets are the hash keys, indicating the lowest salary counted in that particular bucket.
    pub histogram: Option<HashMap<String, usize>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationDetail {
    /// A description of the location, as an array of strings, each refining the location more than the previous.
    ///     - Country
    ///     - Region
    ///     - Sub-region or city
    ///     - City, town or suburb
    /// Locations may have up to five levels of detail. For example,
    ///     - UK
    ///     - South East England
    ///     - Surrey
    ///     - Reigate
    ///     - Leigh
    /// This data may be serialised into a series of locationN query string parameters and be supplied to the search and statistical endpoints.
    pub area: Option<Vec<String>>,
    /// A human readable name for the location, intended for display in applications.
    pub display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationJobs {
    /// The number of jobs available at this location.
    pub count: Option<usize>,
    /// More detail about the location
    pub location: Option<LocationDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobGeoData {
    /// List of Location objects
    pub locations: Option<Vec<LocationJobs>>,
}

fn string_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(buf == "1")
}

#[derive(PartialEq, Clone, Debug)]
pub enum ContractType {
    Permanent,
    Contract,
}

fn decode_contract_type<'de, D>(deserializer: D) -> Result<Option<ContractType>, D::Error>
where
    D: Deserializer<'de>,
{
    let option = Option::<String>::deserialize(deserializer)?;
    match option {
        Some(option) => Ok(match option.as_str() {
            "permanent" => Some(ContractType::Permanent),
            "contract" => Some(ContractType::Contract),
            &_ => None,
        }),
        None => Ok(None),
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ContractTime {
    FullTime,
    PartTime,
}

fn decode_contract_time<'de, D>(deserializer: D) -> Result<Option<ContractTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let option = Option::<String>::deserialize(deserializer)?;
    match option {
        Some(option) => Ok(match option.as_str() {
            "full_time" => Some(ContractTime::FullTime),
            "part_time" => Some(ContractTime::PartTime),
            &_ => None,
        }),
        None => Ok(None),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Job {
    /// A string uniquely identifying this advertisement.
    pub id: String,
    /// The date the ad was created as defined by the original source of the ad.
    /// Where we find an ad without a created date, we use the time we first saw the ad.
    /// It is formatted as an ISO 8601 date time string.
    pub created: String,
    /// A summary of the advertisement.
    pub title: String,
    /// The details of the advertisement, truncated to 500 characters.
    pub description: String,
    /// A URL which will redirect to the advertisement as displayed on the advertiser's site.
    /// Using this URL send a user to the advertiser's site is necessary to be compliant with Adzuna's terms and conditions.
    pub redirect_url: String,
    /// The latitude of the workplace, in degrees.
    pub latitude: f64,
    /// The longitude of the workplace, in degrees.
    pub longitude: f64,
    /// The category of the advertisement.
    pub category: Category,
    /// The locality of the advertisement.
    pub location: LocationDetail,
    /// The bottom end of the pay scale for this job, given in the local currency.
    pub salary_min: f64,
    /// The top end of the pay scale for this job, given in the local currency.
    pub salary_max: f64,
    /// A flag indicating if the salary of the job was predicted by Adzuna's 'Jobsworth' technology.
    /// Jobsworth predicts salaries for jobs with no advertised salary.
    /// Predictions are based on continual analysis of millions of ads. Most of the time predictions are accurate within 10%.
    #[serde(deserialize_with = "string_bool")]
    pub salary_is_predicted: bool,
    /// The company offering the job.
    pub company: Company,
    /// Either `permanent` or `contract` to indicate whether the job is permanent or just a short-term contract.
    #[serde(default)]
    #[serde(deserialize_with = "decode_contract_type")]
    pub contract_type: Option<ContractType>,
    /// Either `full_time` or `part_time` to indicate the hours of the job.
    #[serde(default)]
    #[serde(deserialize_with = "decode_contract_time")]
    pub contract_time: Option<ContractTime>,
    /// TBD
    pub adref: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct JobSearchResults {
    /// An array of the search results as Job objects.
    pub results: Vec<Job>,
    /// Number of jobs that were matched
    pub count: usize,
    /// The mean salary across all the results
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

fn location_serialize<S>(locations: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ser = s.serialize_map(Some(8))?;
    for (i, location) in locations.iter().enumerate() {
        ser.serialize_entry(&format!("location{i}"), location)?;
    }
    ser.end()
}

#[derive(Debug, Default, Serialize)]
pub struct Parameters {
    #[serde(serialize_with = "location_serialize")]
    #[serde(flatten)]
    pub locations: Vec<String>,
    pub category: Option<String>,

    pub what: Option<String>,
    pub months: Option<usize>,

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
