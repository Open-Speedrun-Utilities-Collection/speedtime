//! Generic Transport and Timing server

mod single_usr_host;

use async_trait::async_trait;

pub use single_usr_host::SimpleBackendHost;

use crate::{
    transport::{STTransBackend, STTransSocket, STTransStream},
    STTransResult,
};

#[async_trait]
/// Backend trait for Speedrun Timing Server Host
pub trait STTransBackendHost {
    /// Initialize a backend object
    //async fn new() -> Result<&'back Self, Box<dyn Error>>;
    /// Bind a socket of generic type
    async fn bind_socket(
        &self,
        back: &dyn STTransBackend<dyn STTransSocket, dyn STTransStream>,
    ) -> STTransResult<()>;

    // Release a socket of generic type
    //async fn release_socket(&self, sock: impl STTransSocket) -> Result<(), Box<dyn Error>>;

    //async fn send(&self, msg: &str) -> Result<(), Box<dyn Error>>;
}
