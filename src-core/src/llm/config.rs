use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub api_key: String,
    pub model: String,
    pub embedding_model: String,
    pub temperature: f32,
    pub max_tokens: Option<u16>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gpt-4-turbo-preview".to_string(),
            embedding_model: "text-embedding-3-small".to_string(),
            temperature: 0.7,
            max_tokens: Some(2048),
        }
    }
}