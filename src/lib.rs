pub mod httpsend {
    use anyhow::{Context, Result};
    use chrono::{TimeZone, Utc};
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use reqwest;
    use serde::{Deserialize, Serialize};
    use base64::decode;

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        iat: usize,
        exp: usize,
        iss: String,
    }

    pub trait HttpSend {
        fn generate_token(app_id: &str, private_key: &str, issue_time: i64) -> Result<String> {
            let expiration = Utc
                .timestamp(issue_time, 0)
                .checked_add_signed(chrono::Duration::minutes(10))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                iat: issue_time as usize,
                exp: expiration as usize,
                iss: app_id.to_string(),
            };

            let private_key = decode(private_key)?;

            let token = encode(
                &Header::new(Algorithm::RS256),
                &claims,
                &EncodingKey::from_rsa_pem(&private_key).with_context(|| {
                    "could not encode jwt into rsa256 format - are you sure your key is in pem format?"
                })?,
            )?;

            Ok(token)
        }

        fn get_installation_id(token: &str, org: &str, base_url: &str) -> Result<String> {
            let client = reqwest::blocking::Client::new();

            client
                .get(format!("{}/orgs/{}/installation", base_url, org))
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", APP_USER_AGENT)
                .send()
                .with_context(|| "could not send request to retrieve installation id from github")?
                .text()
                .with_context(|| "could not convert installation id response to text")
        }

        fn get_access_token(access_token_url: &str, token: &str) -> Result<String> {
            let client = reqwest::blocking::Client::new();

            client
                .post(access_token_url)
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", APP_USER_AGENT)
                .send()
                .with_context(|| "could not send request to retrieve access token from github")?
                .text()
                .with_context(|| "could not convert access token request to text")
        }
    }

    pub fn run<T>(
        _t: T,
        app_id: &str,
        private_key: &str,
        org: &str,
        base_url: &str,
        issue_time: i64,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        T: HttpSend,
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

        let token = T::generate_token(&app_id, &private_key, issue_time)
            .with_context(|| "could not generate jwt")?;

        let res = T::get_installation_id(&token, &org, &base_url)
            .with_context(|| format!("could not retrieve installion id from github"))?;

        let gh_installation_response: GhInstallationResponse = serde_json::from_str(&res)
            .with_context(|| {
                format!(
                    "could not extract installion id from github response - response: {}",
                    &res
                )
            })?;

        let access_token_url = &gh_installation_response.access_tokens_url;

        let res = T::get_access_token(&access_token_url, &token).with_context(|| {
            format!(
                "could not get access token - used access token url: {}",
                &access_token_url
            )
        })?;

        let gh_access_token_response: GhAccessTokenResponse = serde_json::from_str(&res)
            .with_context(|| {
                format!(
                    "could not extract access token from github response - response: {}",
                    res
                )
            })?;

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
    use crate::httpsend::{run, HttpSend};
    use anyhow::Result;
    use serde_json::json;

    #[test]
    fn run_test() {
        struct MockHttpSend;

        impl HttpSend for MockHttpSend {
            fn generate_token(
                _app_id: &str,
                _private_key: &str,
                _issue_time: i64,
            ) -> Result<String> {
                Ok("token".to_string())
            }

            fn get_installation_id(_token: &str, _org: &str, _base_url: &str) -> Result<String> {
                Ok(r#"{"id": 2342234, "access_tokens_url": "test url"}"#.to_string())
            }

            fn get_access_token(_access_token_url: &str, _token: &str) -> Result<String> {
                Ok(
                    r#"{"token": "access token", "expires_at": "2022-02-16T21:34:13Z"}"#
                        .to_string(),
                )
            }
        }

        let app_id = "234233";
        let private_key = "base64 encoded key";
        let org = "org";
        let base_url = "https://api.github.com";
        let issue_time = 1645121374;

        let expected_result: serde_json::Value = json!({
            "access_token": "access token",
            "expiration": "2022-02-16T21:34:13Z",
            "access_token_url": "test url",
            "installation_id": 2342234
        });

        let result = run(
            MockHttpSend,
            app_id,
            private_key,
            org,
            base_url,
            issue_time,
        );

        match result {
            Ok(d) => assert_eq!(
                serde_json::from_str::<serde_json::Value>(&d).unwrap(),
                expected_result
            ),
            err => panic!("should have been Ok, got {:#?}", err),
        };
    }
}
