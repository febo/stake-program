//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StakeError {
    /// 0 - Amount cannot be greater than zero
    #[error("Amount cannot be greater than zero")]
    AmountGreaterThanZero = 0x0,
}

impl solana_program::program_error::PrintProgramError for StakeError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
