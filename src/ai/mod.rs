use crate::utils::E;
use async_trait::async_trait;

pub mod huggingface;
pub mod local_hf;

#[async_trait]
pub trait AIApi: Send + Sync {
    async fn get_embedding(&self, _: &String) -> Result<Vec<f64>, E>;
}
