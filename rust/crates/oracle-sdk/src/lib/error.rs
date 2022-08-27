use thiserror::Error;

#[derive(Debug, Error)]
pub enum OracleError {
    #[error("Unable to parse '{0}': error: {1}")]
    UnableToParse(String, String),
    #[error("Unable to read file '{0}', error: {1}")]
    UnableToReadFile(String, String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
    #[error("Instruction execution error : {0}")]
    InstructionExecutionError(String),
    #[error("Sending transaction failed : {0}")]
    TransactionError(String),
    #[error("Fetching resource failed at account : {0}")]
    FetchError(String),
}

#[allow(dead_code)]
impl OracleError {
    pub fn to_str(&self) -> &'static str {
        match self {
            OracleError::UnableToParse(_, _) => "UnableToParse",
            OracleError::UnableToReadFile(_, _) => "UnableToReadFile",
            OracleError::UnexpectedError(_) => "UnexpectedError",
            OracleError::InstructionExecutionError(_) => "InstructionExecutionError",
            OracleError::TransactionError(_) => "TransactionError",
            OracleError::FetchError(_) => "FetchError",
        }
    }
}

/// A common result to remove need for typing `Result<T, OracleError>`
pub type OracleTypedResult<T> = Result<T, OracleError>;
