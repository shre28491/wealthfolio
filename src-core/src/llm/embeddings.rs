use crate::errors::WealthfolioError;
use crate::llm::LlmClient;

pub struct EmbeddingService {
    client: LlmClient,
}

impl EmbeddingService {
    pub fn new(client: LlmClient) -> Self {
        Self { client }
    }

    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, WealthfolioError> {
        self.client.generate_embeddings(text).await
    }

    pub async fn embed_documents(&self, documents: Vec<String>) -> Result<Vec<Vec<f32>>, WealthfolioError> {
        let mut embeddings = Vec::new();
        
        for doc in documents {
            let embedding = self.embed_text(&doc).await?;
            embeddings.push(embedding);
        }
        
        Ok(embeddings)
    }

    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}