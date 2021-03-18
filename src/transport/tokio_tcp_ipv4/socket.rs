use super::STTransSocket;
use crate::{STTransIoResult, STTransResult};
use async_trait::async_trait;
use derive_more::Constructor;
use log::debug;
use std::io;
use tokio::net::TcpListener;

#[derive(Debug, Constructor)]
/// Tcp socket type for the Tcp over IPv4 Backend of STTransServer
pub struct TcpIpv4Socket {
    pub socket: Option<TcpListener>,
}

#[async_trait]
impl STTransSocket for TcpIpv4Socket {
    async fn bind_socket(&mut self, addr: &str) -> STTransIoResult<()> {
        debug!(
            "Attempting to bind socket {:?} to address {}",
            self.socket, addr
        );

        // Check if the socket already exists
        if self.socket.is_some() {}

        match self.socket {
            Some(_) => {
                // Get reference to error message in static memory
                const ERR_MSG: &str = "Attempted to bind to address that is already in use";

                // Output Error
                debug!("{}", ERR_MSG);
                Err(Box::new(io::Error::new(io::ErrorKind::AddrInUse, ERR_MSG)))
            }
            None => {
                // Bind socket
                self.socket = Some(TcpListener::bind(addr).await?);
                Ok(())
            }
        }
    }

    async fn accept_connections(&self) -> STTransResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Str reference to the standard IPv4 loopback address in static memory
    /// Note: Uses port 0 - Currently assumes it causes the OS to assign open port automagically
    ///
    /// This assumption is valid with the tokio::net implementation at time of writing
    /// TODO: Doctest this assumption
    const IPV4_LOOPBACK: &str = "127.0.0.1:0";

    #[tokio::test]
    /// Tests if the current interface for socket::bind is correct
    async fn bind_socket_interface_correct() -> STTransIoResult<()> {
        let mut sock = TcpIpv4Socket::new(None);
        sock.bind_socket(IPV4_LOOPBACK).await
    }

    #[tokio::test]
    /// Tests if socket fails when attempting to bind an address to it twice.
    ///
    /// Note: The heavy amount of boilerplate in this macro is cased by
    /// not being about to use #[should_fail] in async tokio tests
    ///
    async fn bind_socket_panic_on_double_bind() -> STTransIoResult<()> {
        // Create Socket
        let mut sock = TcpIpv4Socket::new(None);

        // Bind socket
        let bind1 = sock.bind_socket(IPV4_LOOPBACK).await;

        assert!(bind1.is_ok());

        // Bind socket again
        let bind2 = sock.bind_socket(IPV4_LOOPBACK).await;

        assert!(bind2.is_err());

        Ok(())
    }

    #[tokio::test]
    /// Tests if the interface for  STTransSocket::accept_connections returns without panicing
    async fn accept_connections_interface_correct() -> STTransIoResult<()> {
        let sock = TcpIpv4Socket::new(None);

        let sock = sock.accept_connections().await;

        assert!(sock.is_ok());

        Ok(())
    }
}
