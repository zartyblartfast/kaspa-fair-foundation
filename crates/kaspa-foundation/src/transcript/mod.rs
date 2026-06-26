//! Stable proof transcript models for offline verification and evidence reuse.

pub mod canonical;
pub mod online_verifier;
pub mod schema;
pub mod verifier;

pub use canonical::{canonical_tn10_proof_transcript, CanonicalTranscriptValues, ProofTranscript};
pub use schema::{
    StepMode, StepPurpose, TranscriptSafetyBoundary, TranscriptStep, EVIDENCE_SCHEMA_VERSION,
    TRANSCRIPT_SCHEMA_VERSION,
};
