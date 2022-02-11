use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::Utc;
use std::fs;

use ghapp_org_access_token::config::Config;

fn main() {
    let config = Config::new().unwrap();

    /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        iat: usize,
        exp: usize,
        iss: String,
    }

    let issued = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(-10))
        .expect("valid timestamp");

    let expiration = issued
        .checked_add_signed(chrono::Duration::minutes(10))
        .expect("valid timestamp")
        .timestamp();


    let claims = Claims {
            iat: issued.timestamp() as usize,
            exp: expiration as usize,
            iss: config.app_id.to_string(),
    };

    let private_key = fs::read(config.private_key_path).unwrap();

    let token = encode(&Header::new(Algorithm::RS256), &claims, &EncodingKey::from_rsa_pem(&private_key).unwrap()).unwrap();

    println!("{:?}", token);
}
