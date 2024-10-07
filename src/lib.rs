#![deny(unreachable_pub)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]

pub mod api;
pub mod data;
mod error;
mod utils;

pub use api::*;
pub use error::*;

use std::sync::Arc;

use reqwest::Client;

#[derive(Debug, Clone)]
pub struct EagleApi {
    inner: Arc<EagleInner>,
}

#[derive(Debug)]
struct EagleInner {
    host: String,
    client: Client,
}

impl EagleApi {
    pub fn new(host: &str) -> Self {
        EagleApi {
            inner: Arc::new(EagleInner {
                host: host.trim_end_matches('/').to_string(),
                client: Client::new(),
            }),
        }
    }
}
