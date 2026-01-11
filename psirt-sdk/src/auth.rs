use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    token_type: String,
    expires_in: u16,
    pub access_token: String,
    scope: String
}