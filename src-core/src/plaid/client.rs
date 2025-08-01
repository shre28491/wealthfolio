use crate::errors::WealthfolioError;
use crate::plaid::config::{PlaidConfig, PlaidEnvironment};
use crate::plaid::models::*;
use reqwest::{Client, StatusCode};
use serde_json::json;

pub struct PlaidClient {
    config: PlaidConfig,
    client: Client,
}

impl PlaidClient {
    pub fn new(config: PlaidConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: serde_json::Value,
    ) -> Result<T, WealthfolioError> {
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let mut request_body = body;
        request_body["client_id"] = json!(self.config.client_id);
        request_body["secret"] = json!(self.config.secret);

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let result = response
                    .json::<T>()
                    .await
                    .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?;
                Ok(result)
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(WealthfolioError::ExternalError(format!(
                    "Plaid API error: {}",
                    error_text
                )))
            }
        }
    }

    pub async fn create_link_token(
        &self,
        user_id: &str,
    ) -> Result<LinkTokenResponse, WealthfolioError> {
        let request = json!({
            "client_name": "Wealthfolio",
            "language": "en",
            "country_codes": ["US"],
            "products": ["transactions", "accounts", "investments"],
            "user": {
                "client_user_id": user_id
            }
        });

        self.make_request("/link/token/create", request).await
    }

    pub async fn exchange_public_token(
        &self,
        public_token: &str,
    ) -> Result<PublicTokenExchangeResponse, WealthfolioError> {
        let request = json!({
            "public_token": public_token
        });

        self.make_request("/item/public_token/exchange", request)
            .await
    }

    pub async fn get_accounts(
        &self,
        access_token: &str,
    ) -> Result<AccountsGetResponse, WealthfolioError> {
        let request = json!({
            "access_token": access_token
        });

        self.make_request("/accounts/get", request).await
    }

    pub async fn get_transactions(
        &self,
        access_token: &str,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> Result<TransactionsGetResponse, WealthfolioError> {
        let request = json!({
            "access_token": access_token,
            "start_date": start_date.format("%Y-%m-%d").to_string(),
            "end_date": end_date.format("%Y-%m-%d").to_string(),
        });

        self.make_request("/transactions/get", request).await
    }

    pub async fn get_institution(
        &self,
        institution_id: &str,
    ) -> Result<serde_json::Value, WealthfolioError> {
        let request = json!({
            "institution_id": institution_id,
            "country_codes": ["US"]
        });

        self.make_request("/institutions/get_by_id", request).await
    }
}