use chrono::{DateTime, Utc, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Database models
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::plaid_connections)]
pub struct PlaidConnection {
    pub id: String,
    pub access_token: String,
    pub item_id: String,
    pub institution_id: String,
    pub institution_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::plaid_connections)]
pub struct NewPlaidConnection {
    pub id: String,
    pub access_token: String,
    pub item_id: String,
    pub institution_id: String,
    pub institution_name: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::plaid_accounts)]
pub struct PlaidAccount {
    pub id: String,
    pub connection_id: String,
    pub account_id: String,
    pub account_name: String,
    pub account_type: String,
    pub account_subtype: Option<String>,
    pub currency: String,
    pub current_balance: f64,
    pub available_balance: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::plaid_accounts)]
pub struct NewPlaidAccount {
    pub id: String,
    pub connection_id: String,
    pub account_id: String,
    pub account_name: String,
    pub account_type: String,
    pub account_subtype: Option<String>,
    pub currency: String,
    pub current_balance: f64,
    pub available_balance: Option<f64>,
}

// API models
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenRequest {
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<String>,
    pub products: Vec<String>,
    pub user: LinkUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkUser {
    pub client_user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenResponse {
    pub link_token: String,
    pub expiration: String,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTokenExchangeRequest {
    pub public_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTokenExchangeResponse {
    pub access_token: String,
    pub item_id: String,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    pub accounts: Vec<PlaidApiAccount>,
    pub item: PlaidItem,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidApiAccount {
    pub account_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub subtype: Option<String>,
    pub balances: PlaidBalance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidBalance {
    pub available: Option<f64>,
    pub current: f64,
    pub iso_currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidItem {
    pub item_id: String,
    pub institution_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequest {
    pub access_token: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub options: Option<TransactionsOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsOptions {
    pub count: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetResponse {
    pub accounts: Vec<PlaidApiAccount>,
    pub transactions: Vec<PlaidTransaction>,
    pub total_transactions: u32,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidTransaction {
    pub transaction_id: String,
    pub account_id: String,
    pub amount: f64,
    pub iso_currency_code: Option<String>,
    pub date: NaiveDate,
    pub name: String,
    pub merchant_name: Option<String>,
    pub category: Option<Vec<String>>,
    pub pending: bool,
}