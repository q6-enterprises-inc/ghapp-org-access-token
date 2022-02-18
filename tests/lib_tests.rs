use assert_cmd::Command;
use serde_json::json;
use tokio;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn it_works_test() {
    let mock_server = MockServer::start().await;

    let installation_id_response: serde_json::Value = json!({
        "id": 23411111,
        "access_tokens_url": format!("{}/app/installations/23411111/access_tokens", &mock_server.uri())
    });

    let expected_token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpYXQiOjE2NDUxMjEzNzQsImV4cCI6MTY0NTEyMTk3NCwiaXNzIjoiMTIzNDM0MyJ9.Pc98ZwxuJBiU0aaOE5bl9Q2TnKFWWhAmQKQLVQ07g_3na55ujGd6LZJjOsNBgaQmaU9y4HqtgIuumgYVP_U-2hY3U_2EtI_pzhszYEMpkaOGHGahg4GF_kj86Bbw0Oj2EhBuQSn4oHP68qk-bqXEFue9jSbWduMn3s95nXbsvq1DdNtsNn3H-KrgZHrTO6CKu_XqYu0KZjYow88v1OWFiaYXqk8n-XjmvEvAdu6BHRJHy7tM6wgn_WbykelUNeYDy860GFdz_Gp5v9wdCo51LtnvmItqxDKjHcQVUyiB5TlX7Talnz3fjVo0STUwFsqlf3MSRW_-RK_PJpjcoFOfpw";

    let installation_id_response_template =
        ResponseTemplate::new(200).set_body_json(installation_id_response);

    Mock::given(method("GET"))
        .and(path("/orgs/test-inc/installation"))
        .and(header("Authorization", expected_token))
        .respond_with(installation_id_response_template)
        .mount(&mock_server)
        .await;

    let access_tokens_url_response: serde_json::Value = json!({
        "token": "access token",
        "expires_at": "2022-02-16T21:34:13Z"
    });

    let access_tokens_url_response_template =
        ResponseTemplate::new(200).set_body_json(access_tokens_url_response);

    Mock::given(method("POST"))
        .and(path("/app/installations/23411111/access_tokens"))
        .and(header("Authorization", expected_token))
        .respond_with(access_tokens_url_response_template)
        .mount(&mock_server)
        .await;

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd
        .args(&[
            "--app-id",
            "1234343",
            "--private-key-path",
            "tests/data/privkey-fake.pem",
            "--org",
            "test-inc",
            "--base-url",
            &mock_server.uri(),
            "--issue-time",
            "1645121374",
        ])
        .assert();

    assert.success();
}
