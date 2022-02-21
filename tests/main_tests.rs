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
            "--private-key",
            "LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQpNSUlFb1FJQkFBS0NBUUJJSVI3dm1YbG1XUG1FdkRWWklid1czOGpMdWk2SjhsT1d6UThVMzJIMndsc3J6cC9BCklQVnBNV3Q5WUU2SVdnajlTSlE2QjBSa0k3OXRUVW5uZkJNSmZUNERPd0JtVlRZclZ3eHp3U3BIZXpLVGMxQmkKakxiNXF5R2pucjBmS3l5Q01vd1hoTThKcC9GN04vTVkxbEEzemF1a1h5RjBqWE5TTVNTeThiellWWW5jaEs3NQp4U1pGUXYrNTVqODVnbWxZekhWNmI4S2pXL2FudEJKZ0J6R3hnN3BUSElydzJMSUxIeENLUy9TdnI0dGE5dlFDClVxTURWMXR1VUIxY0dPUXkrakhEVFJub3M0YlBkSmZhRUZQZUVodGJoQm5XMklIS3lkL0taaHN0eWRQdGV4ODcKcGtHa0E4a2RzV0ZMZk9sdkRCbEs5WEJ5c3M4blJJdldZVVVOQWdNQkFBRUNnZ0VBTStPMzZMQ2xXRTdMK29TWApjRzdIYkVGRHArUmgxSldETXVxRVhnU3g2aDQ2RHBMUDlCMEJ6OUpndk1xSzFLYkk3c1hHOU5RRGpITDNKTC9FCldnYTFmMDhkVEorbWt2ZmNSMk9STHJrWkhHRnhxZ2Y4SFZtVHpKc2pVUjFDZXVqSlZVVGQ2WkYrakRqSWpWZGIKeUZOdFZDZmV3aG81N0NwU2V4K2xnaUpNTStqWi9KQVJFbWZHTHhPVExnWmh4a3NQK2Z1elFOM0tuUjdhMGFjRgppbFg1ZHhDR0lHSVFSazNmNnVPb3RabjdQSDQ0TzdFZjRTZGVSSlZVN3hwSjEyWjZnZ0ZvSUVkL3pZQ3ZFVm1oCmw4Smo0SThzSkxEZ3hYaW01dDFyUFRRRUh2MEJwcXQ4V1JtYlRZMzVpVWo5WFZhVDhQZ0VpM2lqbE5CbVFBNFMKM080T1ZRS0JnUUNLME4zd0ZUVk0vaDBEZC82MURXOUpGM0ZrbThWU2l6VnFlSW1kVEpaMVc2UGQ3a0I4cXlZbApOZXJZNFFBcGJzS1pUMHNBMi9wZE5VOVN1MG9TU0xHQnJtTmozaHNLU3ZtYUdyUmhEYi9zZ2tqalFMK1RkYzBvClNUYnZJNVhJaUhrUWNmOEozZzgvOTNaMkRTSDc2U2FwVFJTQW1RU3QrNE5wNUp4bkZPQXpkd0tCZ1FDRkJNODYKSFY2ZEFmWjlRdXdWNlJaOFQvUHVOVWc0ZDF3cTNqNzdKRTdjTG1EQ0xlWGFYenBLQlNWdHd6OHovS2pKVDcrMwplWGY3Qm9mb3RGUzdpL3pZd3JlMVU3akF1SUpMQUE4SW9iQ21QZHRHWXNBeDROUGEwMVJCOEN0WXY3V01ibDJSClYzUlVXUUhhZlJLRjFoOXlIZCtSRXdxQjhNTzBtVGtITDEzRW13S0JnQXp0VEcycUcrK1Nqbk1mUG9udnVWaE8KSlpJZjYyMDNaMzhGd0pMMGFlSjk2VEdtbVR6QzEyZUpzTmdIZy9OWHpDbG90K3haaitRVUxkSGZmUk5jejZMcQo4WGlBVzNaRzZ4Mk50UlNBcWtuRkRES1ZlZ0dxYTdOL3RlenBIRzZ6UHNyb1FyN1JmZ3dQNkdrdjZlVkNuZ1lRCkZvT3ZQRDlUZDdacHdxTGF6OUliQW9HQURjSlltUjlPbVJrSjRSMGFCSTR1dUQ4ZzhVUHJxNE9WTWczNUg4czYKcVFYWEZsN2ZCcjZRN2ZVb1VQbEFRV1ZpSkNpdW9SRGNlMGZLcVNSMkcwdzRwRWRINzJhd2xxYS9PM2pQRlVwOApWa1hSdDY4aFNFZUVRbjlYbU5aNTlWMG5MMVovTVRpRm5PLzBCQ0NwMk5RMFBuNmVrSWdTcnl3ellpdnpQUzRHCkdmOENnWUJ0VCtpL1lhZEU0bTFvUURBMVJxUit1Wk9hZkl2Z3BEOVg0TW1iNjdZZWVHNVlOTkpqTFg0S0dDd3AKVFZFNGhSWjE0ZjQwQVcwSjFlTHBLcTZ6alg3OHlFdGM5M0w3aFZBaERlQkZhZkJ6ZUkwVDVqZ2djNHVucWphNgpzbm1rNHFUMXM0L2FRQmhsQ2g2RDBWNTNTMDlSeDR2cktPRVd5WTJMZ2gvUm1jOUxkdz09Ci0tLS0tRU5EIFJTQSBQUklWQVRFIEtFWS0tLS0tCg==",
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
