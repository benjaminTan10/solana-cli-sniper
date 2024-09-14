use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JupiterError {
    #[error("Empty route")]
    EmptyRoute = 6000,
    #[error("Slippage tolerance exceeded")]
    SlippageToleranceExceeded = 6001,
    #[error("Invalid calculation")]
    InvalidCalculation = 6002,
    #[error("Missing platform fee account")]
    MissingPlatformFeeAccount = 6003,
    #[error("Invalid slippage")]
    InvalidSlippage = 6004,
    #[error("Not enough percent to 100")]
    NotEnoughPercent = 6005,
    #[error("Token input index is invalid")]
    InvalidInputIndex = 6006,
    #[error("Token output index is invalid")]
    InvalidOutputIndex = 6007,
    #[error("Not Enough Account keys")]
    NotEnoughAccountKeys = 6008,
    #[error("Non zero minimum out amount not supported")]
    NonZeroMinimumOutAmountNotSupported = 6009,
    #[error("Invalid route plan")]
    InvalidRoutePlan = 6010,
    #[error("Invalid referral authority")]
    InvalidReferralAuthority = 6011,
    #[error("Token account doesn't match the ledger")]
    LedgerTokenAccountDoesNotMatch = 6012,
    #[error("Invalid token ledger")]
    InvalidTokenLedger = 6013,
    #[error("Token program ID is invalid")]
    IncorrectTokenProgramId = 6014,
    #[error("Token program not provided")]
    TokenProgramNotProvided = 6015,
    #[error("Swap not supported")]
    SwapNotSupported = 6016,
    #[error("Exact out amount doesn't match")]
    ExactOutAmountNotMatched = 6017,
}
impl From<JupiterError> for ProgramError {
    fn from(e: JupiterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for JupiterError {
    fn type_of() -> &'static str {
        "JupiterError"
    }
}
impl PrintProgramError for JupiterError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
