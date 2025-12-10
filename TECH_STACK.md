# Titanium-rs Tech Stack

## Core Language & Runtime
- **Language**: Rust (Edition 2021)
- **Runtime**: `tokio` (Async runtime, I/O, Task scheduling)

## Critical Dependencies
- **Serialization/Parsing**: 
    - `serde` / `serde_json`: Standard serialization framework.
    - `simd-json`: SIMD-accelerated JSON parsing (x86_64 AVX2/SSE4.2, aarch64 NEON).
    - `serde_repr`: For integer-based enums (Discord OpCodes).
- **Networking**:
    - `tokio-tungstenite`: Async WebSockets.
    - `rustls`: Modern, secure TLS implementation (bypassing OpenSSL).
    - `hyper`: Low-level HTTP implementation.
- **Concurrency & Memory**:
    - `flume`: High-performance multi-producer, multi-consumer channels.
    - `dashmap`: High-performance concurrent hash map for caching.
    - `parking_lot`: Fast, compact locking primitives (Mutex, RwLock).
    - `mimalloc`: Performance-oriented memory allocator (Microsoft).
- **Encryption (Voice)**:
    - `xsalsa20poly1305` / `chacha20poly1305`: Pure Rust implementations of authenticated encryption ciphers.

## Build System & Tooling
- **Build**: `cargo`
- **Linting**: `clippy` (Strict/Pedantic configuration)
- **Documentation**: `rustdoc` (Standard)
- **CI/CD**: GitHub Actions (target)

## Architectural Patterns
- **Monorepo / Workspace**: Multi-crate workspace architecture for separation of concerns (`model`, `gateway`, `voice`, `cache`, `http`).
- **Zero-Copy Serialization**: Extensive use of `Cow<'a, str>` and reference handling to parse implementation details directly from network buffers without allocation.
