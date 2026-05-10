/*
 * Copyright (c) 2026 Joseph Woodall IV
 * This software is licensed under the PolyForm Noncommercial License 1.0.0.
 * You may not use this file or the resulting binaries for any commercial purposes.
 * The trained model weights (.tssm files) associated with this project 
 * are licensed under the Creative Commons Attribution-NonCommercial 4.0 
 * International License (CC BY-NC 4.0).
 */

# NORTH STAR: OCTOPUS OS

## Our Mission
To engineer a strictly autonomous, self-governing Agentic Operating System. It operates entirely without human intervention, running natively within the Linux ecosystem on constrained consumer hardware (Intel Core Ultra, 16GB RAM). We reject cloud dependency, brute-force scaling, and Python-based inference in favor of bare-metal structural efficiency, eBPF sensory integration, and dynamic memory orchestration.

## The Architecture: The Octopus Paradigm
The OS abandons the monolithic "generalist" model. It is a highly optimized hive of 1.58-bit micro-experts natively communicating with the Linux kernel.

1. **The Central Ganglion (Main Brain):** A lightweight, continuously running temporal TSSM. It monitors kernel state via eBPF, updates its continuous SSM memory, evaluates threat/action criticality, and orchestrates tasks.
2. **The Sensory Tentacles (eBPF):** Rust-compiled eBPF probes injected directly into Ring 0 kernel tracepoints. They feed live file, network, and scheduling data to the Ganglion via lock-free ring buffers.
3. **The Arms (Micro-Experts):** Specialized models (e.g., `coder.tssm`, `net_sec.tssm`) stored as compressed binary files. They are memory-mapped (`mmap`) into RAM on the fly, executed on isolated threads, and instantly flushed upon completion.

## The Technical Pillars
* **Compute:** 1.58-bit Ternary Weights. Inference relies entirely on AVX-512/AMX integer SIMD operations. Zero floating-point multiplication.
* **Memory:** Linear State Space Models (SSM). Zero $O(N^2)$ Transformer attention. Memory footprint remains strictly $O(1)$, enabling infinite context monitoring.
* **Execution:** Bare-metal Rust. Zero Python runtime. Sensory input via `eBPF`; system output via non-blocking `io_uring`.

## The Core Principles (The Arbiter)
The system is governed by a deterministic, hardcoded Rust validation layer. The AI cannot bypass these absolute laws:
1. **The Preservation Principle:** The OS cannot alter, delete, or overwrite host binaries or protected directories (e.g., `/boot`, `/etc`, `/bin`).
2. **The Parasite Principle:** The OS cannot exhaust host resources. The Arbiter blocks the Ganglion from spawning an Arm if system RAM falls below 2GB or CPU thermal limits are breached.
3. **The Containment Principle:** The OS cannot open arbitrary network sockets. All external communication is restricted to a hardcoded whitelist evaluated before `io_uring` execution.

*"The architecture is the skeleton; the arbiter is the law; the kernel is the soul."*