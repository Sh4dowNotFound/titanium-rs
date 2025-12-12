# Titanium-rs

[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Docs](https://docs.rs/titanium-rs/badge.svg)](https://docs.rs/titanium-rs)

**Titanium-rs** is a high-performance, concurrent Discord library for Rust, designed for massive scale.

## Documentation
Full documentation is available on [docs.rs](https://docs.rs/titanium-rs). To build locally:
```bash
cargo doc --open
```

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

```rust
use titanium_rs::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN");
    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;

    let client = Client::builder(token)
        .intents(intents)
        .event_handler(Handler)
        .build()
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message_create(&self, ctx: Context, msg: Message<'_>) {
        if msg.content == "!ping" {
            let response = titanium_model::builder::MessageBuilder::new()
                .content("Pong!")
                .build();
            let _ = ctx.http.create_message(msg.channel_id, &response).await;
        }
    }
}
```

## Examples

Check out the `examples/` directory for full working bots:

- **[Basic Bot](https://github.com/Sh4dowNotFound/titanium-rs/blob/main/titanium/examples/basic_bot.rs)**: Simple ping-pong bot demonstrating event handling.
- **[Slash Commands](https://github.com/Sh4dowNotFound/titanium-rs/blob/main/titanium/examples/slash_commands.rs)**: Handling Application Commands (interactions).
- **[Components](https://github.com/Sh4dowNotFound/titanium-rs/blob/main/titanium/examples/components.rs)**: Buttons and Select Menus.
- **[Embeds](https://github.com/Sh4dowNotFound/titanium-rs/blob/main/titanium/examples/embeds_bot.rs)**: Rich embeds using the builder API.
- **[Voice Receive](https://github.com/Sh4dowNotFound/titanium-rs/blob/main/titanium/examples/voice_receive.rs)**: connecting to voice channels (skeleton).

To run an example:
```bash
cargo run --example basic_bot
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

## License

AGPL-3.0-or-later
