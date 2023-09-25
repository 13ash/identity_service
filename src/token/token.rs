use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

pub const DEFAULT_TOKEN_EXPIRATION: i64 = 60 * 60 * 24 * 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode_token(
    user_name: String,
    encoding_key: &EncodingKey,
    exp_duration_seconds: Option<i64>,
) -> String {
    let duration: i64;
    match exp_duration_seconds {
        None => duration = DEFAULT_TOKEN_EXPIRATION,
        Some(_) => duration = exp_duration_seconds.unwrap(),
    }
    let claims = Claims {
        sub: user_name,
        exp: (Utc::now() + Duration::seconds(duration)).timestamp() as usize,
    };
    let token = encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA),
        &claims,
        encoding_key,
    )
    .unwrap();

    token
}

pub fn decode_token(token: &str, decoding_key: &DecodingKey) -> Claims {
    let validation = Validation::new(Algorithm::EdDSA);
    let token_data = decode::<Claims>(token, decoding_key, &validation).unwrap();

    token_data.claims
}
