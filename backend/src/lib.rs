pub mod app;

pub mod utils;

use app::server;
use app::AppState;

use utils::{load_certs, load_key, serve, sigint_abort};

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV6};
use std::sync::Arc;
use std::sync::RwLock;

use tokio::task;

use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

use tracing::subscriber;
use tracing::{Instrument, Level};

const NAME: &str = "mtrack";

const KEY: &str = "1234";

pub struct Args {
    pub verbose: bool,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub cert: String,
    pub key: String,
}

pub struct Config {
    level: Level,
    addr: SocketAddr,
    server_config: ServerConfig,
}

impl Config {
    pub fn new(args: Args) -> Result<Self, Box<dyn std::error::Error>> {
        let level = if args.verbose {
            Level::TRACE
        } else {
            Level::INFO
        };

        let addr = SocketAddr::V6(SocketAddrV6::new(
            Ipv4Addr::to_ipv6_mapped(&args.ip),
            args.port,
            0,
            0,
        ));

        let server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(load_certs(&args.cert)?, load_key(&args.key)?)?;

        Ok(Self {
            level,
            addr,
            server_config,
        })
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(config.level)
        .finish();
    let _ = subscriber::set_global_default(subscriber);

    let state = Arc::new(RwLock::new(AppState {
        key: KEY.to_string(),
    }));

    let handle = task::spawn(
        serve(
            server,
            config.addr,
            Arc::clone(&state),
            TlsAcceptor::from(Arc::new(config.server_config)),
        )
        .instrument(tracing::error_span!(NAME)),
    );

    let server = sigint_abort(NAME, handle).await;

    tracing::info!("{} server is down", NAME);

    if let Some(t) = server? {
        t?;
    }

    Ok(())
}
