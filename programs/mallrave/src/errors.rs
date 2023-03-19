use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The text size you want to enter is too long")]LenghtError,
    #[msg("BalanceError")]BalanceError,
    #[msg("PubkeyError")]PubkeyError,
}
