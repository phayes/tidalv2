#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod apis;
pub mod models;

#[cfg(feature = "tidalrs")]
pub mod tidalrs;

/// Initialize basic logging for the TIDAL API client
///
/// This function sets up logging to help debug HTTP requests and responses.
/// Call this before making any API calls to see the logging output.
///
/// # Example
/// ```no_run
/// use tidalv2::init_logging;
///
/// // Initialize logging at the start of your application
/// init_logging();
///
/// // Now all HTTP requests will be logged with their headers, URL, method, and response code
/// ```
pub fn init_logging() {
    env_logger::init();
}
