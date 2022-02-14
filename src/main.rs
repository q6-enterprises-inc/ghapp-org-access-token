use chrono::Utc;
use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Github app id.
    #[clap(short, long)]
    app_id: String,

    /// Relative path to the Github App private key.
    #[clap(short, long)]
    private_key_path: String,

    /// Organization name as it appears in the github url, i.e. https://github.com/my-org/my-repo.
    #[clap(short, long)]
    org: String,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: usize,
    exp: usize,
    iss: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GhInstallationResponse {
    id: u32,
    access_tokens_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GhAccessTokenResponse {
    token: String,
    expires_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FinalOutput {
    access_token: String,
    expiration: String,
    access_token_url: String,
    installation_id: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

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

    let token = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(&private_key).unwrap(),
    )
    .unwrap();

    let client = reqwest::blocking::Client::new();

    let res = client
        .get(format!(
            "https://api.github.com/orgs/{}/installation",
            args.org
        ))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", APP_USER_AGENT)
        .send()?
        .text()?;

    let gh_installation_response: GhInstallationResponse = serde_json::from_str(&res)?;
    let access_token_url = &gh_installation_response.access_tokens_url;

    let res = client
        .post(access_token_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", APP_USER_AGENT)
        .send()?
        .text()?;

    let gh_access_token_response: GhAccessTokenResponse = serde_json::from_str(&res)?;

    let output = FinalOutput {
        access_token: gh_access_token_response.token,
        expiration: gh_access_token_response.expires_at,
        access_token_url: gh_installation_response.access_tokens_url.to_string(),
        installation_id: gh_installation_response.id,
    };

    println!("{}", serde_json::to_string(&output)?);

    Ok(())
}
