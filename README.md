# ghapp_org_access_token
`ghapp_org_access_token` gets an access token for an org level bot. The access token is then intended to be used with CI/CD to clone private dependencies. 
The next step will be to incorporate this into a github action.

## Install
- Follow these instructions to setup an org level bot: https://docs.github.com/en/developers/overview/managing-deploy-keys#server-to-server-tokens.
- Be sure you grant the bot the appropriate permissions.
- Record your bot's app id.
- Generate a private key for the app and download the key.
- Be sure that you have Rust and Cargo installed.
- Install the app:
```
git clone https://github.com/q6-enterprises-inc/ghapp_org_access_token.git &&\
cd ghapp_org_access_token &&\
cargo install --path . &&\
ghapp_org_access_token --help
```
## Usage
```
ghapp_org_access_token 0.1.0

USAGE:
    ghapp_org_access_token [OPTIONS] --app-id <APP_ID> --private-key-path <PRIVATE_KEY_PATH> --org <ORG>

OPTIONS:
    -a, --app-id <APP_ID>
            Github app id

    -b, --base-url <BASE_URL>
            Github API fully qualified base url. Remember to include 'http://'! [default:
            https://api.github.com]

    -h, --help
            Print help information

    -i, --issue-time <ISSUE_TIME>
            Epoch time in seconds, defaults to current Epoch [default: 1645126270]

    -o, --org <ORG>
            Organization name as it appears in the github url, i.e. https://github.com/my-org/my-
            repo

    -p, --private-key-path <PRIVATE_KEY_PATH>
            Relative path to the Github App private key

    -V, --version
            Print version information
```
## Output
If you have correctly created the app for your organization, `ghapp_org_access_token` will output something like this:
```
{
  "access_token": "ghs_1aaLvxCh12343uL23hvVmjJlXaij1c7yHM3Q",
  "expiration": "2022-02-12T21:24:58Z",
  "access_token_url": "https://api.github.com/app/installations/22228671/access_tokens",
  "installation_id": 22228671
}
```
