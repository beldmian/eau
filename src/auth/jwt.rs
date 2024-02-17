use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::auth::{AuthProvider, IdentificationPayload};
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
    pub fn new(secret: String) -> Result<impl AuthProvider, E> {
        Ok(Self {
            key: Hmac::new_from_slice(secret.as_bytes())?,
        })
    }
}
