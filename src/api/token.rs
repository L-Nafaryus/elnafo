use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl TokenClaims {
    pub fn create(
        sub: String,
        secret: String,
        duration: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + chrono::Duration::try_seconds(duration).unwrap()).timestamp() as usize;
        let claims = Self { sub, exp, iat };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn validate(token: String, secret: String) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?
        .claims)
    }
}
