//! mtrack is a position tracking app based on getting and posting positions via http requests.
//!
//! ## Design
//!
//! The idea is that a predefined set of users can upload their current position with a post request.
//! The uploaded positions can then be downloaded with a get request by another set of predefined users.
//! Both uploading and downloading is password protected and a frontend is served to allow easy uploading and downloading.

pub mod app;

pub mod utils;

use app::server;
use app::{AppState, State, UserEntry};

use utils::{load_certs, load_key, serve, sigint_abort};

use std::collections::HashMap;
use std::fs;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV6};
use std::sync::Arc;
use std::sync::RwLock;

use tokio::task;

use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

use tracing::subscriber;
use tracing::{Instrument, Level};

/// The name of the application.
const NAME: &str = "mtrack";

/// The names of the pages the application serves (except for "home").
const PAGE_NAMES: [&str; 3] = ["login", "postpos", "tracker"];

/// Abstracts the command line parameters.
pub struct Args {
    pub verbose: bool,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub cert: String,
    pub key: String,
    pub upload_users: String,
    pub download_users: String,
    pub dist: String,
}

/// Abstracts the configuration of the application.
pub struct Config {
    /// maximal tracing level of the application
    level: Level,
    /// socket address to which the application binds
    addr: SocketAddr,
    /// TLS server config used for connections
    server_config: ServerConfig,
    /// database of the users who can upload their positions
    upload_users: Vec<UserEntry>,
    /// database of the users who can download uploaded positions
    download_users: Vec<UserEntry>,
    /// path to the frontend distribution
    dist: String,
}

impl Config {
    /// Creates a new configuration of the application.
    /// - `args` are the commandline parameters.
    ///
    ///  # Errors
    ///
    ///  An error is returned if making the TLS server config fails or if loading one of the user databases fails.
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

        let upload_users: Vec<UserEntry> =
            serde_json::from_str(&fs::read_to_string(args.upload_users)?)?;

        let download_users: Vec<UserEntry> =
            serde_json::from_str(&fs::read_to_string(args.download_users)?)?;

        let dist = args.dist;

        Ok(Self {
            level,
            addr,
            server_config,
            upload_users,
            download_users,
            dist,
        })
    }
}

/// Runs the application.
/// - `config` is the configuration the application is run in.
///
///  # Errors
///
///  An error is returned if loading the pages fails or if there is a problem with the aborted server.
pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fn load_pages<'a>(
        dist: &str,
    ) -> Result<HashMap<&'a str, String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut pages = HashMap::new();

        pages.insert(
            "home",
            fs::read_to_string(dist.to_string() + "/index.html")?,
        );

        for name in PAGE_NAMES {
            let page = fs::read_to_string(dist.to_string() + &format!("/{}/index.html", name))?;
            pages.insert(name, page);
        }

        Ok(pages)
    }

    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(config.level)
        .finish();
    let _ = subscriber::set_global_default(subscriber);

    let app_state = Arc::new(RwLock::new(AppState {
        sessions: HashMap::with_capacity(config.download_users.len()),
        positions: HashMap::with_capacity(config.upload_users.len()),
        download_users: config.download_users,
        upload_users: config.upload_users,
        pages: load_pages(&config.dist)?,
    }));
    let state = State {
        app_state: Arc::clone(&app_state),
        dist: config.dist,
    };

    let handle = task::spawn(
        serve(
            server,
            config.addr,
            state,
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
