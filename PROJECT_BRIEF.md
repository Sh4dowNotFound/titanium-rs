# Titanium-rs Project Brief

**Project Name**: Titanium-rs
**Repository**: [https://github.com/Sh4dowNotFound/titanium-rs](https://github.com/Sh4dowNotFound/titanium-rs)
**License**: AGPL-3.0-or-later
**Version**: 0.1.0 (Alpha)

## Overview
Titanium-rs is an ultra-high-performance, concurrent Discord library written in Rust, engineered for massive scale. It is designed to handle millions of guilds with minimal memory footprint and maximum throughput. Unlike traditional libraries, Titanium prioritizes zero-copy parsing, allocation-free hot paths, and strict safety guarantees.

## Core Value Proposition
- **Performance First**: Built on top of `simd-json` for AVX2/SSE4.2 accelerated parsing and `mimalloc` for optimized memory allocation.
- **Zero-Copy Architecture**: Uses advanced Rust lifetimes and `Cow` (Clone-on-Write) semantics to avoid unnecessary data copying during Gateway event processing.
- **Safety**:Strictly enforced `#![deny(unsafe_code)]` policy (with documented SIMD exceptions), ensuring memory safety without sacrificing speed.
- **Modularity**: Split into focused crates (`titanium-gateway`, `titanium-voice`, `titanium-model`, `titanium-cache`) to allow users to select only what they need.

## Target Audience
- Developers of massive-scale Discord bots (100k+ guilds).
- Systems engineers requiring low-level control over Discord infrastructure.
- Rust developers seeking a modern, async-first alternative to existing libraries.

## Key Features
1.  **Titanium Gateway**: specialized WebSocket client with transparent zlib-stream support, auto-sharding, and ETF (Erlang Term Format) compatibility.
2.  **Titanium Voice**: Zero-allocation voice packet encryption using in-place buffer operations for high-density voice nodes.
3.  **Titanium Cache**: Lock-free concurrent caching implementation based on `DashMap` for highly contentious reads/writes.
