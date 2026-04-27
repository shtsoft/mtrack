use mtrack::Config;

use std::net::Ipv4Addr;
use std::process;

use clap::{ArgAction, Parser};

/// mtrack serves the GPS information it receives
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Enable verbose logging
    #[arg(long, short, action=ArgAction::SetTrue)]
    verbose: bool,
    /// IP address for the server
    #[arg(long, default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    ip: Ipv4Addr,
    /// Port for the server
    #[arg(short, long, default_value_t = 10443)]
    port: u16,
    /// Path to the TLS certificate
    #[arg(short, long)]
    cert: String,
    /// Path to the TLS key
    #[arg(short, long)]
    key: String,
    /// Path to the upload users database
    #[arg(short, long)]
    upload_users: String,
    /// Path to the download users database
    #[arg(short, long)]
    download_users: String,
    /// Path to the frontend distribution directory
    #[arg(long)]
    dist: String,
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
        dist: args.dist,
    };

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Application setup failed: {:?}", err);
        process::exit(1);
    });

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            eprintln!("Tokio runtime setup failed: {:?}", err);
            process::exit(1);
        }
    };

    if let Err(err) = rt.block_on(mtrack::run(config)) {
        eprintln!("Application execution failed: {:?}", err);
        process::exit(1);
    }

    process::exit(0);
}
