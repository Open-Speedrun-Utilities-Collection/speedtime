//! TCP over IPv4 stream wrapper using tokio

use super::{STTransResult, STTransSocket, STTransStream, TcpIpv4Socket};
use async_trait::async_trait;
use derive_more::Constructor;

#[derive(Debug, Constructor)]
/// Tcp steam type for the TCP over IPv4 backend of STTransserver
pub struct TcpIpv4Stream {
    pub socket: TcpIpv4Socket,
}

#[async_trait]
impl STTransStream for TcpIpv4Stream {
    async fn send(&self, _msg: &str) -> STTransResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
