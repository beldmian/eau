use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::auth::{AuthProvider, IdentificationPayload};
use crate::config::JWTAuthConfig;
use crate::utils::E;

#[derive(Clone)]
pub struct JWTAuthProvider {
    key: Hmac<Sha256>,
}

impl AuthProvider for JWTAuthProvider {
    fn create_secret(&self, claims: IdentificationPayload) -> Result<String, E> {
        Ok(claims.sign_with_key(&self.key)?)
    }
    fn verify_secret(&self, token: String) -> Result<IdentificationPayload, E> {
        Ok(token.verify_with_key(&self.key)?)
    }
}

impl JWTAuthProvider {
    pub fn new(config: &JWTAuthConfig) -> Result<impl AuthProvider, E> {
        Ok(Self {
            key: Hmac::new_from_slice(config.secret.as_bytes())?,
        })
    }
}
