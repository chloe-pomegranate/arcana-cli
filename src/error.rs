//! Error types for Arcana

use thiserror::Error;

/// Unified error type for the Arcana application
#[derive(Error, Debug)]
pub enum ArcanaError {
    /// Wrapped IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Card lookup failed
    #[error("Card not found: '{0}'")]
    CardNotFound(String),

    /// Spread lookup failed
    #[error("Unknown spread: '{0}'")]
    UnknownSpread(String),

    /// No active reading to save
    #[error("No reading to save")]
    NoReadingToSave,

    /// TUI runtime error
    #[error("TUI error: {0}")]
    Tui(String),
}
