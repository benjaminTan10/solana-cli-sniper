use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum FeeShareError {
    #[error("Unauthorized")]
    Unauthorized = 6000,
    #[error("Redeem amount exceeds available fees")]
    InsufficientFunds = 6001,
}
impl From<FeeShareError> for ProgramError {
    fn from(e: FeeShareError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FeeShareError {
    fn type_of() -> &'static str {
        "FeeShareError"
    }
}
impl PrintProgramError for FeeShareError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
