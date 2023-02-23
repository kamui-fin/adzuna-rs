#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]
pub use self::client::Client;
pub use self::request::RequestBuilder;

pub mod client;
pub mod models;
pub mod request;

// TODO:
// - rate limiter
// - CI
