use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum VirtualXykError {
    #[error("Unauthorized fee withdrawal")]
    InvalidFeeAuthority = 6000,
    #[error("The fee amount is invalid")]
    InvalidFeeAmount = 6001,
    #[error("Slippage exceeded")]
    SlippageExceeded = 6002,
}
impl From<VirtualXykError> for ProgramError {
    fn from(e: VirtualXykError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for VirtualXykError {
    fn type_of() -> &'static str {
        "VirtualXykError"
    }
}
impl PrintProgramError for VirtualXykError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
