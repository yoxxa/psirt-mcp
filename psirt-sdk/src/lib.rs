mod config;
mod auth;

use config::PsirtApiConfig;
use auth::Auth;

use psirt_utils::construct_headers;

use std::{collections::HashMap, time::Duration};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::{
    Client, 
    Error, 
    StatusCode, 
    Response, 
    header::{CONTENT_TYPE, HeaderMap},
    retry::for_host
};
use serde_json::json;

#[derive(Debug)]
pub struct PsirtApi {
    config: PsirtApiConfig,
    pub client: Client,
    pub bearer: String,
    pub bearer_auth: Arc<RwLock<Option<String>>>
}

impl PsirtApi {
    pub fn new() -> PsirtApi {
        let headers = construct_headers();
        let config = PsirtApiConfig::new();
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(150))
            .retry(for_host("https://apix.cisco.com").max_retries_per_request(2))
            .build()
            .expect("Failed to build PsirtApi `reqwest` client");
        Self {
            config,
            client,
            bearer: String::new(),
            bearer_auth: Arc::new(RwLock::new(None))
        }
    }
}

