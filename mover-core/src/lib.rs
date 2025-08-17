//! Core types and traits for the mover automation library

pub mod error;
pub mod platform;
pub mod types;

pub use error::*;
pub use platform::*;
pub use types::*;

/// Re-export common types
pub mod prelude {
    pub use crate::error::*;
    pub use crate::platform::*;
    pub use crate::types::*;
}
