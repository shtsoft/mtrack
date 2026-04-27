//! This module defines utility functions for running a server with TLS.

use std::fmt::Debug;
use std::future::Future;
use std::net::SocketAddr;

use futures::future;

use thiserror::Error;

use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::Duration;

use tokio_rustls::rustls::pki_types::pem;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

use tracing::{Instrument, Span};

/// The maximum time allowed for a connection to complete the TLS handshake.
const TLS_TIMEOUT: Duration = Duration::from_millis(5000);

/// Loads TLS certificates from a PEM file.
/// - `filename` is the path to the file containing the certificates.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or parsed.
pub fn load_certs(
    filename: &str,
) -> Result<Vec<CertificateDer<'static>>, Box<dyn std::error::Error>> {
    let mut certs = Vec::new();
    for result in CertificateDer::pem_file_iter(filename)? {
        match result {
            Ok(cert) => certs.push(cert),
            Err(err) => return Err(Box::new(err)),
        }
    }
    Ok(certs)
}

/// Loads a TLS private key from a PEM file.
/// - `filename` is the path to the file containing the key.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or if the key is invalid.
pub fn load_key(filename: &str) -> Result<PrivateKeyDer<'static>, pem::Error> {
    PrivateKeyDer::from_pem_file(filename)
}

/// Runs the server and handles incoming TLS connections.
/// - `server` is the service function to run for each connection.
/// - `addr` is the socket address to listen on.
/// - `state` is the shared application state passed to the server.
/// - `acceptor` is the TLS acceptor used to upgrade TCP streams.
///
/// # Errors
///
/// Returns an error if binding to the address or accepting a connection fails.
///
/// # Notes
///
/// This function runs indefinitely and only returns if an error occurs.
pub async fn serve<S, F, State: Clone + Send + 'static>(
    server: S,
    addr: SocketAddr,
    state: State,
    acceptor: TlsAcceptor,
) -> tokio::io::Result<()>
where
    S: Fn(TlsStream<TcpStream>, State) -> F + Clone + Send + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);

    let mut id: u64 = 0;
    loop {
        let server = server.clone();
        let state = state.clone();

        let (socket, addr) = listener.accept().await?;
        tracing::info!("Accepted connection from {}", addr);

        let tls_stream = acceptor.accept(socket);
        let timer = time::sleep(TLS_TIMEOUT);

        task::spawn(async move {
            futures::pin_mut!(tls_stream);
            futures::pin_mut!(timer);
            match future::select(tls_stream, timer).await {
                future::Either::Left((result, _)) => {
                    match result {
                        Ok(tls_socket) => {
                            tracing::info!("Established TLS connection from {}", addr);

                            let span = tracing::error_span!(
                                "service",
                                "connection-id" = id,
                                "client-address" = %addr,
                            );
                            span.follows_from(Span::current());
                            server(tls_socket, state).instrument(span).await;
                        }
                        Err(err) => {
                            tracing::error!(
                                "Failed to establish TLS for connection from {}: {:?}",
                                addr,
                                err
                            );
                        }
                    };
                }
                future::Either::Right(_) => {
                    tracing::warn!("TLS handshake for connection from {} timed out", addr);
                }
            }
        });

        id += 1;
    }
}

/// Errors that can occur when managing tasks.
#[derive(Debug, Error)]
pub enum HandleError {
    #[error("{}", self)]
    IO(#[from] tokio::io::Error),
    #[error("{}", self)]
    Join(#[from] task::JoinError),
}

/// Runs a task until an interrupt signal (Ctrl+C) is received, then aborts the task.
/// - `handle_name` is a descriptive name for the task.
/// - `handle` is the join handle of the running task.
///
/// # Errors
///
/// Returns an error if an I/O issue occurs or if the task fails to join.
pub async fn sigint_abort<T: Send>(
    handle_name: &str,
    handle: JoinHandle<T>,
) -> Result<Option<T>, HandleError> {
    let sigint = signal::ctrl_c();

    futures::pin_mut!(handle);
    futures::pin_mut!(sigint);
    match future::select(handle, sigint).await {
        future::Either::Left((result, _)) => {
            let t = result?;
            Ok(Some(t))
        }
        future::Either::Right((result, handle)) => {
            handle.abort();
            tracing::info!("Aborted {}", handle_name);
            result?;
            Ok(None)
        }
    }
}
