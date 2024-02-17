use serde::{Deserialize, Serialize};

use crate::utils::E;

pub mod jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentificationPayload {
    pub id: i64,
}

pub trait AuthProvider: Send + Sync {
    fn create_secret(&self, _: IdentificationPayload) -> Result<String, E>;
    fn verify_secret(&self, _: String) -> Result<IdentificationPayload, E>;
}
