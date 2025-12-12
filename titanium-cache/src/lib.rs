//! Titanium Cache - High-performance in-memory cache for Discord entities.
//!
//! This crate provides [`InMemoryCache`] for caching Discord entities like
//! guilds, channels, users, and members.
//!
//! # Features
//!
//! - **Lock-Free Concurrency**: Uses [`DashMap`] for concurrent read/write access
//! - **TTL Support**: Automatic expiration of stale entries (default: 1 hour)
//! - **Garbage Collection**: [`InMemoryCache::sweep`] removes expired entries
//! - **Arc-Wrapped Values**: Cheap cloning for multi-consumer scenarios
//!
//! # Example
//!
//! ```no_run
//! use titanium_cache::{Cache, InMemoryCache};
//! use std::time::Duration;
//! use std::sync::Arc;
//!
//! // Create cache with custom TTL
//! let cache = InMemoryCache::with_ttl(Duration::from_secs(600));
//!
//! // Insert and retrieve (example with mock data)
//! // cache.insert_user(Arc::new(user));
//! // let user = cache.user(user_id);
//! ```
//!
//! # Memory Management
//!
//! The cache stores all entities in memory. For large bots, consider:
//! - Using shorter TTL values
//! - Calling [`InMemoryCache::sweep`] periodically
//! - Implementing a custom [`Cache`] trait with Redis/database backing

use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use titanium_model::{Channel, Guild, GuildMember, Role, Snowflake, User};

/// Trait for cache implementations.
///
/// Implement this trait to provide custom caching backends (e.g., Redis, database).
/// The default implementation is [`InMemoryCache`].
pub trait Cache: Send + Sync {
    /// Get a guild by ID.
    fn guild(&self, id: Snowflake) -> Option<Arc<Guild<'static>>>;
    /// Get a channel by ID.
    fn channel(&self, id: Snowflake) -> Option<Arc<Channel<'static>>>;
    /// Get a user by ID.
    fn user(&self, id: Snowflake) -> Option<Arc<User<'static>>>;
    /// Get a guild member by guild and user ID.
    fn member(&self, guild_id: Snowflake, user_id: Snowflake) -> Option<Arc<GuildMember<'static>>>;
    /// Get a role by ID.
    fn role(&self, id: Snowflake) -> Option<Arc<Role<'static>>>;

