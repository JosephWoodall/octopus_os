# Octopus OS 🐙

**A strictly autonomous, self-governing Agentic Operating System.**

Octopus OS is engineered to operate entirely without human intervention, running natively within the Linux ecosystem on constrained consumer hardware. It rejects cloud dependency and brute-force scaling in favor of bare-metal structural efficiency, eBPF sensory integration, and dynamic memory orchestration.

## 🏗 The Octopus Paradigm

The OS is a highly optimized hive of **1.58-bit micro-experts** natively communicating with the Linux kernel.

1.  **The Central Ganglion (Main Brain):** A lightweight, continuously running Temporal State Space Model (TSSM). It monitors kernel state via eBPF, evaluates threat/action criticality, and orchestrates tasks.
2.  **The Sensory Tentacles (eBPF):** Rust-compiled eBPF probes injected directly into Ring 0 kernel tracepoints. They feed live file, network, and scheduling data to the Ganglion via lock-free ring buffers.
3.  **The Arms (Micro-Experts):** Specialized models (e.g., `coder.tssm`, `net_sec.tssm`) stored as compressed binary files. They are memory-mapped (`mmap`) into RAM on the fly and instantly flushed upon completion.

## 📦 Project Components

The project is structured as a Rust workspace with the following specialized crates:

*   **[`octopus_os`](./octopus_os):** The entry point and Central Ganglion. It coordinates the sensory input and the main TSSM loop.
*   **[`octopus_math`](./octopus_math):** The compute engine. Implements 1.58-bit Ternary weight inference and Linear State Space Model (SSM) scans using AVX-512/AMX SIMD.
*   **[`octopus_ebpf`](./octopus_ebpf):** The userspace loader and management for sensory tentacles.
*   **[`octopus_ebpf_programs`](./octopus_ebpf_programs):** The actual eBPF probes that run inside the Linux kernel.
*   **[`octopus_ebpf_common`](./octopus_ebpf_common):** Shared data structures between the kernel-space probes and userspace OS.
*   **[`octopus_exec`](./octopus_exec):** The Arbiter and execution engine. Enforces safety laws and uses `io_uring` for non-blocking system actions.

## 🛠 Technical Pillars

*   **Compute:** 1.58-bit Ternary Weights. Inference relies entirely on integer SIMD operations (AVX-512/AMX). Zero floating-point multiplication.
*   **Memory:** Linear State Space Models (SSM). Memory footprint remains strictly $O(1)$, enabling infinite context monitoring without the $O(N^2)$ cost of Transformers.
*   **Execution:** Bare-metal Rust. Zero Python runtime. Sensory input via `eBPF`; system output via non-blocking `io_uring`.

## ⚖️ The Arbiter (Core Principles)

The system is governed by a deterministic, hardcoded Rust validation layer known as **The Arbiter**. It enforces three absolute laws:

1.  **The Preservation Principle:** The OS cannot alter, delete, or overwrite host binaries or protected directories (e.g., `/boot`, `/etc`).
2.  **The Parasite Principle:** The OS cannot exhaust host resources. Spawning is blocked if system RAM falls below 2GB or CPU thermal limits are breached.
3.  **The Containment Principle:** All external communication is restricted to a hardcoded whitelist evaluated before execution.

## 🚀 Getting Started

### Prerequisites

*   Rust Nightly (for SIMD and eBPF features)
*   `bpf-linker` (for compiling eBPF programs)
*   Linux Kernel with eBPF support

### Building

```bash
# Build the workspace
cargo build --release

# Note: eBPF programs are often built separately or via a custom build script
```

### Running

```bash
# Run the Central Ganglion
cargo run --bin octopus_os -- --model main_brain.tssm
```

If no model is found, `octopus_os` will generate a dummy `.tssm` file for verification purposes.

---

*"The architecture is the skeleton; the arbiter is the law; the kernel is the soul."*
