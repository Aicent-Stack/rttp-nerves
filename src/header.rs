// Aicent Stack | RTTP (Real-Time Transfer Protocol)
// Domain: http://rttp.com
// Purpose: Fixed 64-byte ultra-low latency Pulse Frame Header.
// Specification: RFC-002 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-002: RTTP Neural Pulse Frame Header

use serde::{Serialize, Deserialize};

#[repr(C, align(64))] 
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PulseFrameHeader {
    pub magic: u32,              
    pub version: u16,            
    pub flags: u16,              
    pub rpki_fingerprint: [u8; 32],
    pub zcmk_bid: u64,
    pub semantic_hash: u64,
    pub priority: u8,             
    pub ttl: u8,                  
    pub timestamp_ns: u32,        
    pub checksum: u16,            
}

impl PulseFrameHeader {
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, 64) }
    }
}

pub fn on_pulse_received(frame: &[u8]) {
    if frame.len() < 64 { return; }
    let header = unsafe { &*(frame.as_ptr() as *const PulseFrameHeader) };
    if header.magic != 0x5254_5450 { return; }

    // 🛡️ [Reflex Trinity Note]
    // Verification (RPKI) and Clearing (ZCMK) happen at the Orchestration layer
    // to maintain a clean unidirectional dependency graph.
}
