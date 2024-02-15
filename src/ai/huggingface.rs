use crate::ai;
use crate::utils::E;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum HFEmbeddingResponse {
    Three(Vec<Vec<Vec<f64>>>),
    Two(Vec<Vec<f64>>),
    One(Vec<f64>),
}

pub struct HFApi {
    authorization_token: String,
    models_pipeline: Vec<String>,
}

fn mean_pooling(matrix: &[Vec<f64>]) -> Vec<f64> {
    let mut pooled_values: Vec<f64> = Vec::new();

    for i in 0..matrix[0].len() {
        let mut sum: f64 = 0.0;
        matrix.iter().for_each(|row| {
            sum += row[i];
        });
        let mean_value = sum / matrix.len() as f64;
        pooled_values.push(mean_value);
    }

    pooled_values
}

impl HFApi {
    pub fn new(authorization_token: &String, models_pipeline: &Vec<String>) -> HFApi {
        Self {
            authorization_token: authorization_token.to_string(),
            models_pipeline: models_pipeline.to_owned(),
        }
    }
    async fn get_embedding(&self, text: &String, model: &str) -> Result<Vec<f64>, E> {
        let client = reqwest::Client::new();
        let resp: HFEmbeddingResponse = client
            .post(format!(
                "https://api-inference.huggingface.co/models/{}",
                model
            ))
            .header(
                "Authorization",
                format!("Bearer {}", self.authorization_token),
            )
            .json(&HFRequest {
                inputs: text.to_string(),
            })
            .send()
            .await?
            .json()
            .await?;
        match resp {
            HFEmbeddingResponse::Three(vec_resp) => Ok(mean_pooling(&vec_resp[0])),
            HFEmbeddingResponse::Two(vec_resp) => Ok(mean_pooling(&vec_resp)),
            HFEmbeddingResponse::One(vec_resp) => Ok(vec_resp),
        }
    }
    async fn get_embedding_retrying(&self, text: &String, model: &str) -> Result<Vec<f64>, E> {
        let mut res = self.get_embedding(text, model).await;
        while res.is_err() {
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            res = self.get_embedding(text, model).await;
        }
        res
    }
    async fn get_full_embedding(&self, text: &String) -> Result<Vec<f64>, E> {
        let mut result: Vec<f64> = Vec::new();
        for model in &self.models_pipeline {
            result.append(&mut self.get_embedding_retrying(text, model.as_str()).await?);
        }
        Ok(result)
    }
}

#[async_trait]
impl ai::AIApi for HFApi {
    async fn get_embedding(&self, text: &String) -> Result<Vec<f64>, E> {
        self.get_full_embedding(text).await
    }
}
