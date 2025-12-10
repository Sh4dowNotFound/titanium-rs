# Security Policy

## Supported Versions

Please use the latest version of Titanium-rs to ensure you have the most up-to-date security patches.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of `titanium-rs` seriously. If you discover a security vulnerability, please follow these steps:

1.  **Do NOT open a public issue.**
2.  Email the maintainers directly at `security@titanium-rs.dev` (or see GitHub Profile for direct contact).
3.  Include details about the vulnerability, steps to reproduce, and potential impact.

We will acknowledge your report within 48 hours and work towards a fix.

## Security Features

Titanium-rs is built with security in mind:
-   **No Unsafe Code**: By default, `#![deny(unsafe_code)]` is enforced (with documented exceptions for SIMD).
-   **Memory Safety**: Written in Rust to prevent common memory vulnerabilities.
-   **Regular Audits**: We run `cargo audit` in our CI/CD pipeline.
