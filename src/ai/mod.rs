use serde::{Serialize, Deserialize};
use crate::utils::E;

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HFEmbeddingResponse {
    ListThree(Vec<Vec<Vec<f64>>>),
    ListTwo(Vec<Vec<f64>>),
    ListOne(Vec<f64>),
}

pub struct HFApi {
    authorization_token: String,
    models_pipeline: Vec<String>,
}

fn mean_pooling(matrix: &Vec<Vec<f64>>) -> Vec<f64> {
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
    pub async fn get_embedding(&self, text: &String, model: &str) -> Result<Vec<f64>, E> {
        let client = reqwest::Client::new();
        let resp: HFEmbeddingResponse = client.post(format!("https://api-inference.huggingface.co/models/{}", model))
            .header("Authorization", format!("Bearer {}", self.authorization_token))
            .json(&HFRequest{
                inputs: text.to_string()
            }).send().await?.json().await?;
        match resp {
            HFEmbeddingResponse::ListThree(vec_resp) => Ok(mean_pooling(&vec_resp[0])),
            HFEmbeddingResponse::ListTwo(vec_resp) => Ok(mean_pooling(&vec_resp)),
            HFEmbeddingResponse::ListOne(vec_resp) => Ok(vec_resp)
        }
    }

    pub async fn get_embedding_retrying(&self, text: &String, model: &str) -> Result<Vec<f64>, E> {
        let mut res = self.get_embedding(text, model).await;
        while let Err(_) = res {
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            res = self.get_embedding(text, model).await;
        }
        res
    }

    pub async fn get_full_embedding(&self, text: &String) -> Result<Vec<f64>, E> {
        let mut result: Vec<f64> = Vec::new();
        for model in &self.models_pipeline {
            result.append(&mut self.get_embedding_retrying(text, model.as_str()).await?);
        }
        Ok(result)
    }
}
