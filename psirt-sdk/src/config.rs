use psirt_utils::load_env_variables;

#[derive(Debug)]
pub struct PsirtApiConfig {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

// TODO: make this read in from either env variables or a TOML config file
impl PsirtApiConfig {
    pub fn new() -> PsirtApiConfig {
        let (client_id, 
            client_secret, grant_type) = load_env_variables();
        Self {
            client_id,
            client_secret,
            grant_type,
        }
    }
}