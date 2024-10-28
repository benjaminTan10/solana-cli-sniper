use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum FundraiseError {
    #[error("Invalid token mint")]
    InvalidTokenMint = 6000,
    #[error("Invalid funding mint")]
    InvalidFundingMint = 6001,
    #[error("Already funded")]
    AlreadyFunded = 6002,
    #[error("Cannot redeem")]
    CannotRedeem = 6003,
    #[error("Expired")]
    Expired = 6004,
    #[error("Unauthorized")]
    Unauthorized = 6005,
    #[error("Not funded")]
    NotFunded = 6006,
}
impl From<FundraiseError> for ProgramError {
    fn from(e: FundraiseError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FundraiseError {
    fn type_of() -> &'static str {
        "FundraiseError"
    }
}
impl PrintProgramError for FundraiseError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
