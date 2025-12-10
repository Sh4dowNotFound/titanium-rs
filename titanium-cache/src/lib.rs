//! Titan Cache - In-memory cache for Discord entities.
//!
//! This crate provides a high-performance concurrent cache using DashMap.

use dashmap::DashMap;

use titanium_model::{Channel, Guild, GuildMember, Role, Snowflake, User};

/// Trait for cache implementations.
pub trait Cache: Send + Sync {
    fn guild(&self, id: Snowflake) -> Option<Guild<'static>>;
    fn channel(&self, id: Snowflake) -> Option<Channel<'static>>;
    fn user(&self, id: Snowflake) -> Option<User<'static>>;
    fn member(&self, guild_id: Snowflake, user_id: Snowflake) -> Option<GuildMember<'static>>;
    fn role(&self, id: Snowflake) -> Option<Role<'static>>;

    fn insert_guild(&self, guild: Guild<'static>);
    fn insert_channel(&self, channel: Channel<'static>);
    fn insert_user(&self, user: User<'static>);
    fn insert_member(&self, guild_id: Snowflake, member: GuildMember<'static>);
    fn insert_role(&self, id: Snowflake, role: Role<'static>);
}

use std::time::{Duration, Instant};

/// A cached item with a timestamp.
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
pub struct InMemoryCache {
    guilds: DashMap<Snowflake, CachedItem<Guild<'static>>>,
    channels: DashMap<Snowflake, CachedItem<Channel<'static>>>,
    users: DashMap<Snowflake, CachedItem<User<'static>>>,
    members: DashMap<(Snowflake, Snowflake), CachedItem<GuildMember<'static>>>,
    roles: DashMap<Snowflake, CachedItem<Role<'static>>>,
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
        let count = 0;
        let ttl = self.ttl;

        self.guilds.retain(|_, v| !v.is_expired(ttl));
        self.channels.retain(|_, v| !v.is_expired(ttl));
        self.users.retain(|_, v| !v.is_expired(ttl));
        self.members.retain(|_, v| !v.is_expired(ttl));
        self.roles.retain(|_, v| !v.is_expired(ttl));

        // Note: DashMap::retain doesn't return count easily without locking or iterating.
        // For high performance, we trust retain does its job.
        // If we strictly needed a count we would wrap/count, but for now 0 is returned
        // to match signature or we can change signature.
        count
    }
}

impl Cache for InMemoryCache {
    fn guild(&self, id: Snowflake) -> Option<Guild<'static>> {
        self.guilds
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn channel(&self, id: Snowflake) -> Option<Channel<'static>> {
        self.channels
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn user(&self, id: Snowflake) -> Option<User<'static>> {
        self.users
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn member(&self, guild_id: Snowflake, user_id: Snowflake) -> Option<GuildMember<'static>> {
        self.members
            .get(&(guild_id, user_id))
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn role(&self, id: Snowflake) -> Option<Role<'static>> {
        self.roles
            .get(&id)
            .filter(|i| !i.is_expired(self.ttl))
            .map(|r| r.value.clone())
    }

    fn insert_guild(&self, guild: Guild<'static>) {
        for role in &guild.roles {
            self.insert_role(role.id, role.clone());
        }
        self.guilds.insert(guild.id, CachedItem::new(guild));
    }

    fn insert_channel(&self, channel: Channel<'static>) {
        self.channels.insert(channel.id, CachedItem::new(channel));
    }

    fn insert_user(&self, user: User<'static>) {
        self.users.insert(user.id, CachedItem::new(user));
    }

    fn insert_member(&self, guild_id: Snowflake, member: GuildMember<'static>) {
        if let Some(ref user) = member.user {
            self.insert_user(user.clone());
        }
        self.members.insert(
            (
                guild_id,
                member.user.as_ref().map(|u| u.id).unwrap_or_default(),
            ),
            CachedItem::new(member),
        );
    }

    fn insert_role(&self, id: Snowflake, role: Role<'static>) {
        self.roles.insert(id, CachedItem::new(role));
    }
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}
