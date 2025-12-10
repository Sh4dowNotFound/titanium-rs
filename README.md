# Titanium-rs

[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Rust](https://github.com/Sh4dowNotFound/titanium-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Sh4dowNotFound/titanium-rs/actions)
[![Documentation](https://img.shields.io/badge/docs-titanium-brightgreen)](https://sh4downotfound.github.io/titanium-rs/titanium/index.html)

**Titanium-rs** is a high-performance, concurrent Discord library for Rust, designed for massive scale.

## Documentation
Full documentation is available at: [https://sh4downotfound.github.io/titanium-rs/](https://sh4downotfound.github.io/titanium-rs/titanium/index.html)

## Features

- **Titanium Gateway**: A robust, zero-copy, highly concurrent WebSocket client for the Discord Gateway.
    - Zero-copy JSON parsing (via `simd-json` when enabled).
    - Zlib-stream compression support.
    - specialized `mimalloc` support for high throughput.
- **Titanium Voice**: A voice client with zero-allocation packet encryption.
- **Titanium Model**: Comprehensive, zero-copy friendly data models for Discord API entities.
- **Titanium Cache**: High-performance concurrent cache based on `DashMap`.

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
titanium-rs = "0.1"
```

Then in your code:

```rust
use titanium::prelude::*;
```

## Optimization

To enable high-performance memory allocation:

```toml
[dependencies]
titanium-rs = { version = "0.1", features = ["performance"] }
```

## Safety

This project adheres to strict safety standards:
- `#![deny(unsafe_code)]` is enforced globally.
- Exceptions are essentially limited to SIMD optimizations and are strictly documented.

## Context for LLMs

If you are using AI tools (Cursor, Copilot, etc.), provide them with the following context files for optimal assistance:
- [General Project Brief](PROJECT_BRIEF.md)
- [Tech Stack Details](TECH_STACK.md)

## License

AGPL-3.0-or-later
