use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::net::SocketAddr;

use futures::future;

use rustls_pemfile::Item;

use thiserror::Error;

use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::mpsc;
use tokio::task;
use tokio::task::JoinHandle;

use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

use tracing::{Instrument, Span};

pub fn load_certs(filename: &str) -> std::io::Result<Vec<CertificateDer<'static>>> {
    let mut reader = BufReader::new(File::open(filename)?);
    rustls_pemfile::certs(&mut reader).collect()
}

pub fn load_key(filename: &str) -> std::io::Result<PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(File::open(filename)?);
    loop {
        match rustls_pemfile::read_one(&mut reader)? {
            Some(Item::Pkcs1Key(key)) => return Ok(key.into()),
            Some(Item::Pkcs8Key(key)) => return Ok(key.into()),
            Some(Item::Sec1Key(key)) => return Ok(key.into()),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No keys found",
                ))
            }
            _ => {}
        }
    }
}

#[derive(Debug, Error)]
pub enum ServeError<E: Error> {
    #[error("{}", self)]
    IO(#[from] tokio::io::Error),
    #[error("{}", self)]
    Send(#[from] mpsc::error::SendError<JoinHandle<Result<(), E>>>),
}

pub async fn serve<S, F, State: Clone + Send>(
    server: S,
    addr: SocketAddr,
    state: State,
    acceptor: TlsAcceptor,
) -> tokio::io::Result<()>
where
    S: Fn(TlsStream<TcpStream>, State) -> F + Send,
    F: Future<Output = ()> + Send + 'static,
{
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);

    let mut id: u64 = 0;
    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::info!("Accepted connection from {}", addr);

        let tls_socket = match acceptor.accept(socket).await {
            Ok(tls_socket) => tls_socket,
            Err(err) => {
                tracing::error!("Failed to add TLS for connection from {}: {:?}", addr, err);
                continue;
            }
        };
        tracing::info!("Added TLS for connection from {}", addr);

        let span = tracing::error_span!("service", "connection-id" = id, "client-address" = %addr);
        span.follows_from(Span::current());
        task::spawn(server(tls_socket, state.clone()).instrument(span));

        id += 1;
    }
}

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("{}", self)]
    IO(#[from] tokio::io::Error),
    #[error("{}", self)]
    Join(#[from] task::JoinError),
}

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
