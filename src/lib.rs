#![doc = include_str!("../README.md")]
pub use self::client::Client;
pub use self::request::RequestBuilder;

pub mod client;
pub mod models;
pub mod request;

// TODO:
// - error handling
// - rate limiter
// - health method
// - ping method
// - CI
// - deserialize enum
//
// FIX:
// - mandatory what parameter for search
