pub mod config;
pub mod stake;

pub use config::*;
pub use stake::*;

/// Discriminator of an uninitialized account.
pub const UNINITIALIZED: [u8; 8] = [0u8; 8];
