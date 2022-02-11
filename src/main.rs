use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::Utc;
use std::fs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Github app id.
    #[clap(short, long)]
    app_id: String,

    /// Relative path to the Github App private key.
    #[clap(short, long)]
    private_key_path: String,
}

fn main() {
    let args = Args::parse();

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
            iss: args.app_id.to_string(),
    };

    let private_key = fs::read(args.private_key_path).unwrap();

    let token = encode(&Header::new(Algorithm::RS256), &claims, &EncodingKey::from_rsa_pem(&private_key).unwrap()).unwrap();

    println!("{}", token);
}
