use tauri::State;
use wealthfolio_core::db::connection::DbConnection;
use wealthfolio_core::plaid::{PlaidConfig, PlaidService, PlaidConnection, PlaidAccount};
use wealthfolio_core::plaid::models::PlaidTransaction;
use chrono::NaiveDate;

#[tauri::command]
pub async fn create_plaid_link_token(
    user_id: String,
    db: State<'_, DbConnection>,
) -> Result<String, String> {
    // For now, use sandbox credentials
    let config = PlaidConfig {
        client_id: std::env::var("PLAID_CLIENT_ID").unwrap_or_else(|_| "your_client_id".to_string()),
        secret: std::env::var("PLAID_SECRET").unwrap_or_else(|_| "your_secret".to_string()),
        environment: wealthfolio_core::plaid::config::PlaidEnvironment::Sandbox,
        redirect_uri: None,
    };

    let service = PlaidService::new(config, db.inner().clone());
    
    service
        .create_link_token(&user_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn exchange_plaid_public_token(
    public_token: String,
    institution_name: String,
    db: State<'_, DbConnection>,
) -> Result<String, String> {
    let config = PlaidConfig {
        client_id: std::env::var("PLAID_CLIENT_ID").unwrap_or_else(|_| "your_client_id".to_string()),
        secret: std::env::var("PLAID_SECRET").unwrap_or_else(|_| "your_secret".to_string()),
        environment: wealthfolio_core::plaid::config::PlaidEnvironment::Sandbox,
        redirect_uri: None,
    };

    let service = PlaidService::new(config, db.inner().clone());
    
    service
        .exchange_public_token(&public_token, &institution_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_plaid_connections(
    db: State<'_, DbConnection>,
) -> Result<Vec<PlaidConnection>, String> {
    let config = PlaidConfig::default();
    let service = PlaidService::new(config, db.inner().clone());
    
    service
        .get_connections()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_plaid_accounts(
    connection_id: String,
    db: State<'_, DbConnection>,
) -> Result<Vec<PlaidAccount>, String> {
    let config = PlaidConfig::default();
    let service = PlaidService::new(config, db.inner().clone());
    
    service
        .get_accounts_by_connection(&connection_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sync_plaid_accounts(
    connection_id: String,
    db: State<'_, DbConnection>,
) -> Result<Vec<PlaidAccount>, String> {
    let config = PlaidConfig {
        client_id: std::env::var("PLAID_CLIENT_ID").unwrap_or_else(|_| "your_client_id".to_string()),
        secret: std::env::var("PLAID_SECRET").unwrap_or_else(|_| "your_secret".to_string()),
        environment: wealthfolio_core::plaid::config::PlaidEnvironment::Sandbox,
        redirect_uri: None,
    };

    let service = PlaidService::new(config, db.inner().clone());
    
    service
        .sync_accounts(&connection_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sync_plaid_transactions(
    connection_id: String,
    start_date: String,
    end_date: String,
    db: State<'_, DbConnection>,
) -> Result<Vec<PlaidTransaction>, String> {
    let config = PlaidConfig {
        client_id: std::env::var("PLAID_CLIENT_ID").unwrap_or_else(|_| "your_client_id".to_string()),
        secret: std::env::var("PLAID_SECRET").unwrap_or_else(|_| "your_secret".to_string()),
        environment: wealthfolio_core::plaid::config::PlaidEnvironment::Sandbox,
        redirect_uri: None,
    };

    let service = PlaidService::new(config, db.inner().clone());
    
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;
    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;
    
    service
        .sync_transactions(&connection_id, start, end)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_plaid_connection(
    connection_id: String,
    db: State<'_, DbConnection>,
) -> Result<(), String> {
    let config = PlaidConfig::default();
    let service = PlaidService::new(config, db.inner().clone());
    
    service
        .remove_connection(&connection_id)
        .map_err(|e| e.to_string())
}