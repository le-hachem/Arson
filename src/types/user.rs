use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserData {
    pub email: String,
    pub api_key: String,
}

impl UserData {
    pub fn new(email: String, api_key: String) -> Self {
        Self { email, api_key }
    }

    pub fn is_valid(&self) -> bool {
        !self.email.is_empty() && !self.api_key.is_empty()
    }
} 