use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Error {
    #[error("Value out of range")]
    OutOfRange,
    #[error("Conversion/casting error")]
    Conversion,
    #[error("Unknown type in the type subsystem")]
    UnknowType,
    #[error("Type mismatch")]
    MisMatchType,
    #[error("Division by zero")]
    DivideByZero,
    #[error("Incompatible type")]
    InCompatibleType,
    #[error("Out of memory error")]
    OutOfMemory,
}