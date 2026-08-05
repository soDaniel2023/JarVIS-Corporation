//! The shared event mechanism.
//!
//! `CoreEvent` intentionally has no variants yet. No subsystem exists in
//! this crate to produce events, so inventing placeholder variants now
//! would be guessing at future needs. Modules that introduce real
//! behavior (wake word, STT, commands, etc.) add their own variants
//! here when they are built.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum CoreEvent {}
