# Adzuna API Wrapper

An easy to use, asynchronous, and complete Rust crate for interacting with the [Adzuna](https://www.adzuna.com/) API.

<!-- You can view the crate's entire documentation [here](). For those who are looking for the official Adzuna API docs, you can view it [here](https://developer.adzuna.com/overview). -->

<!-- [![Build](https://github.com/halcyonnouveau/roux/actions/workflows/rust.yml/badge.svg)](https://github.com/halcyonnouveau/roux/actions/workflows/rust.yml) -->
<!-- [![Documentation](https://img.shields.io/badge/documentation-available-green.svg)](https://docs.rs/roux) -->
<!-- [![Crate](https://img.shields.io/crates/v/roux.svg)](https://crates.io/crates/roux) -->

[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/gpl-3.0.html)

## Installation

Via `cargo`, add this to your project's `Cargo.toml`:

```
[dependencies]
adzuna-rs = "1.0.0"
```

## Usage

First, obtain an `api_id` and `api_key` by [registering](https://developer.adzuna.com/signup) for the API. Then, you can instantiate a `Client`:

```rs
use adzuna::Client;

let client = Client::new("API_ID".into(), "API_KEY".into());
```

You can access all the endpoints from this `client`.
Calling an endpoint will return a request builder, which allows you to chain calls for idiomatic query parameter configuration.

After customizing the query, you have to call `.fetch()`, which asynchronously sends the request and returns the data in a `Result<T, AdzunaError>`.
`AdzunaError` optionally contains more information about the error returned by the API as such:

```rs
AdzunaError {
    api_error: Some(
        ApiException {
            exception: "AUTH_FAIL",
            doc: "https://api.adzuna.com/v1/doc",
            display: "Authorisation failed",
        },
    ),
    http_status: 401,
}
```

### Examples

Getting the top companies for SWE in Texas:

```rs
let companies = client
    .top_companies()
    .what("software engineering")
    .location("US")
    .location("Texas")
    .fetch()
    .await;
```

Search for UI Design jobs 5km away from Boston:

```rs
let jobs = client
    .search()
    .what("ui design")
    .where("boston")
    .distance(5)
    .fetch()
    .await;
```

Search for part time sales jobs sorted by salary in descending order:

```rs
use adzuna::models::{SortBy, SortDirection};

let jobs = client
    .search()
    .what("sales")
    .sort_by(SortBy::Salary)
    .sort_dir(SortDirection::Down)
    .fetch()
    .await;
```

Generate a histogram of salary data for data analyst jobs:

```rs
let jobs = client
    .histogram()
    .what("data analyst")
    .fetch()
    .await;
```

## Contributing

Contributions are always welcome! This crate currently covers all the endpoints mentioned in the official documentation, but if you see something missing or encounter a bug, feel free to open an issue or create a pull request.
