use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum TokenLaunchpadError {
    #[error("Insufficient SOL to pay for the transaction.")]
    InsufficientBalance = 6000,
    #[error("The amount must be available in the curve .")]
    InvalidAmount = 6001,
    #[error("The slippage must be under 100 percent.")]
    InvalidSlippage = 6002,
    #[error("The cost amount is not in the allowed slippage interval.")]
    SlippageOverflow = 6003,
    #[error("Threshold limit exceeded.")]
    ThresholdReached = 6004,
    #[error("Trade disabled, market cap threshold reached.")]
    InvalidTokenAccount = 6005,
    #[error("Invalid curve account.")]
    InvalidCurveAccount = 6006,
    #[error("Invalid fee account address.")]
    InvalidFeeAccount = 6007,
    #[error("Curve limit exceeded.")]
    CurveLimit = 6008,
    #[error("Invalid curve type.")]
    InvalidCurveType = 6009,
    #[error("Invalid currency.")]
    InvalidCurrency = 6010,
    #[error("Artithmetics error")]
    Arithmetics = 6011,
    #[error("Market Cap threshold not hit, cannot migrate funds yet")]
    ThresholdNotHit = 6012,
    #[error("Invalid Authority provided.")]
    InvalidAuthority = 6013,
    #[error("Trade amount too low , resulting in 0 costs")]
    TradeAmountTooLow = 6014,
    #[error("Config field needs to be present during initialization")]
    ConfigFieldMissing = 6015,
    #[error("Unsupported different currency types")]
    DifferentCurrencies = 6016,
    #[error("Basis points too high")]
    BasisPointTooHigh = 6017,
    #[error("Fee share too High")]
    FeeShareTooHigh = 6018,
    #[error("Token decimals are not within the supported range")]
    TokenDecimalsOutOfRange = 6019,
    #[error("Token Name too long, max supported length is 32 bytes")]
    TokenNameTooLong = 6020,
    #[error("Token Symbol too long, max supported length is 10 bytes")]
    TokenSymbolTooLong = 6021,
    #[error("Token URI too long, max supported length is 200 bytes")]
    TokenUriTooLong = 6022,
    #[error("Minimum Decimal Places cannot be lower than Maximum Decimal Places")]
    IncorrectDecimalPlacesBounds = 6023,
    #[error("Minimum Token Supply cannot be lower than Maximum Token Supply")]
    IncorrectTokenSupplyBounds = 6024,
    #[error("Token Total Supply out of bounds")]
    TotalSupplyOutOfBounds = 6025,
    #[error(
        "This setup will produce final collateral amount less than the migration fee"
    )]
    FinalCollateralTooLow = 6026,
    #[error("One of the Coefficients is equal to ZERO")]
    CoefficientZero = 6027,
    #[error("Market cap Threshold under the Hard lower bound limits")]
    MarketCapThresholdTooLow = 6028,
    #[error("Default coef_b set out of hard limit bounds")]
    CoefBOutofBounds = 6029,
    #[error("General error")]
    General = 6030,
}
impl From<TokenLaunchpadError> for ProgramError {
    fn from(e: TokenLaunchpadError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for TokenLaunchpadError {
    fn type_of() -> &'static str {
        "TokenLaunchpadError"
    }
}
impl PrintProgramError for TokenLaunchpadError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
