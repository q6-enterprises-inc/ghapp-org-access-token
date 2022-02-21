use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use ghapp_org_access_token::httpsend::{run, HttpSend};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Github app id.
    #[clap(short, long)]
    app_id: String,

    /// Base64 encoded Github App private key.
    #[clap(short, long)]
    private_key: String,

    /// Organization name as it appears in the github url, i.e. https://github.com/my-org/my-repo.
    #[clap(short, long)]
    org: String,

    /// Github API fully qualified base url. Remember to include 'http://'!
    #[clap(short, long, default_value = "https://api.github.com")]
    base_url: String,

    /// Epoch time in seconds, defaults to current Epoch.
    #[clap(short, long, default_value_t=Utc::now().checked_add_signed(chrono::Duration::seconds(-10)).expect("valid timestamp").timestamp())]
    issue_time: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    struct Output;

    impl HttpSend for Output {}

    let result = run(
        Output,
        &args.app_id,
        &args.private_key,
        &args.org,
        &args.base_url,
        args.issue_time,
    )?;

    println!("{}", result);

    Ok(())
}
