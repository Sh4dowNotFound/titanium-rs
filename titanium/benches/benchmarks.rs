//! Performance benchmarks for titanium-rs
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::time::Duration;

/// Benchmark Snowflake serialization
fn bench_snowflake(c: &mut Criterion) {
    use titanium_model::Snowflake;

    let snowflake = Snowflake(123456789012345678);

    let mut group = c.benchmark_group("snowflake");
    group.throughput(Throughput::Elements(1));

    group.bench_function("serialize_itoa", |b| {
        b.iter(|| {
            let mut buf = itoa::Buffer::new();
            black_box(buf.format(snowflake.0));
        })
    });

    group.bench_function("serialize_to_string", |b| {
        b.iter(|| {
            black_box(snowflake.0.to_string());
        })
    });

    group.finish();
}

/// Benchmark JSON parsing
fn bench_json_parsing(c: &mut Criterion) {
    let json_str = r#"{
        "id": "123456789012345678",
        "username": "test_user",
        "discriminator": "0001",
        "avatar": "abc123",
        "bot": false,
        "system": false,
        "mfa_enabled": true,
        "banner": null,
        "accent_color": 16711680,
        "locale": "en-US",
        "verified": true,
        "email": "test@example.com",
        "flags": 64,
        "premium_type": 2,
        "public_flags": 64
    }"#;

    let json_bytes = json_str.as_bytes().to_vec();

    let mut group = c.benchmark_group("json_parsing");
    group.throughput(Throughput::Bytes(json_str.len() as u64));

    group.bench_function("simd_json_user", |b| {
        b.iter(|| {
            let mut buf = json_bytes.clone();
            let _: titanium_model::User = simd_json::from_slice(&mut buf).unwrap();
        })
    });

    group.finish();
}

/// Benchmark EmbedBuilder
fn bench_embed_builder(c: &mut Criterion) {
    use titanium_model::builder::EmbedBuilder;

    let mut group = c.benchmark_group("embed_builder");
    group.throughput(Throughput::Elements(1));

    group.bench_function("simple_embed", |b| {
        b.iter(|| {
            black_box(
                EmbedBuilder::simple("Title", "Description")
                    .color(0x5865F2)
                    .build(),
            );
        })
    });

    group.bench_function("complex_embed", |b| {
        b.iter(|| {
            black_box(
                EmbedBuilder::new()
                    .title("Title")
                    .description("Description")
                    .color(0x5865F2)
                    .field("Field 1", "Value 1", true)
                    .field("Field 2", "Value 2", true)
                    .field("Field 3", "Value 3", false)
                    .footer("Footer text", None::<String>)
                    .build(),
            );
        })
    });

    group.finish();
}

/// Benchmark Cache operations
fn bench_cache(c: &mut Criterion) {
    use std::sync::Arc;
    use titanium_cache::{Cache, InMemoryCache};
    use titanium_model::{Snowflake, TitanString, User};

    let cache = InMemoryCache::new();

    // Pre-populate cache
    for i in 0..1000u64 {
        let user = User {
            id: Snowflake(i),
            username: TitanString::Owned(format!("user_{}", i)),
            discriminator: TitanString::Borrowed("0001"),
            avatar: None,
            bot: false,
            system: false,
            mfa_enabled: None,
            banner: None,
            accent_color: None,
            locale: None,
            verified: None,
            email: None,
            flags: None,
            premium_type: None,
            public_flags: None,
            global_name: None,
            avatar_decoration_data: None,
        };
        cache.insert_user(Arc::new(user));
    }

    let mut group = c.benchmark_group("cache");

    group.bench_function("get_user_hit", |b| {
        b.iter(|| {
            black_box(cache.user(Snowflake(500)));
        })
    });

    group.bench_function("get_user_miss", |b| {
        b.iter(|| {
            black_box(cache.user(Snowflake(999999)));
        })
    });

    group.finish();
}

/// Benchmark MessageBuilder
fn bench_message_builder(c: &mut Criterion) {
    use titanium_model::builder::{ActionRowBuilder, ButtonBuilder, EmbedBuilder, MessageBuilder};

    let mut group = c.benchmark_group("message_builder");
    group.throughput(Throughput::Elements(1));

    group.bench_function("simple_text", |b| {
        b.iter(|| {
            black_box(MessageBuilder::text("Hello, World!").build());
        })
    });

    group.bench_function("with_embed", |b| {
        b.iter(|| {
            black_box(
                MessageBuilder::new()
                    .content("Check this out!")
                    .embed(EmbedBuilder::simple("Title", "Desc"))
                    .build(),
            );
        })
    });

    group.bench_function("with_components", |b| {
        b.iter(|| {
            black_box(
                MessageBuilder::new()
                    .content("Click a button:")
                    .component(
                        ActionRowBuilder::new()
                            .add_button(ButtonBuilder::primary("Yes", "btn_yes"))
                            .add_button(ButtonBuilder::danger("No", "btn_no")),
                    )
                    .build(),
            );
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(100);
    targets = bench_snowflake, bench_json_parsing, bench_embed_builder, bench_cache, bench_message_builder
}

criterion_main!(benches);
