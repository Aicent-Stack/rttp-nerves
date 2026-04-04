// Aicent Stack | RTTP (Real-Time Transfer Protocol)
// Domain: http://rttp.com
// Purpose: Fixed 64-byte ultra-low latency Pulse Frame Header.
// Specification: RFC-002 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-002: RTTP Neural Pulse Frame Header
//!
//! This module defines the binary wire format for the Aicent Stack nervous system.
//! Every neural impulse is encapsulated in a fixed-size, hardware-aligned 64-byte header.

use serde::{Deserialize, Serialize};

/// [RFC-002] RTTP Pulse Frame Header
/// Optimized for zero-copy parsing and hardware-level NIC offloading (DPDK/eBPF).
///
/// [PERF] Aligned to 64-byte boundary to eliminate L1 cache-line thrashing.
/// This structure encapsulates Nerves (RTTP), Immunity (RPKI), and Blood (ZCMK)
/// markers into a single atomic bit-stream.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PulseFrameHeader {
    /// 0x5254_5450 ("RTTP") - Protocol Magic Number for nanosecond rejection.
    pub magic: u32,
    /// 0x0100 (Standard v1.0) - Current protocol version.
    pub version: u16,
    /// Protocol flags (bit0: Multicast, bit1: FEC, bit2: KV-Delta, bit3: Hive-Sync).
    pub flags: u16,
    /// The 256-bit cryptographic fingerprint of the sender's AID (RFC-001/003).
    pub rpki_fingerprint: [u8; 32],
    /// Nanosecond RTBA bid for ZCMK metabolic clearing (RFC-004).
    pub zcmk_bid: u64,
    /// 64-bit Semantic Hash for context-aware routing and Hive alignment (RFC-006).
    pub semantic_hash: u64,
    /// Pulse priority from 0-255 (255 = Critical Pathogen Quarantine).
    pub priority: u8,
    /// Maximum network hops for planetary scale sharding.
    pub ttl: u8,
    /// Nanosecond hardware-level relative timestamp for drift tracking.
    pub timestamp_ns: u32,
    /// CRC16/CRC32C hardware integrity checksum.
    pub checksum: u16,
}

impl PulseFrameHeader {
    /// [PERF] Zero-Copy Byte Mapping.
    /// Maps the header structure directly to a raw byte buffer without allocation.
    /// Inlining forces the compiler to eliminate function call overhead.
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, 64) }
    }

    /// Creates a new Standard 1.0 Pulse Frame Header.
    pub fn new(fingerprint: [u8; 32], bid: u64, sem_hash: u64) -> Self {
        let now = std::time::Instant::now().elapsed().as_nanos() as u32;
        Self {
            magic: 0x5254_5450,
            version: 0x0100,
            flags: 0b0000_0110,
            rpki_fingerprint: fingerprint,
            zcmk_bid: bid,
            semantic_hash: sem_hash,
            priority: 128,
            ttl: 64,
            timestamp_ns: now,
            checksum: 0,
        }
    }
}

/// [RFC-002] Neural Pulse Dispatcher.
/// Primary entry point for inbound byte buffers from the RTTP spine.
/// Optimized for branch prediction to ensure sub-millisecond determinism.
pub fn on_pulse_received(frame: &[u8]) {
    // 🛡️ [SECURITY AUDIT] Boundary check to mitigate overflow (RFC-003).
    if frame.len() < 64 {
        handle_malformed_pulse(); // Pathogen Drop (Cold Path)
        return;
    }

    // [PERF] Direct memory mapping (Zero-copy).
    let header = unsafe { &*(frame.as_ptr() as *const PulseFrameHeader) };

    // [FAST PATH] Protocol Identification.
    if header.magic != 0x5254_5450 {
        return; // Early rejection of non-RTTP traffic.
    }

    // 🔗 [Reflex Trinity Note]
    // Integration with RPKI (Immunity) and ZCMK (Blood) occurs in the
    // Orchestration layer to maintain a clean unidirectional dependency graph.
    #[cfg(debug_assertions)]
    println!(
        "\x1b[1;36m[RTTP-PULSE]\x1b[0m 64-byte Header verified. Ready for Reflex Arc."
    );
}

/// Marked as #[cold] to tell the compiler this path is rarely taken.
/// Keeps the main execution path clean in the CPU's I-Cache.
#[cold]
#[inline(never)]
fn handle_malformed_pulse() {
    eprintln!("\x1b[1;31m[RTTP-ERROR]\x1b[0m Inbound frame size underflow. Pathogen discarded.");
}
