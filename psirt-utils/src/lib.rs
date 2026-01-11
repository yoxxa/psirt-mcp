use std::env;
use dotenv::dotenv;

pub fn load_env_variables() -> (String, String, String) {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let grant_type = env::var("GRANT_TYPE").unwrap();
    (client_id, client_secret, grant_type)
}

#[cfg(test)]
mod tests {

    use crate::load_env_variables;

    #[test]
    fn test_load_env_variables() {
        let (client_id, 
            client_secret, grant_type) = load_env_variables();
    }
}