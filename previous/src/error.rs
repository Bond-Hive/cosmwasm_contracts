use cosmwasm_std::StdError;

#[derive(Debug, PartialEq)]
pub enum ContractError {
    Std(StdError),
    InvalidAmount,
    NotInitialized,
    AlreadyInitialized,
    MaturityReached,
    MaturityNotReached,
    NotOpenYet,
    QuoteRequired,
    AvailableRedemptionNotSet,
    AvailableRedemptionAlreadySet,
    ContractStopped,
    QuoteStillValid,
    QuoteChanged,
    QuoteExpired,
    Unauthorized,
}

impl From<StdError> for ContractError {
    fn from(error: StdError) -> Self {
        ContractError::Std(error)
    }
}
