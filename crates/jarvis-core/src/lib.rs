//! jarvis-core — shared project foundations.
//!
//! This crate intentionally contains no subsystem logic (no wake word,
//! STT, TTS, AI, memory, automation, device control, or plugins). It
//! exists only to give future modules a common set of identifiers,
//! an event mechanism, and an error type to build on.

pub mod error;
pub mod events;
pub mod types;

pub use error::CoreError;
pub use events::CoreEvent;
pub use types::{CommandId, Confidence};
