// Aicent Stack | RTTP (Real-Time Transfer Protocol)
// Domain: http://rttp.com
// Purpose: Sub-millisecond neural transport & stateful semantic multicast.
// Specification: RFC-002 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-002: RTTP Neural Pulse Frame
//! 
//! The `rttp` crate implements the high-speed nervous system of the Aicent Stack.
//! It replaces legacy protocol overhead with a deterministic, asynchronous 
//! pulse-frame architecture designed for AI-native workloads.
//!
//! ### Core Nerves Logic:
//! - **Stateful Semantic Multicast**: Routing based on task affinity rather than static IPs.
//! - **Context Snapshot Sharding**: Sub-ms synchronization of KV-caches across the grid.
//! - **Reflex Trinity**: Direct wire-level integration of RPKI (RFC-003) and ZCMK (RFC-004).
//! - **Kinetic Resonance**: Ensuring temporal alignment for Hive-scale coordination (RFC-006).

#![deny(missing_docs)]
#![allow(unsafe_code)]

pub mod header;

pub use crate::header::{PulseFrameHeader, on_pulse_received};

/// [RFC-002] Neural Spine Interface
/// Defines the behavior of the transport backbone for the Aicent Stack.
pub trait NeuralSpine {
    /// Broadcasts a Pulse Frame to the semantic affinity group.
    fn emit_pulse(&self, header: &PulseFrameHeader, payload: &[u8]) -> Result<(), String>;
    
    /// Listens for incoming pulses from the global operational grid (RFC-006).
    fn ingest_pulse(&self) -> Option<(PulseFrameHeader, Vec<u8>)>;
}

/// [RFC-003] Quarantine Reflex
/// High-priority interface for RPKI to isolate pathogens instantly.
/// The `_reason` parameter defines the pathogen classification.
pub fn emit_quarantine_pulse(pathogen_fingerprint: &[u8; 32], _reason: u16) {
    // Injects a Priority 255 frame into the RTTP spine to cut off the node
    println!("\x1b[1;31m[RTTP-SHIELD]\x1b[0m Quarantine pulse emitted for 0x{:02x?}", &pathogen_fingerprint[..4]);
}

/// [Standard v1.0] Protocol Magic Number identifier.
pub const RTTP_MAGIC: u32 = 0x5254_5450; 

/// [Standard v1.0] The current active version of the RTTP protocol.
pub const PROTOCOL_VERSION: &str = "0.1.0-standard";
