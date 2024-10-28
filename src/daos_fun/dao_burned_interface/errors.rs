use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DaoBurnedError {
    #[error("Invalid carry basis")]
    InitInvalidCarryBasis = 6000,
    #[error("Fundraise must be finalized")]
    InitCurveFundraiseNotFinalized = 6001,
    #[error("Curve already initialized")]
    InitCurveAlreadyInitialized = 6002,
    #[error("Redemption not started")]
    RedeemNotStarted = 6003,
    #[error("Redemption already started")]
    RedeemAlreadyStarted = 6004,
    #[error("User redeemed more than they have")]
    RedeemExceedOwned = 6005,
    #[error("Fund is not expired")]
    RedeemFundNotExpired = 6006,
    #[error("No more delegate authorities allowed")]
    DelegateExceededMax = 6007,
    #[error("Authority not found")]
    DelegateAuthorityNotFound = 6008,
    #[error("Authority already exists")]
    DelegateDuplicateAuthority = 6009,
    #[error("Unauthorized")]
    Unauthorized = 6010,
    #[error("Fund is expired")]
    ExecuteFundExpired = 6011,
    #[error("Fund is closed")]
    ExecuteFundClosed = 6012,
}
impl From<DaoBurnedError> for ProgramError {
    fn from(e: DaoBurnedError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for DaoBurnedError {
    fn type_of() -> &'static str {
        "DaoBurnedError"
    }
}
impl PrintProgramError for DaoBurnedError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