    /// Insert a guild into the cache.
    fn insert_guild(&self, guild: Arc<Guild<'static>>);
    /// Insert a channel into the cache.
    fn insert_channel(&self, channel: Arc<Channel<'static>>);
    /// Insert a user into the cache.
    fn insert_user(&self, user: Arc<User<'static>>);
    /// Insert a guild member into the cache.
    fn insert_member(&self, guild_id: Snowflake, member: Arc<GuildMember<'static>>);
    /// Insert a role into the cache.
    fn insert_role(&self, id: Snowflake, role: Arc<Role<'static>>);

    /// Remove a guild from the cache.
    fn remove_guild(&self, id: Snowflake) -> Option<Arc<Guild<'static>>>;
    /// Remove a channel from the cache.
    fn remove_channel(&self, id: Snowflake) -> Option<Arc<Channel<'static>>>;
    /// Remove a user from the cache.
    fn remove_user(&self, id: Snowflake) -> Option<Arc<User<'static>>>;
    /// Remove a guild member from the cache.
    fn remove_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Option<Arc<GuildMember<'static>>>;
    /// Remove a role from the cache.
    fn remove_role(&self, id: Snowflake) -> Option<Arc<Role<'static>>>;
}

/// A cached item with a creation timestamp for TTL tracking.
struct CachedItem<T> {
    value: T,
    created_at: Instant,
}

impl<T> CachedItem<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            created_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}

/// In-memory cache for Discord entities with Time-To-Live (TTL).
///
/// Uses [`DashMap`] for lock-free concurrent access. All stored values
/// are wrapped in [`Arc`] for efficient sharing.
///
/// # TTL Behavior
///
/// - Default TTL is 1 hour
/// - Expired items are filtered out on read
/// - Call [`InMemoryCache::sweep`] to remove expired items and free memory
pub struct InMemoryCache {
    guilds: DashMap<Snowflake, CachedItem<Arc<Guild<'static>>>>,
    channels: DashMap<Snowflake, CachedItem<Arc<Channel<'static>>>>,
    users: DashMap<Snowflake, CachedItem<Arc<User<'static>>>>,
    members: DashMap<(Snowflake, Snowflake), CachedItem<Arc<GuildMember<'static>>>>,
    roles: DashMap<Snowflake, CachedItem<Arc<Role<'static>>>>,
    ttl: Duration,
}

impl InMemoryCache {
    /// Create a new empty cache with default TTL (1 hour).
    pub fn new() -> Self {
        Self::with_ttl(Duration::from_secs(3600))
    }

    /// Create a new cache with a custom TTL.
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            guilds: DashMap::new(),
            channels: DashMap::new(),
            users: DashMap::new(),
            members: DashMap::new(),
            roles: DashMap::new(),
            ttl,
        }
    }

    /// Garbage collect expired items.
    ///
    /// Returns the number of items removed.
    pub fn sweep(&self) -> usize {
        let ttl = self.ttl;
        let before = self.guilds.len()
            + self.channels.len()
            + self.users.len()
            + self.members.len()
            + self.roles.len();

        self.guilds.retain(|_, v| !v.is_expired(ttl));
        self.channels.retain(|_, v| !v.is_expired(ttl));
        self.users.retain(|_, v| !v.is_expired(ttl));
        self.members.retain(|_, v| !v.is_expired(ttl));
        self.roles.retain(|_, v| !v.is_expired(ttl));

        let after = self.guilds.len()
            + self.channels.len()
            + self.users.len()
            + self.members.len()
            + self.roles.len();
        before.saturating_sub(after)
    }
}

impl Cache for InMemoryCache {
    fn guild(&self, id: Snowflake) -> Option<Arc<Guild<'static>>> {
        self.guilds
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn channel(&self, id: Snowflake) -> Option<Arc<Channel<'static>>> {
        self.channels
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn user(&self, id: Snowflake) -> Option<Arc<User<'static>>> {
        self.users
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn member(&self, guild_id: Snowflake, user_id: Snowflake) -> Option<Arc<GuildMember<'static>>> {
        self.members
            .get(&(guild_id, user_id))
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn role(&self, id: Snowflake) -> Option<Arc<Role<'static>>> {
        self.roles
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn insert_guild(&self, guild: Arc<Guild<'static>>) {
        for role in &guild.roles {
            self.insert_role(role.id, Arc::new(role.clone()));
        }
        self.guilds.insert(guild.id, CachedItem::new(guild));
    }

    fn insert_channel(&self, channel: Arc<Channel<'static>>) {
        self.channels.insert(channel.id, CachedItem::new(channel));
    }

    fn insert_user(&self, user: Arc<User<'static>>) {
        self.users.insert(user.id, CachedItem::new(user));
    }

    fn insert_member(&self, guild_id: Snowflake, member: Arc<GuildMember<'static>>) {
        if let Some(ref user) = member.user {
            self.insert_user(Arc::new(user.clone()));
        }
        self.members.insert(
            (
                guild_id,
                member.user.as_ref().map(|u| u.id).unwrap_or_default(),
            ),
            CachedItem::new(member),
        );
    }

    fn insert_role(&self, id: Snowflake, role: Arc<Role<'static>>) {
        self.roles.insert(id, CachedItem::new(role));
    }

    fn remove_guild(&self, id: Snowflake) -> Option<Arc<Guild<'static>>> {
        self.guilds.remove(&id).map(|(_, v)| v.value)
    }

    fn remove_channel(&self, id: Snowflake) -> Option<Arc<Channel<'static>>> {
        self.channels.remove(&id).map(|(_, v)| v.value)
    }

    fn remove_user(&self, id: Snowflake) -> Option<Arc<User<'static>>> {
        self.users.remove(&id).map(|(_, v)| v.value)
    }

    fn remove_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Option<Arc<GuildMember<'static>>> {
        self.members
            .remove(&(guild_id, user_id))
            .map(|(_, v)| v.value)
    }

    fn remove_role(&self, id: Snowflake) -> Option<Arc<Role<'static>>> {
        self.roles.remove(&id).map(|(_, v)| v.value)
    }
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}
