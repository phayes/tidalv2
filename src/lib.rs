#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub(crate) mod apis;
pub mod auth;
pub mod client;
pub mod error;
pub mod models;

pub use client::TidalClient;

#[cfg(feature = "tidalrs")]
pub mod tidalrs;
