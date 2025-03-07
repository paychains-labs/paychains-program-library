#![deny(missing_docs)]
//! Governance Chat program

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current sdk types for downstream users building with a different sdk version
pub use paychains_program;
