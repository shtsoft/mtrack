//! This module defines some utility functions to run a server with TLS.

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

/// The time a connection is allowed to upgrade to TLS.
const TLS_TIMEOUT: Duration = Duration::from_millis(5000);

/// Loads TLS certificates from a file.
/// - `filename` is the file containing the certificates.
///
/// # Errors
///
/// An error is returned if opening the file fails.
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

/// Loads a TLS key from a file.
/// - `filename` is the file containing the key.
///
/// # Errors
///
/// An error is returned if opening the file fails or if reading from the file fails.
pub fn load_key(filename: &str) -> Result<PrivateKeyDer<'static>, pem::Error> {
    PrivateKeyDer::from_pem_file(filename)
}

/// Runs a server on each TLS connection it establishes.
/// - `server` is the server which is run.
/// - `addr` is the socket address of the listener of the TLS connection.
/// - `state` is the state of the server.
/// - `acceptor` is the TLS acceptor upgrading the TCP socket to a TLS connection.
///
/// # Errors
///
/// An error is returned if binding to `addr` fails or accepting a connection fails.
///
/// # Notes
///
/// This function only returns in the error case.
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
                            tracing::info!("Added TLS for connection from {}", addr);

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
                                "Failed to add TLS for connection from {}: {:?}",
                                addr,
                                err
                            );
                        }
                    };
                }
                future::Either::Right(_) => {
                    tracing::warn!("Negotiating TLS for connection from {} timed out", addr);
                }
            }
        });

        id += 1;
    }
}

/// Errors which can occur when handling tasks.
#[derive(Debug, Error)]
pub enum HandleError {
    #[error("{}", self)]
    IO(#[from] tokio::io::Error),
    #[error("{}", self)]
    Join(#[from] task::JoinError),
}

/// Runs a task until an interrupt signal is received aborting the task.
/// - `handle_name` is the task name.
/// - `handle` is the task handle.
///
/// # Errors
///
/// An error is returned if there is an underlying I/O error or if the task failed to execute to completion.
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
