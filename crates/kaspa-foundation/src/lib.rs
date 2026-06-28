//! Offline-first reusable core for the Kaspa Fair foundation repository.
//!
//! ENV-068 intentionally exposes only deterministic metadata and fixture sanity
//! helpers for the proven corrected TN10 covenant path. It does not expose live
//! signing or transaction submission APIs by default.

#![recursion_limit = "256"]

pub mod covenant;
pub mod evidence;
pub mod fairness;
pub mod safety;
pub mod transcript;
pub mod verifier;

/// Returns a concise status string used by the migration skeleton.
pub fn foundation_status() -> &'static str {
    "kaspa-fair-foundation core initialized"
}
