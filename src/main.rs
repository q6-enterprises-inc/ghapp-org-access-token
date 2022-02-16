use clap::Parser;
use ghapp_org_access_token::httpsend::{HttpSend, run};
use anyhow::Result;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    struct Output;

    impl HttpSend for Output {}

    let result = run(Output, args.app_id, args.private_key_path, args.org)?;

    println!("{}", result);

    Ok(())
}
