use crate::utils::E;
use async_trait::async_trait;

pub mod huggingface;

#[async_trait]
pub trait AIApi: Send + Sync {
    async fn get_embedding(&self, _: &String) -> Result<Vec<f64>, E>;
}
