use jsonwebtoken::{encode, Header, errors::Error as JwtError, EncodingKey, Validation, Algorithm, decode, DecodingKey};

use crate::{AuthToken, RefreshToken};

pub struct TokenIssuer {
    auth_encoding: EncodingKey,
    auth_decoding: DecodingKey,
    refresh_encoding: EncodingKey,
    refresh_decoding: DecodingKey
}

impl TokenIssuer {
    pub fn new(auth_secret: String, refresh_secret: String) -> TokenIssuer {
        TokenIssuer {
            auth_encoding: EncodingKey::from_secret(auth_secret.as_bytes()),
            auth_decoding: DecodingKey::from_secret(auth_secret.as_bytes()),
            refresh_encoding: EncodingKey::from_secret(refresh_secret.as_bytes()),
            refresh_decoding: DecodingKey::from_secret(refresh_secret.as_bytes())
        }
    }

    pub fn get_auth(&self, auth_token: &AuthToken) -> Result<String, JwtError> {
        encode(&Header::default(), auth_token, &self.auth_encoding)
    }

    pub fn validate_auth(&self, jwt: &str, username: &str, base_uri: &str) -> Result<AuthToken, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.sub = Some(username.into());
        validation.set_audience(&[base_uri]);

        match decode::<AuthToken>(jwt, &self.auth_decoding, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err),
        }
    }

    pub fn get_refresh(&self, refresh_token: &RefreshToken) -> Result<String, JwtError> {
        encode(&Header::default(), refresh_token, &self.refresh_encoding)
    }

    pub fn validate_refresh(&self, jwt: &str, username: &str) -> Result<RefreshToken, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.sub = Some(username.into());

        match decode::<RefreshToken>(jwt, &self.refresh_decoding, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err),
        }
    }
}