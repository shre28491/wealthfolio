use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateEmbeddingRequestArgs,
    },
    Client,
};
use crate::errors::WealthfolioError;
use crate::llm::config::LlmConfig;

pub struct LlmClient {
    client: Client,
    config: LlmConfig,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Self {
        let client = Client::with_api_key(&config.api_key);
        Self { client, config }
    }

    pub async fn generate_response(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, WealthfolioError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.config.model)
            .temperature(self.config.temperature)
            .max_tokens(self.config.max_tokens.unwrap_or(2048))
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()
                    .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()
                    .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?;

        let content = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| WealthfolioError::ExternalError("No response from LLM".to_string()))?;

        Ok(content)
    }

    pub async fn generate_embeddings(&self, text: &str) -> Result<Vec<f32>, WealthfolioError> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.config.embedding_model)
            .input([text])
            .build()
            .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?;

        let response = self
            .client
            .embeddings()
            .create(request)
            .await
            .map_err(|e| WealthfolioError::ExternalError(e.to_string()))?;

        let embedding = response
            .data
            .first()
            .map(|data| data.embedding.clone())
            .ok_or_else(|| WealthfolioError::ExternalError("No embedding returned".to_string()))?;

        Ok(embedding)
    }
}