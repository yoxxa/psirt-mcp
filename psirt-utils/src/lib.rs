use std::env;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

pub fn load_env_variables() -> (String, String, String) {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let grant_type = env::var("GRANT_TYPE").unwrap();
    (client_id, client_secret, grant_type)
}

pub fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}

#[cfg(test)]
mod tests {
    use crate::{
        load_env_variables,
        construct_headers
    };

    #[test]
    fn test_load_env_variables() {
        let (client_id, 
            client_secret, grant_type) = load_env_variables();
    }
    
    #[test]
    fn test_construct_headers() {
        let headers = construct_headers();
    }
}