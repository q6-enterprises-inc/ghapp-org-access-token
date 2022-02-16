pub mod httpsend {
    use reqwest;
    use chrono::Utc;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use anyhow::{Result, Context};

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        iat: usize,
        exp: usize,
        iss: String,
    }

    pub trait HttpSend {
        fn generate_token(app_id: String, private_key_path: String) -> Result<String, Box<dyn std::error::Error>> {
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
                iss: app_id.to_string(),
            };

            let private_key = fs::read(private_key_path)?;

            let token = encode(
                &Header::new(Algorithm::RS256),
                &claims,
                &EncodingKey::from_rsa_pem(&private_key)?,
            )?;

            Ok(token)
        }

        fn get_installation_id(token: &str, org: String) -> Result<String, reqwest::Error> {
            let client = reqwest::blocking::Client::new();

            let res = client
                .get(format!(
                    "https://api.github.com/orgs/{}/installation",
                    org
                ))
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", APP_USER_AGENT)
                .send()?;

            res.text()
        }

        fn get_access_token(access_token_url: &str, token: &str) -> Result<String, reqwest::Error> {
            let client = reqwest::blocking::Client::new();

            let res = client
                .post(access_token_url)
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", APP_USER_AGENT)
                .send()?;

            let data = res.text()?;
            Ok(data)
        }
    }

    pub fn run<T>(_t: T, app_id: String, private_key_path: String, org: String) -> Result<String, Box<dyn std::error::Error>>
        where T: HttpSend
    {
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

        let token = T::generate_token(app_id, private_key_path)?;

        let res = T::get_installation_id(&token, org)?;

        let gh_installation_response: GhInstallationResponse = serde_json::from_str(&res).with_context(|| format!("received response: {}", res))?;

        let access_token_url = &gh_installation_response.access_tokens_url;

        let res = T::get_access_token(access_token_url, &token).with_context(|| "used url")?;

        let gh_access_token_response: GhAccessTokenResponse = serde_json::from_str(&res).with_context(|| format!("could not convert json to GhAccessTokenResponse, recieved: {}", res))?;

        let output = FinalOutput {
            access_token: gh_access_token_response.token,
            expiration: gh_access_token_response.expires_at,
            access_token_url: gh_installation_response.access_tokens_url.to_string(),
            installation_id: gh_installation_response.id,
        };

        serde_json::to_string(&output).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use crate::httpsend::{HttpSend, run};

    #[test]
    fn run_test() {
        struct MockHttpSend;

        impl HttpSend for MockHttpSend {
            fn generate_token(_app_id: String, _private_key_path: String) -> Result<String, Box<dyn std::error::Error>> {
                Ok("token".to_string())
            }

            fn get_installation_id(_token: &str, _org: String) -> Result<String, reqwest::Error> {
                Ok(r#"{"id": 2342234, "access_tokens_url": "test url"}"#.to_string())
            }

            fn get_access_token(_access_token_url: &str, _token: &str) -> Result<String, reqwest::Error> {
                Ok(r#"{"token": "access token", "expires_at": "2022-02-16T21:34:13Z"}"#.to_string())
            }
        }

        let app_id = "234233".to_string();
        let private_key_path = "test path".to_string();
        let org = "org".to_string();

        let result = run(MockHttpSend, app_id, private_key_path, org);

        match result {
            Ok(_) => (),
            err => panic!("should have been Ok, got {:#?}", err),
        };
    }
}
