use jsonwebtoken::{DecodingKey, EncodingKey};
use ring::signature::{Ed25519KeyPair, KeyPair};

#[derive(Clone)]
pub struct AuthKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

pub async fn generate_auth_keys() -> AuthKeys {
    let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

    AuthKeys {
        encoding_key,
        decoding_key,
    }
}
