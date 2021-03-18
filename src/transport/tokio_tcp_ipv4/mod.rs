//! Speedrun Timing Server Transport module for TCP over IPv4

mod socket;
mod stream;

use super::{STTransBackend, STTransSocket, STTransStream};
use crate::STTransResult;
use async_trait::async_trait;
use derive_more::Constructor;

pub use socket::*;
pub use stream::*;

#[derive(Debug, Constructor)]
/// Implements TCP transport over IPv4 through the STTTransBackend trait
pub struct TcpIpv4Backend {
    sockets: Vec<TcpIpv4Socket>,
}

#[async_trait]
impl STTransBackend<TcpIpv4Socket, TcpIpv4Stream> for TcpIpv4Backend {
    async fn bind_socket(&self, _sock: TcpIpv4Socket) -> STTransResult<()> {
        todo!()
    }

    async fn release_socket(&self, _sock: TcpIpv4Socket) -> STTransResult<()> {
        todo!()
    }

    async fn send(&self, _msg: &str) -> STTransResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    const TEST_ADDR: (Ipv4Addr, u16) = (Ipv4Addr::new(127, 0, 0, 1), 0);
}
