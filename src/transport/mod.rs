//! Data transfer backend module

#[cfg(feature = "tokio_transport_backend_tcpipv4")]
/// Data transfer backend for TCP over IPv4
mod tokio_tcp_ipv4;

use crate::{STTransIoResult, STTransResult};
use async_trait::async_trait;

#[async_trait]
/// Backend trait for Speedrun Timing Server Host
pub trait STTransBackend<SOCK: STTransSocket + Send, STREAM: STTransStream + Send> {
    /// Initialize a backend object
    //async fn new() -> Result<&'back Self, Box<dyn Error>>;
    /// Bind a socket of generic type
    async fn bind_socket(&self, sock: SOCK) -> STTransResult<()>;
    /// Release a socket of generic type
    async fn release_socket(&self, sock: SOCK) -> STTransResult<()>;

    async fn send(&self, msg: &str) -> STTransResult<()>;
}

#[async_trait]
/// Backend trait wrapping a Backend Socket generically
pub trait STTransSocket {
    async fn bind_socket(&mut self, addr: &str) -> STTransIoResult<()>;
    async fn accept_connections(&self) -> STTransResult<()>;
}

#[async_trait]
/// Backend trait wrapping a backend stream generically
pub trait STTransStream {
    async fn send(&self, msg: &str) -> STTransResult<()>;
}
