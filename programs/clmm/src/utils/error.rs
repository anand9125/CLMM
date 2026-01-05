use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("InsufficentAmount")]
    InsufficentAmount,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("InvalidPositionRange")]
    InvalidPositionRange,
    #[msg("InvalidMint")]
    InvalidMint,
    #[msg("InvalidRange")]
    InvalidTickRange,
    #[msg("InvalidRange")]
    InvalidRange,
    #[msg("MintRangeMustCoverCurrentPrice")]
    MintRangeMustCoverCurrentPrice,
    #[msg("NoLiquidityToRemove")]
    NoLiquidityToRemove


}