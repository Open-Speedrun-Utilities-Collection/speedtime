//! OSTU Speedrun Timing Library

mod server;
mod transport;

use std::error::Error;
use std::io;
use std::result::Result;

/// Crate result type
pub type STTransResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
pub type STTransIoResult<T> = Result<T, Box<io::Error>>;
