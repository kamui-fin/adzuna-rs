use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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

// GET /version
// Parameters:
// app_id: string
// app_key: string
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
    #[serde(rename = "api_version")]
    pub api_version: i64,
    #[serde(rename = "software_version")]
    pub software_version: String,
}

impl Client {
    pub fn new(app_id: String, app_key: String) -> Self {
        Self { app_id, app_key }
    }
    // low level HTTP GET request helper
    async fn get<T>(&self, url: String) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "{}?app_id={}&app_key={}",
                url, self.app_id, self.app_key
            ))
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

        response
            .unwrap()
            .json::<T>()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
