use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    Api {
        status: u32,
        message: String,
    },
    Network {
        message: String,
    },
    Validation {
        field: String,
        message: String,
    },
    Storage {
        message: String,
    },
    Serialization {
        message: String,
    },
    Unknown {
        message: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Api { status, message } => {
                write!(f, "API Error ({}): {}", status, message)
            }
            AppError::Network { message } => {
                write!(f, "Network Error: {}", message)
            }
            AppError::Validation { field, message } => {
                write!(f, "Validation Error on {}: {}", field, message)
            }
            AppError::Storage { message } => {
                write!(f, "Storage Error: {}", message)
            }
            AppError::Serialization { message } => {
                write!(f, "Serialization Error: {}", message)
            }
            AppError::Unknown { message } => {
                write!(f, "Unknown Error: {}", message)
            }
        }
    }
}

impl AppError {
    pub fn api(status: u32, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
        }
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage {
            message: message.into(),
        }
    }

    pub fn serialization(message: impl Into<String>) -> Self {
        Self::Serialization {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn unknown(message: impl Into<String>) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn is_recoverable(&self) -> bool {
        matches!(self, AppError::Network { .. } | AppError::Api { .. })
    }

    #[allow(dead_code)]
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::Api { status, .. } if *status >= 500 => ErrorSeverity::Critical,
            AppError::Api { .. } => ErrorSeverity::Warning,
            AppError::Network { .. } => ErrorSeverity::Warning,
            AppError::Validation { .. } => ErrorSeverity::Info,
            AppError::Storage { .. } => ErrorSeverity::Error,
            AppError::Serialization { .. } => ErrorSeverity::Error,
            AppError::Unknown { .. } => ErrorSeverity::Critical,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

pub type AppResult<T> = Result<T, AppError>;

// Helper trait for converting common errors
pub trait IntoAppError<T> {
    fn into_app_error(self) -> AppResult<T>;
}

impl<T> IntoAppError<T> for Result<T, gloo_net::Error> {
    fn into_app_error(self) -> AppResult<T> {
        self.map_err(|e| AppError::network(format!("Network request failed: {:?}", e)))
    }
}

impl<T> IntoAppError<T> for Result<T, serde_json::Error> {
    fn into_app_error(self) -> AppResult<T> {
        self.map_err(|e| AppError::serialization(format!("JSON parsing failed: {:?}", e)))
    }
} 