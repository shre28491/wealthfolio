use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaidConfig {
    pub client_id: String,
    pub secret: String,
    pub environment: PlaidEnvironment,
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaidEnvironment {
    Development,
    Sandbox,
    Production,
}

impl PlaidEnvironment {
    pub fn as_str(&self) -> &str {
        match self {
            PlaidEnvironment::Development => "development",
            PlaidEnvironment::Sandbox => "sandbox",
            PlaidEnvironment::Production => "production",
        }
    }

    pub fn base_url(&self) -> &str {
        match self {
            PlaidEnvironment::Development => "https://development.plaid.com",
            PlaidEnvironment::Sandbox => "https://sandbox.plaid.com",
            PlaidEnvironment::Production => "https://production.plaid.com",
        }
    }
}

impl Default for PlaidConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            secret: String::new(),
            environment: PlaidEnvironment::Sandbox,
            redirect_uri: None,
        }
    }
}