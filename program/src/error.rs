//! Program error types.

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that can be returned by the Solana BPF Loader v4 program.
#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum LoaderV4Error {
    /// This is a placeholder error.
    #[error("This is a placeholder error")]
    Placeholder,
}

impl PrintProgramError for LoaderV4Error {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<LoaderV4Error> for ProgramError {
    fn from(e: LoaderV4Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for LoaderV4Error {
    fn type_of() -> &'static str {
        "LoaderV4Error"
    }
}
