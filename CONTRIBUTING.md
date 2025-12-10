# Contributing to Titanium-rs

First off, thanks for taking the time to contribute!

## Core Philosophy
Titanium-rs is built on three pillars:
1.  **Safety**: We enforce `#[deny(unsafe_code)]`. If you need `unsafe`, it must be isolated, strictly justified (e.g., SIMD), and documented.
2.  **Performance**: Allocations are the enemy. Use `Cow`, iterators, and zero-copy parsing wherever possible.
3.  **Correctness**: We use aggressive internal lints (`clippy::pedantic`). Your code must pass `cargo clippy` and `cargo test` before merging.

## Development Workflow
1.  **Fork & Clone** the repository.
2.  **Install dependencies**: `cargo build`
3.  **Run Tests**: `cargo test` (Ensure everything passes).
4.  **Lint**: `cargo clippy -- -W clippy::pedantic` (Fix all warnings).
5.  **Format**: `cargo fmt`
6.  **Commit**: Use conventional commits (e.g., `feat: add voice encryption`, `fix: parsing error`).

## CI/CD Pipeline
Every PR is checked against:
- Clippy (Pedantic)
- `cargo test --all`
- `cargo doc --no-deps`

## Documentation
Documentation is hosted via GitHub Pages and built automatically. Ensure your public API has distinct rustdoc comments.
