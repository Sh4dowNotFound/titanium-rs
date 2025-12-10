#[cfg(test)]
mod tests {
    use crate::{
        member::{Emoji, Sticker},
        Guild, Snowflake, User,
    };

    #[test]
    fn test_user_cdn() {
        let user = User {
            id: Snowflake(123456789),
            username: "test".to_string().into(),
            discriminator: "0".to_string().into(),
            global_name: None,
            avatar: Some("a_123hash".to_string().into()),
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
            avatar_decoration_data: None,
        };

        // Animated avatar
        assert_eq!(
            user.avatar_url(),
            Some("https://cdn.discordapp.com/avatars/123456789/a_123hash.gif".to_string())
        );

        // Default avatar (pomelo)
        // (123456789 >> 22) % 6
        // 123456789 = 0x075BCD15
        // >> 22 = 29
        // 29 % 6 = 5
        assert_eq!(
            user.default_avatar_url(),
            "https://cdn.discordapp.com/embed/avatars/5.png"
        );

        // Face (prefers avatar)
        assert_eq!(
            user.face(),
            "https://cdn.discordapp.com/avatars/123456789/a_123hash.gif"
        );
    }

    #[test]
    fn test_guild_cdn() {
        let guild = Guild {
            id: Snowflake(123),
            name: "Test Guild".to_string().into(),
            icon: Some("iconhash".to_string().into()),
            icon_hash: None,
            splash: Some("splashhash".to_string().into()),
            discovery_splash: None,
            owner_id: None,
            permissions: None,
            region: None,
            afk_channel_id: None,
            afk_timeout: None,
            verification_level: None,
            default_message_notifications: None,
            explicit_content_filter: None,
            roles: vec![],
            emojis: vec![],
            features: vec![],
            mfa_level: None,
            application_id: None,
            system_channel_id: None,
            system_channel_flags: None,
            rules_channel_id: None,
            max_presences: None,
            max_members: None,
            vanity_url_code: None,
            description: None,
            banner: None,
            premium_tier: None,
            premium_subscription_count: None,
            preferred_locale: None,
            public_updates_channel_id: None,
            max_video_channel_users: None,
            max_stage_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            member_count: None,
            nsfw_level: None,
            stickers: vec![],
            premium_progress_bar_enabled: None,
            safety_alerts_channel_id: None,
            voice_states: vec![],
        };

        assert_eq!(
            guild.icon_url(),
            Some("https://cdn.discordapp.com/icons/123/iconhash.png".to_string())
        );
        assert_eq!(
            guild.splash_url(),
            Some("https://cdn.discordapp.com/splashes/123/splashhash.png".to_string())
        );
    }

    #[test]
    fn test_emoji_sticker_cdn() {
        let emoji = Emoji {
            id: Some(Snowflake(999)),
            name: None,
            roles: vec![].into(),
            user: None,
            require_colons: false,
            managed: false,
            animated: true,
            available: true,
        };
        assert_eq!(
            emoji.url(),
            Some("https://cdn.discordapp.com/emojis/999.gif".to_string())
        );

        let sticker = Sticker {
            id: Snowflake(888),
            pack_id: None,
            name: "Test".to_string().into(),
            description: None,
            tags: "".to_string().into(),
            sticker_type: 1,
            format_type: 1, // PNG
            user: None,
            sort_value: None,
        };
        assert_eq!(sticker.url(), "https://cdn.discordapp.com/stickers/888.png");
    }
}
