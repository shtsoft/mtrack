pub mod app;

pub mod utils;

use app::server;
use app::AppState;

use utils::{serve,load_certs, load_key, sigint_abort};

use std::env::Args;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV6};
use std::sync::Arc;
use std::sync::RwLock;

use tokio::task;

use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

use tracing::subscriber;
use tracing::{Instrument, Level};

const NAME: &str = "mtrack";

const LEVEL: Level = Level::INFO;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 10443;

const CERTFILE: &str = "example.pem";
const KEYFILE: &str = "example.key";

const KEY: &str = "1234";

pub struct Config {
    level: Level,
    addr: SocketAddr,
    server_config: ServerConfig,
}

impl Config {
    pub fn new(mut _args: Args) -> Result<Self, Box<dyn std::error::Error>> {
        let level = LEVEL;

        let addr = SocketAddr::V6(SocketAddrV6::new(Ipv4Addr::to_ipv6_mapped(&IP), PORT, 0, 0));

        let server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(load_certs(CERTFILE)?, load_key(KEYFILE)?)?;

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
