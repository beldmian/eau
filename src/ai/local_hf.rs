use async_trait::async_trait;
use candle_core::Tensor;
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{Config, BertModel, DTYPE};
use hf_hub::api::sync::Api;
use hf_hub::{Repo, RepoType};
use tokenizers::Tokenizer;

use crate::utils::E;
use crate::config::LocalHFConfig;

use super::AIApi;

pub struct LocalHF {
    device: candle_core::Device,
    tokenizer: Tokenizer,
    model: BertModel,
}

impl LocalHF {
    pub fn new(config: &LocalHFConfig) -> Result<Box<dyn AIApi>, E> {
        let device = candle_core::Device::Cpu;
        let repo = Repo::with_revision(config.model_name.clone(), RepoType::Model, "main".to_string());
        let (config_filename, tokenizer_filename, weights_filename) = {
            let api = Api::new()?;
            let api = api.repo(repo);
            let config = api.get("config.json")?;
            let tokenizer = api.get("tokenizer.json")?;
            let weights = api.get("pytorch_model.bin")?;
            (config, tokenizer, weights)
        };
        let config = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename)?;

        let vb = VarBuilder::from_pth(&weights_filename, DTYPE, &device)?;
        let model = BertModel::load(vb, &config)?;
        Ok(Box::new(Self {
            device,
            tokenizer,
            model,
        }))
    }
}

#[async_trait]
impl AIApi for LocalHF {
    async fn get_embedding(&self, text: &String) -> Result<Vec<f64>, E> {
        let tokens = self.tokenizer
            .encode(text.as_str(), true)?
            .get_ids()
            .to_vec();
        let token_ids = Tensor::new(&tokens[..], &self.device)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        let embedding = self.model.forward(&token_ids, &token_type_ids)?;
        let (_n_sentence, n_tokens, _hidden_size) = embedding.dims3()?;
        let embedding = (embedding.sum(1)? / (n_tokens as f64))?;
        let embedding_vec = embedding.to_vec2::<f32>()?;

        Ok(embedding_vec[0]
            .clone()
            .into_iter()
            .map(|x| x as f64)
            .collect())
    }
}
