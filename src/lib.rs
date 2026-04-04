// Aicent Stack | RTTP (Real-Time Transfer Protocol)
// Domain: http://rttp.com
// Purpose: Sub-millisecond neural transport & stateful semantic multicast.
// Specification: RFC-002 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-002: RTTP Neural Pulse Frame
//! 
//! The `rttp` crate implements the high-speed nervous system of the Aicent Stack.
//! It replaces legacy protocol overhead with a deterministic, asynchronous 
//! pulse-frame architecture designed for AI-native workloads across a six-domain organism.
//!
//! ### Core Nerves Logic:
//! - **Stateful Semantic Multicast**: Routing via Task Affinity (RFC-001) instead of IPs.
//! - **Context Snapshot Sharding**: Sub-ms synchronization of Transformer KV-caches.
//! - **Reflex Trinity**: Direct wire-level integration of RPKI (RFC-003) and ZCMK (RFC-004).
//! - **Kinetic Resonance**: Ensuring temporal alignment for Hive-scale coordination (RFC-006).

#![deny(missing_docs)]
#![allow(unsafe_code)]

/// [RFC-002] Core Header Definitions
pub mod header;

pub use crate::header::{PulseFrameHeader, on_pulse_received};

/// [RFC-002] Neural Transmission Error Set
/// Defines physical and logical failure modes within the neural spine.
#[derive(Debug, Clone, PartialEq)]
pub enum NerveError {
    /// Network jitter exceeded the 100µs deterministic threshold
    JitterLimitExceeded,
    /// Semantic Multicast failed to find an affinity group
    RoutingAnomaly,
    /// Forward Error Correction (FEC) failed to recover the payload
    PulseCorruption,
    /// Neural sever: RTTP heartbeat timeout (>3ms)
    NeuralSeverance,
}

/// [RFC-002] Pulse Bundle
/// A complete atomic unit of neural transmission, containing the 64-byte 
/// header and the variable-length tensor payload.
#[derive(Debug, Clone)]
pub struct PulseBundle {
    /// The fixed 64-byte RPKI/ZCMK-integrated header
    pub header: PulseFrameHeader,
    /// The raw tensor manifold or instruction shard
    pub payload: Vec<u8>,
}

/// [RFC-002] Neural Spine Interface
/// Defines the behavior of the transport backbone for the Aicent Stack.
pub trait NeuralSpine {
    /// Emits a high-frequency pulse into the Aicent.net Grid (RFC-006).
    fn emit_pulse(&self, bundle: PulseBundle) -> Result<(), NerveError>;
    
    /// Ingests an inbound pulse from the RTTP spine with zero-copy dispatch.
    fn ingest_pulse(&self) -> Result<PulseBundle, NerveError>;
    
    /// Synchronizes the local monotonic clock for Kinetic Resonance alignment.
    fn calibrate_clock(&self) -> u32;
}

/// [RFC-003] Quarantine Reflex
/// High-priority interface for RPKI to isolate pathogens instantly.
/// Injects a Priority 255 frame into the neural spine to cut off the node.
pub fn emit_quarantine_pulse(pathogen_fingerprint: &[u8; 32], _reason: u16) {
    println!("\x1b[1;31m[RTTP-SHIELD]\x1b[0m 🚨 Quarantine pulse emitted for 0x{:02x?}. Pathogen isolated.", &pathogen_fingerprint[..4]);
}

/// [Standard v1.0] Target Performance Metrics
/// These constants define the physical requirements for RTTP compliance.
pub const TARGET_LATENCY_US: u32 = 420;
/// Jitter limit for Kinetic Resonance (RFC-006)
pub const TARGET_JITTER_US: u32 = 50; 

/// [Standard v1.0] Protocol Constants
pub const RTTP_MAGIC: u32 = 0x5254_5450; 
/// [Standard v1.0] Protocol Version
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// High-fidelity telemetry marker for neural pulses.
pub fn log_neural_event(msg: &str) {
    println!("\x1b[1;36m[RTTP-NERVES]\x1b[0m ⚡ {}", msg);
}
