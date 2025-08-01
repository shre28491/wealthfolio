use tauri::State;
use wealthfolio_core::db::connection::DbConnection;
use wealthfolio_core::llm::{LlmConfig, QueryEngine};

#[tauri::command]
pub async fn query_financial_assistant(
    query: String,
    db: State<'_, DbConnection>,
) -> Result<String, String> {
    // Get OpenAI API key from environment
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "your_openai_api_key".to_string());

    let config = LlmConfig {
        api_key,
        model: "gpt-4-turbo-preview".to_string(),
        embedding_model: "text-embedding-3-small".to_string(),
        temperature: 0.7,
        max_tokens: Some(2048),
    };

    let query_engine = QueryEngine::new(config, db.inner().clone());
    
    query_engine
        .query(&query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_financial_insights(
    db: State<'_, DbConnection>,
) -> Result<String, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "your_openai_api_key".to_string());

    let config = LlmConfig {
        api_key,
        model: "gpt-4-turbo-preview".to_string(),
        embedding_model: "text-embedding-3-small".to_string(),
        temperature: 0.7,
        max_tokens: Some(2048),
    };

    let query_engine = QueryEngine::new(config, db.inner().clone());
    
    query_engine
        .generate_financial_insights()
        .await
        .map_err(|e| e.to_string())
}