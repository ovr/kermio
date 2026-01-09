use std::fmt;

/// Error types for Hermes JavaScript runtime operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// JavaScript evaluation error (e.g., syntax error, runtime error, thrown exception)
    EvaluationError(String),

    /// JavaScript compilation error (when compiling to bytecode)
    CompilationError(String),

    /// Invalid bytecode format or corrupted bytecode
    InvalidBytecode(String),

    /// JavaScript preparation error (when preparing code for repeated execution)
    PreparationError(String),

    /// Runtime creation or configuration error
    RuntimeError(String),

    /// Generic error from the underlying C++ implementation
    InternalError(String),
}

impl Error {
    /// Create an evaluation error
    pub fn evaluation<S: Into<String>>(msg: S) -> Self {
        Error::EvaluationError(msg.into())
    }

    /// Create a compilation error
    pub fn compilation<S: Into<String>>(msg: S) -> Self {
        Error::CompilationError(msg.into())
    }

    /// Create an invalid bytecode error
    pub fn invalid_bytecode<S: Into<String>>(msg: S) -> Self {
        Error::InvalidBytecode(msg.into())
    }

    /// Create a preparation error
    pub fn preparation<S: Into<String>>(msg: S) -> Self {
        Error::PreparationError(msg.into())
    }

    /// Create a runtime error
    pub fn runtime<S: Into<String>>(msg: S) -> Self {
        Error::RuntimeError(msg.into())
    }

    /// Create an internal error
    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Error::InternalError(msg.into())
    }

    /// Check if the error message contains a substring
    pub fn contains(&self, needle: &str) -> bool {
        match self {
            Error::EvaluationError(msg)
            | Error::CompilationError(msg)
            | Error::InvalidBytecode(msg)
            | Error::PreparationError(msg)
            | Error::RuntimeError(msg)
            | Error::InternalError(msg) => msg.contains(needle),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            Error::EvaluationError(msg)
            | Error::CompilationError(msg)
            | Error::InvalidBytecode(msg)
            | Error::PreparationError(msg)
            | Error::RuntimeError(msg)
            | Error::InternalError(msg) => msg,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EvaluationError(msg) => write!(f, "JavaScript evaluation error: {}", msg),
            Error::CompilationError(msg) => write!(f, "JavaScript compilation error: {}", msg),
            Error::InvalidBytecode(msg) => write!(f, "Invalid bytecode: {}", msg),
            Error::PreparationError(msg) => write!(f, "JavaScript preparation error: {}", msg),
            Error::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            Error::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Specialized Result type for Hermes operations
pub type Result<T> = std::result::Result<T, Error>;

/// Convert CXX errors to Hermes errors
impl From<cxx::Exception> for Error {
    fn from(e: cxx::Exception) -> Self {
        let msg = e.to_string();

        // Try to categorize the error based on the message
        // Check for JavaScript runtime errors (exceptions thrown in JS)
        if msg.contains("JSError") {
            Error::EvaluationError(msg)
        }
        // Check for compilation/parsing errors
        else if msg.contains("Compiling JS failed")
            || msg.contains("compilation")
            || msg.contains("Failed to compile")
            || msg.contains("expected")
        {
            Error::EvaluationError(msg)
        }
        // Check for bytecode-related errors
        else if msg.contains("bytecode") || msg.contains("Invalid bytecode") {
            Error::InvalidBytecode(msg)
        }
        // Everything else is an internal error
        else {
            Error::InternalError(msg)
        }
    }
}
