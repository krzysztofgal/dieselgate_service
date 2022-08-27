#[derive(Debug, thiserror::Error)]
pub enum DieselUsageCalculationError {
    #[error("Invalid Calculation Parameters: {0}")]
    InvalidParams(String),
    #[error("Internal Error, Invalid configuration")]
    Unimplemented,
}

#[derive(Debug, thiserror::Error)]
pub enum UnitInjectorFailCalculationError {
    #[error("Invalid Calculation Parameters: {0}")]
    InvalidParams(String),
    #[error("Internal Error, Invalid configuration")]
    Unimplemented,
}
