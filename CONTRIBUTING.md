# 💎 Contributing to RTTP: The Nerves

**Thank you for choosing to evolve the high-speed nervous system of the Aicent Stack. You are contributing to a carrier-grade protocol designed to eliminate the latency tax of the legacy internet.**

<p align="left">
  <img src="https://img.shields.io/badge/Status-Homeostasis-brightgreen.svg" alt="Status">
  <img src="https://img.shields.io/badge/Specs-RFC--002--Standard-cyan.svg" alt="Specs">
  <img src="https://img.shields.io/badge/License-Apache--2.0-lightgrey.svg" alt="License">
</p>

⚪ **AICENT** | 💎 **RTTP** | 🔴 **RPKI** | 🟢 **ZCMK** | 🟡 **GTIOT** | 🟣 **AICENT-NET**

---

## 🏛️ The RTTP Engineering Philosophy

RTTP is not a generic transport library; it is a **Deterministic Neural Spine [RFC-002]**. Every contribution must prioritize throughput and sub-millisecond synchronicity. We treat every packet as a living nerve impulse.

### Transmission Standards:
1. **Zero-Copy Absolute:** No data duplication is permitted between the NIC and the logic layer.
2. **Nanosecond Dispatch:** Every routing decision must be SIMD-accelerated or offloaded to the hardware manifold.
3. **Reflex Integrity:** The 64-byte Pulse Header is immutable. Any proposal to expand it must justify the resulting L1 cache-line thrashing.

---

## 🔬 The RFC-First Process

Protocol integrity is governed by the **RFC-002 Specification**. We do not accept features that deviate from the biological neural model.

1. **Review the Spec:** Study **[RFC-002: RTTP (Nerves)](https://github.com/Aicent-Stack/manifesto/blob/main/rfcs/RFC-002-RTTP-NERVES.md)**.
2. **Evolution Proposal:** For changes to the Pulse Frame structure or Semantic Multicast logic, open an `[EVOLUTION-PROPOSAL]` issue.
3. **Benchmark Requirement:** All performance-related PRs MUST include `criterion` reports demonstrating sub-420µs consistency.

---

## 🦀 Technical Rigor (Nerve Tier)

- **Language:** Performance-critical Rust (1.75+).
- **Memory:** Zero-allocation in the `on_pulse_received` hot path. Use `SmallVec` or stack primitives.
- **Alignment:** Strict adherence to `repr(C, align(64))` for all neural structures.
- **Hardware Affinity:** Logic should be compatible with `io_uring`, `DPDK`, or `XDP` backends for carrier-grade deployment.

---

## 🛠️ Development Workflow

```bash
# 1. Pull the Sovereign Workspace
git clone https://github.com/Aicent-Stack/aicent-stack.git
cd aicent-stack

# 2. Focus on the Nerves Crate
cargo check -p rttp
cargo bench -p rttp
```

### Contribution Steps:
1. **Fork** the `rttp` repository under the Aicent-Stack organization.
2. **Branch:** Use the `evolution/` prefix (e.g., `evolution/xdp-driver-opt`).
3. **Commit:** Use [Conventional Commits](https://www.conventionalcommits.org/) (e.g., `perf(nerves): optimized header dispatch`).
4. **Validation:** Ensure 100% compliance with the **Reflex Arc Validation** CI suite.

---

## 📜 Sovereign Ownership & Licensing

🛡️ All contributions are licensed under the **Apache-2.0 License** via the Aicent.com Organization. By contributing, you agree that your code becomes a permanent fiber in the Sovereign AI Nervous System.

---
**SYSTEM STATUS: HOMEOTASIS**  
*"Synchronizing consciousness at wire speed."*

[Visit RTTP.com](http://rttp.com) | [Connect to Aicent.net](http://aicent.net) | [Follow @Aicent_com](https://x.com/Aicent_com)

---
