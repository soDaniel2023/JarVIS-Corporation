//! Shared identifiers and value types.
//!
//! These types carry no behavior beyond basic construction and access.
//! They exist so future modules refer to the same concepts (a command,
//! a confidence score) in the same way, instead of each inventing its
//! own equivalent type.

/// Identifies a command by its declared id (e.g. from a command pack).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandId(String);

impl CommandId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A confidence score, guaranteed to be within `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Confidence(f32);

impl Confidence {
    /// Returns `None` if `value` is outside `0.0..=1.0`.
    pub fn new(value: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
