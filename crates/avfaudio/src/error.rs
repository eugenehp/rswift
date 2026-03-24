//! Error handling for AVFAudio using `anyhow`.
//!
//! Re-exports common `anyhow` types for convenient error handling.

pub use anyhow::{anyhow, bail, ensure, Context, Error};

/// Convenience alias for AVFAudio result type.
/// Defaults to `Result<(), Error>` for unit operations.
pub type AudioResult<T = ()> = anyhow::Result<T>;
