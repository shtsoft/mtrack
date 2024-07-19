use mtrack::Config;

use std::net::Ipv4Addr;
use std::process;

use clap::{ArgAction, Parser};

/// mtrack serves the GPS information it receives
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Make logging verbose
    #[arg(long, short, action=ArgAction::SetTrue)]
    verbose: bool,
    /// IP address the server uses
    #[arg(long, default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    ip: Ipv4Addr,
    /// Port the server uses
    #[arg(short, long, default_value_t = 10443)]
    port: u16,
    /// Path to TLS certificate the server uses
    #[arg(short, long)]
    cert: String,
    /// Path to TLS key the server uses
    #[arg(short, long)]
    key: String,
    /// Path to upload users db the server uses
    #[arg(short, long)]
    upload_users: String,
    /// Path to download users db the server uses
    #[arg(short, long)]
    download_users: String,
}

fn main() {
    let args = Args::parse();
    let args = mtrack::Args {
        verbose: args.verbose,
        ip: args.ip,
        port: args.port,
        cert: args.cert,
        key: args.key,
        upload_users: args.upload_users,
        download_users: args.download_users,
    };

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Setting up application failed: {:?}", err);
        process::exit(1);
    });

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            eprintln!("Setting up tokio runtime failed: {:?}", err);
            process::exit(1);
        }
    };

    if let Err(err) = rt.block_on(mtrack::run(config)) {
        eprintln!("Running application failed: {:?}", err);
        process::exit(1);
    }

    process::exit(0);
}
