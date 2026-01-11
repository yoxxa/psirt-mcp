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
    client: Client,
    bearer_auth: Arc<RwLock<Option<String>>>
}

impl PsirtApi {
    pub fn new() -> PsirtApi {
        let headers = construct_headers();
        let config = PsirtApiConfig::new();
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(5))
            .retry(for_host("https://apix.cisco.com").max_retries_per_request(2))
            .build()
            .expect("Failed to build PsirtApi `reqwest` client");
        Self {
            config,
            client,
            bearer_auth: Arc::new(RwLock::new(None))
        }
    }

    async fn auth(&self) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        let response = self.client.post("https://id.cisco.com/oauth2/default/v1/token")
            .form(&json!({
                "client_id": self.config.client_id,
                "client_secret": self.config.client_secret,
                "grant_type": self.config.grant_type
            }))
            .headers(headers)
            .send()
            .await?;
        let response = response.error_for_status()?;
        Ok(response)
    }

    pub async fn authenticate(&self) -> Result<(), Error> {
        let mut bearer_auth = self.bearer_auth.write().await;
        let response = self.auth().await?;
        let response = response.json::<Auth>().await?;
        *bearer_auth = Some(response.access_token);
        Ok(())
    }
}

