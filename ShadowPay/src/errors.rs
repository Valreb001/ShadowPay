use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    InvalidProof = 1,
    RecipientNotVerified = 2,
    InsufficientFunds = 3,
    ZeroAddress = 4,
    UnauthorizedCaller = 5,
    InvalidAmount = 6,
    AlreadyInitialized = 7,
    ContractPaused = 8,
    AnchorNotFound = 9,
    SettlementFailed = 10,
}
