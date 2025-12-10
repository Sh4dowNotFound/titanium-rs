//! Builders for creating Discord entities.

use crate::{
    Component, CreateMessage, Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedMedia, TitanString,
};

/// Builder for creating an Embed.
#[derive(Debug, Clone, Default)]
pub struct EmbedBuilder<'a> {
    embed: Embed<'a>,
}

impl<'a> EmbedBuilder<'a> {
    /// Create a new EmbedBuilder.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title of the embed.
    #[inline]
    pub fn title(mut self, title: impl Into<TitanString<'a>>) -> Self {
        self.embed.title = Some(title.into());
        self
    }

    /// Set the description of the embed.
    pub fn description(mut self, description: impl Into<TitanString<'a>>) -> Self {
        self.embed.description = Some(description.into());
        self
    }

    /// Set the URL of the embed.
    pub fn url(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.embed.url = Some(url.into());
        self
    }

    /// Set the timestamp of the embed.
    pub fn timestamp(mut self, timestamp: impl Into<TitanString<'a>>) -> Self {
        self.embed.timestamp = Some(timestamp.into());
        self
    }

    /// Set the color of the embed.
    pub fn color(mut self, color: u32) -> Self {
        self.embed.color = Some(color);
        self
    }

    /// Set the color of the embed from RGB values.
    pub fn color_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.embed.color = Some(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
        self
    }

    /// Set the footer of the embed.
    pub fn footer(
        mut self,
        text: impl Into<TitanString<'a>>,
        icon_url: Option<impl Into<TitanString<'a>>>,
    ) -> Self {
        self.embed.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: icon_url.map(Into::into),
            proxy_icon_url: None,
        });
        self
    }

    /// Set the image of the embed.
    pub fn image(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.embed.image = Some(EmbedMedia {
            url: Some(url.into()),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Set the thumbnail of the embed.
    pub fn thumbnail(mut self, url: impl Into<TitanString<'a>>) -> Self {
        self.embed.thumbnail = Some(EmbedMedia {
            url: Some(url.into()),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    /// Set the author of the embed.
    pub fn author(
        mut self,
        name: impl Into<TitanString<'a>>,
        url: Option<impl Into<TitanString<'a>>>,
        icon_url: Option<impl Into<TitanString<'a>>>,
    ) -> Self {
        self.embed.author = Some(EmbedAuthor {
            name: name.into(),
            url: url.map(Into::into),
            icon_url: icon_url.map(Into::into),
            proxy_icon_url: None,
        });
        self
    }

    /// Add a field to the embed.
    pub fn field(
        mut self,
        name: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
        inline: bool,
    ) -> Self {
        self.embed.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    /// Add an inline field.
    pub fn field_inline(
        self,
        name: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
    ) -> Self {
        self.field(name, value, true)
    }

    /// Add a block field (not inline).
    pub fn field_block(
        self,
        name: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
    ) -> Self {
        self.field(name, value, false)
    }

    /// Build the Embed.
    pub fn build(self) -> Embed<'a> {
        self.embed
    }
}

/// Builder for creating a Message.
#[derive(Debug, Clone, Default)]
pub struct MessageBuilder<'a> {
    message: CreateMessage<'a>,
}

impl<'a> MessageBuilder<'a> {
    /// Create a new MessageBuilder.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content of the message.
    #[inline]
    pub fn content(mut self, content: impl Into<TitanString<'a>>) -> Self {
        self.message.content = Some(content.into());
        self
    }

    /// Enable/disable TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        self.message.tts = Some(tts);
        self
    }

    /// Reply to a message (sets message_reference).
    pub fn reply(mut self, message_id: impl Into<crate::Snowflake>) -> Self {
        self.message.message_reference = Some(crate::MessageReference {
            message_id: Some(message_id.into()),
            channel_id: None,
            guild_id: None,
            fail_if_not_exists: Some(true),
        });
        self
    }

    /// Add an embed to the message.
    pub fn embed(mut self, embed: impl Into<Embed<'a>>) -> Self {
        if let Some(embeds) = &mut self.message.embeds {
            embeds.push(embed.into());
        } else {
            self.message.embeds = Some(vec![embed.into()]);
        }
        self
    }

    /// Add multiple embeds.
    pub fn embeds(mut self, embeds: Vec<Embed<'a>>) -> Self {
        if let Some(existing) = &mut self.message.embeds {
            existing.extend(embeds);
        } else {
            self.message.embeds = Some(embeds);
        }
        self
    }

    /// Add a component (ActionRow, etc.) to the message.
    pub fn component(mut self, component: impl Into<Component<'a>>) -> Self {
        if let Some(components) = &mut self.message.components {
            components.push(component.into());
        } else {
            self.message.components = Some(vec![component.into()]);
        }
        self
    }

    /// Add a file to upload.
    pub fn add_file(
        mut self,
        filename: impl Into<TitanString<'a>>,
        data: impl Into<Vec<u8>>,
    ) -> Self {
        self.message.files.push(crate::file::FileUpload::new(
            filename.into().into_owned(),
            data,
        ));
        self
    }

    /// Build the CreateMessage payload.
    pub fn build(self) -> CreateMessage<'a> {
        self.message
    }
}

// ============================================================================
// Modify Guild Builder
// ============================================================================

/// Payload for modifying a guild.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ModifyGuild<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<TitanString<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<crate::Snowflake>,
}

/// Builder for modifying a Guild.
#[derive(Debug, Clone, Default)]
pub struct ModifyGuildBuilder<'a> {
    params: ModifyGuild<'a>,
}

impl<'a> ModifyGuildBuilder<'a> {
    /// Create a new ModifyGuildBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name.
    pub fn name(mut self, name: impl Into<TitanString<'a>>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    /// Set region (deprecated).
    pub fn region(mut self, region: impl Into<TitanString<'a>>) -> Self {
        self.params.region = Some(region.into());
        self
    }

    /// Set verification level.
    pub fn verification_level(mut self, level: u8) -> Self {
        self.params.verification_level = Some(level);
        self
    }

    /// Set default message notifications.
    pub fn default_message_notifications(mut self, level: u8) -> Self {
        self.params.default_message_notifications = Some(level);
        self
    }

    /// Set explicit content filter.
    pub fn explicit_content_filter(mut self, level: u8) -> Self {
        self.params.explicit_content_filter = Some(level);
        self
    }

    /// Set AFK channel ID.
    pub fn afk_channel_id(mut self, id: impl Into<crate::Snowflake>) -> Self {
        self.params.afk_channel_id = Some(id.into());
        self
    }

    /// Set AFK timeout.
    pub fn afk_timeout(mut self, timeout: u32) -> Self {
        self.params.afk_timeout = Some(timeout);
        self
    }

    /// Set icon (base64).
    pub fn icon(mut self, icon: impl Into<TitanString<'a>>) -> Self {
        self.params.icon = Some(icon.into());
        self
    }

    /// Set system channel ID.
    pub fn system_channel_id(mut self, id: impl Into<crate::Snowflake>) -> Self {
        self.params.system_channel_id = Some(id.into());
        self
    }

    /// Build the ModifyGuild payload.
    pub fn build(self) -> ModifyGuild<'a> {
        self.params
    }
}

// ============================================================================
// Modify Member Builder
// ============================================================================

/// Payload for modifying a guild member.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ModifyMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::Snowflake>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<crate::Snowflake>, // Move to voice channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<TitanString<'a>>, // Timeout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}

/// Builder for modifying a GuildMember.
#[derive(Debug, Clone, Default)]
pub struct ModifyMemberBuilder<'a> {
    params: ModifyMember<'a>,
}

impl<'a> ModifyMemberBuilder<'a> {
    /// Create a new ModifyMemberBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set nickname.
    pub fn nick(mut self, nick: impl Into<TitanString<'a>>) -> Self {
        self.params.nick = Some(nick.into());
        self
    }

    /// Set roles (replaces all roles).
    pub fn roles(mut self, roles: Vec<crate::Snowflake>) -> Self {
        self.params.roles = Some(roles);
        self
    }

    /// Mute or unmute.
    pub fn mute(mut self, mute: bool) -> Self {
        self.params.mute = Some(mute);
        self
    }

    /// Deafen or undeafen.
    pub fn deaf(mut self, deaf: bool) -> Self {
        self.params.deaf = Some(deaf);
        self
    }

    /// Move to voice channel (or disconnect if null, but we use strict type here).
    pub fn move_to_channel(mut self, channel_id: impl Into<crate::Snowflake>) -> Self {
        self.params.channel_id = Some(channel_id.into());
        self
    }

    /// Timeout user until timestamp (ISO8601).
    pub fn timeout_until(mut self, timestamp: impl Into<TitanString<'a>>) -> Self {
        self.params.communication_disabled_until = Some(timestamp.into());
        self
    }

    /// Build the ModifyMember payload.
    pub fn build(self) -> ModifyMember<'a> {
        self.params
    }
}

// ============================================================================
// Start Thread Builder
// ============================================================================

/// Payload for starting a thread.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartThread<'a> {
    pub name: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<u8>, // For Start Thread without Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
}

/// Builder for starting a Thread.
#[derive(Debug, Clone)]
pub struct StartThreadBuilder<'a> {
    params: StartThread<'a>,
}

impl<'a> StartThreadBuilder<'a> {
    /// Create a new StartThreadBuilder.
    pub fn new(name: impl Into<TitanString<'a>>) -> Self {
        Self {
            params: StartThread {
                name: name.into(),
                ..Default::default()
            },
        }
    }

    /// Set auto archive duration (60, 1440, 4320, 10080).
    pub fn auto_archive_duration(mut self, duration: u32) -> Self {
        self.params.auto_archive_duration = Some(duration);
        self
    }

    /// Set thread type (for standalone threads).
    pub fn kind(mut self, kind: u8) -> Self {
        self.params.type_ = Some(kind);
        self
    }

    /// Set invitable (private threads).
    pub fn invitable(mut self, invitable: bool) -> Self {
        self.params.invitable = Some(invitable);
        self
    }

    /// Set rate limit per user.
    pub fn rate_limit_per_user(mut self, limit: u32) -> Self {
        self.params.rate_limit_per_user = Some(limit);
        self
    }

    /// Build the StartThread payload.
    pub fn build(self) -> StartThread<'a> {
        self.params
    }
}

// ============================================================================
// Component Builders
// ============================================================================

/// Builder for creating a Button.
#[derive(Debug, Clone)]
pub struct ButtonBuilder<'a> {
    component: crate::Component<'a>,
}

impl<'a> ButtonBuilder<'a> {
    /// Create a new ButtonBuilder.
    #[inline]
    pub fn new() -> Self {
        Self {
            component: crate::Component::Button(crate::component::Button {
                style: crate::component::ButtonStyle::Primary, // Default
                label: None,
                emoji: None,
                custom_id: None,
                url: None,
                disabled: false,
                component_type: crate::component::ComponentType::Button,
            }),
        }
    }

    /// Set style.
    pub fn style(mut self, style: crate::component::ButtonStyle) -> Self {
        if let crate::Component::Button(b) = &mut self.component {
            b.style = style;
        }
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<TitanString<'a>>) -> Self {
        if let crate::Component::Button(b) = &mut self.component {
            b.label = Some(label.into());
        }
        self
    }

    /// Set emoji.
    pub fn emoji(mut self, emoji: impl Into<crate::reaction::ReactionEmoji<'a>>) -> Self {
        if let crate::Component::Button(b) = &mut self.component {
            b.emoji = Some(emoji.into());
        }
        self
    }

    /// Set custom ID.
    pub fn custom_id(mut self, id: impl Into<TitanString<'a>>) -> Self {
        if let crate::Component::Button(b) = &mut self.component {
            b.custom_id = Some(id.into());
        }
        self
    }

    /// Set URL.
    pub fn url(mut self, url: impl Into<TitanString<'a>>) -> Self {
        if let crate::Component::Button(b) = &mut self.component {
            b.url = Some(url.into());
        }
        self
    }

    /// Set disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        if let crate::Component::Button(b) = &mut self.component {
            b.disabled = disabled;
        }
        self
    }

    /// Build the Component.
    pub fn build(self) -> crate::Component<'a> {
        self.component
    }
}

/// Builder for creating a Select Menu.
#[derive(Debug, Clone)]
pub struct SelectMenuBuilder<'a> {
    component: crate::Component<'a>,
}

impl<'a> SelectMenuBuilder<'a> {
    /// Create a new SelectMenuBuilder.
    #[inline]
    pub fn new(custom_id: impl Into<TitanString<'a>>) -> Self {
        Self {
            component: crate::Component::SelectMenu(crate::component::SelectMenu {
                custom_id: custom_id.into(),
                options: Vec::with_capacity(25), // Discord max options
                placeholder: None,
                min_values: None,
                max_values: None,
                disabled: false,
                component_type: crate::component::ComponentType::StringSelect, // Default
            }),
        }
    }

    /// Add an option.
    pub fn option(
        mut self,
        label: impl Into<TitanString<'a>>,
        value: impl Into<TitanString<'a>>,
    ) -> Self {
        if let crate::Component::SelectMenu(s) = &mut self.component {
            s.options.push(crate::component::SelectOption {
                label: label.into(),
                value: value.into(),
                description: None,
                emoji: None,
                default: false,
            });
        }
        self
    }

    /// Set placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<TitanString<'a>>) -> Self {
        if let crate::Component::SelectMenu(s) = &mut self.component {
            s.placeholder = Some(placeholder.into());
        }
        self
    }

    /// Set min values.
    pub fn min_values(mut self, min: u8) -> Self {
        if let crate::Component::SelectMenu(s) = &mut self.component {
            s.min_values = Some(min);
        }
        self
    }

    /// Set max values.
    pub fn max_values(mut self, max: u8) -> Self {
        if let crate::Component::SelectMenu(s) = &mut self.component {
            s.max_values = Some(max);
        }
        self
    }

    /// Set disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        if let crate::Component::SelectMenu(s) = &mut self.component {
            s.disabled = disabled;
        }
        self
    }

    /// Build the Component.
    pub fn build(self) -> crate::Component<'a> {
        self.component
    }
}

/// Builder for creating an Action Row.
#[derive(Debug, Clone)]
pub struct ActionRowBuilder<'a> {
    component: crate::Component<'a>,
}

impl<'a> ActionRowBuilder<'a> {
    pub fn new() -> Self {
        Self {
            component: crate::Component::ActionRow(crate::component::ActionRow {
                components: Vec::with_capacity(5), // Discord max components per row
                component_type: crate::component::ComponentType::ActionRow,
            }),
        }
    }

    pub fn add_button(mut self, button: ButtonBuilder<'a>) -> Self {
        if let crate::Component::ActionRow(r) = &mut self.component {
            r.components.push(button.build());
        }
        self
    }

    pub fn add_select_menu(mut self, menu: SelectMenuBuilder<'a>) -> Self {
        if let crate::Component::ActionRow(r) = &mut self.component {
            r.components.push(menu.build());
        }
        self
    }

    pub fn build(self) -> crate::Component<'a> {
        self.component
    }
}

/// Builder for Interaction Response.
#[derive(Debug, Clone)]
pub struct InteractionResponseBuilder<'a> {
    response: crate::interaction::InteractionResponse<'a>,
}

impl<'a> Default for InteractionResponseBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> InteractionResponseBuilder<'a> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            response: crate::interaction::InteractionResponse {
                response_type:
                    crate::interaction::InteractionCallbackType::ChannelMessageWithSource,
                data: Some(Default::default()),
            },
        }
    }

    pub fn kind(mut self, kind: crate::interaction::InteractionCallbackType) -> Self {
        self.response.response_type = kind;
        self
    }

    pub fn content(mut self, content: impl Into<TitanString<'a>>) -> Self {
        if let Some(data) = &mut self.response.data {
            data.content = Some(content.into());
        }
        self
    }

    pub fn embed(mut self, embed: impl Into<crate::Embed<'a>>) -> Self {
        if self.response.data.is_none() {
            self.response.data = Some(Default::default());
        }
        if let Some(data) = &mut self.response.data {
            data.embeds.push(embed.into());
        }
        self
    }

    pub fn build(self) -> crate::interaction::InteractionResponse<'a> {
        self.response
    }
}

// ============================================================================
// Create Channel Builder
// ============================================================================

/// Payload for creating a channel.
/// Payload for creating a channel.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateChannel<'a> {
    pub name: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub kind: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<crate::json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

/// Builder for creating a Channel.
#[derive(Debug, Clone, Default)]
pub struct CreateChannelBuilder<'a> {
    params: CreateChannel<'a>,
}

impl<'a> CreateChannelBuilder<'a> {
    pub fn new(name: impl Into<TitanString<'a>>) -> Self {
        let mut builder = Self::default();
        builder.params.name = name.into();
        builder
    }

    pub fn kind(mut self, kind: u8) -> Self {
        self.params.kind = Some(kind);
        self
    }

    pub fn topic(mut self, topic: impl Into<TitanString<'a>>) -> Self {
        self.params.topic = Some(topic.into());
        self
    }

    pub fn build(self) -> CreateChannel<'a> {
        self.params
    }
}

// ============================================================================
// Create Role Builder
// ============================================================================

/// Payload for creating a role.
/// Payload for creating a role.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateRole<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

/// Builder for creating a Role.
#[derive(Debug, Clone, Default)]
pub struct CreateRoleBuilder<'a> {
    params: CreateRole<'a>,
}

impl<'a> CreateRoleBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<TitanString<'a>>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.params.color = Some(color);
        self
    }

    pub fn hoist(mut self, hoist: bool) -> Self {
        self.params.hoist = Some(hoist);
        self
    }

    pub fn icon(mut self, icon: impl Into<TitanString<'a>>) -> Self {
        self.params.icon = Some(icon.into());
        self
    }

    pub fn unicode_emoji(mut self, emoji: impl Into<TitanString<'a>>) -> Self {
        self.params.unicode_emoji = Some(emoji.into());
        self
    }

    pub fn mentionable(mut self, mentionable: bool) -> Self {
        self.params.mentionable = Some(mentionable);
        self
    }

    pub fn build(self) -> CreateRole<'a> {
        self.params
    }
}

// ============================================================================
// Command Builder (Slash Commands)
// ============================================================================

/// Builder for creating an Application Command (Slash Command).
#[derive(Debug, Clone, serde::Serialize)]
pub struct CommandBuilder<'a> {
    pub name: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub kind: Option<crate::command::CommandType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl<'a> CommandBuilder<'a> {
    pub fn new(name: impl Into<TitanString<'a>>, description: impl Into<TitanString<'a>>) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            name_localizations: None,
            description_localizations: None,
            default_member_permissions: None,
            dm_permission: None,
            kind: Some(crate::command::CommandType::ChatInput),
            nsfw: None,
        }
    }

    pub fn build(self) -> Self {
        self
    }
}

// ============================================================================
// AutoMod Rule Builder
// ============================================================================

#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct AutoModRuleBuilder {
    pub name: String,
}

// Duplicates removed

// ============================================================================
// Missing Variants Placeholders
// ============================================================================

/// Payload for creating a scheduled event.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateScheduledEvent<'a> {
    pub name: TitanString<'a>,
    pub privacy_level: crate::scheduled::ScheduledEventPrivacyLevel,
    pub scheduled_start_time: TitanString<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<TitanString<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TitanString<'a>>,
    pub entity_type: crate::scheduled::ScheduledEventEntityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<crate::scheduled::ScheduledEventEntityMetadata<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<TitanString<'a>>, // Base64
}

/// Builder for creating a Scheduled Event.
#[derive(Debug, Clone)]
pub struct ScheduledEventBuilder<'a> {
    params: CreateScheduledEvent<'a>,
}

impl<'a> ScheduledEventBuilder<'a> {
    /// Create a new ScheduledEventBuilder.
    pub fn new(
        name: impl Into<TitanString<'a>>,
        start_time: impl Into<TitanString<'a>>,
        entity_type: crate::scheduled::ScheduledEventEntityType,
    ) -> Self {
        Self {
            params: CreateScheduledEvent {
                name: name.into(),
                scheduled_start_time: start_time.into(),
                entity_type,
                privacy_level: crate::scheduled::ScheduledEventPrivacyLevel::GuildOnly,
                ..Default::default()
            },
        }
    }

    /// Set description.
    #[inline]
    pub fn description(mut self, description: impl Into<TitanString<'a>>) -> Self {
        self.params.description = Some(description.into());
        self
    }

    /// Set end time.
    #[inline]
    pub fn end_time(mut self, time: impl Into<TitanString<'a>>) -> Self {
        self.params.scheduled_end_time = Some(time.into());
        self
    }

    /// Set channel ID (required for Stage/Voice events).
    #[inline]
    pub fn channel_id(mut self, id: impl Into<crate::Snowflake>) -> Self {
        self.params.channel_id = Some(id.into());
        self
    }

    /// Set location (required for External events).
    #[inline]
    pub fn location(mut self, location: impl Into<TitanString<'a>>) -> Self {
        self.params.entity_metadata = Some(crate::scheduled::ScheduledEventEntityMetadata {
            location: Some(location.into()),
        });
        self
    }

    /// Set cover image (base64).
    #[inline]
    pub fn image(mut self, image: impl Into<TitanString<'a>>) -> Self {
        self.params.image = Some(image.into());
        self
    }

    /// Build the payload.
    #[inline]
    pub fn build(self) -> CreateScheduledEvent<'a> {
        self.params
    }
}

/// Builder for creating a Poll.
#[derive(Debug, Clone)]
pub struct PollBuilder<'a> {
    poll: crate::poll::Poll<'a>,
}

impl<'a> PollBuilder<'a> {
    /// Create a new PollBuilder.
    pub fn new(question: impl Into<TitanString<'a>>) -> Self {
        Self {
            poll: crate::poll::Poll {
                question: crate::poll::PollMedia {
                    text: Some(question.into()),
                    emoji: None,
                },
                answers: Vec::new(),
                expiry: None,
                allow_multiselect: false,
                layout_type: None,
                results: None,
            },
        }
    }

    /// Add an answer.
    pub fn answer(mut self, answer: impl Into<crate::poll::PollAnswer<'a>>) -> Self {
        self.poll.answers.push(answer.into());
        self
    }

    /// Set expiry.
    pub fn expiry(mut self, expiry: impl Into<TitanString<'a>>) -> Self {
        self.poll.expiry = Some(expiry.into());
        self
    }

    /// Allow multiselect.
    pub fn allow_multiselect(mut self, allow: bool) -> Self {
        self.poll.allow_multiselect = allow;
        self
    }

    /// Build the Poll.
    pub fn build(self) -> crate::poll::Poll<'a> {
        self.poll
    }
}

/// Payload for executing a webhook.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ExecuteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<crate::Embed<'static>>,
    // Note: files and components omitted for brevity in this iteration, but can be added
}

/// Builder for executing a Webhook.
#[derive(Debug, Clone, Default)]
pub struct WebhookExecuteBuilder<'a> {
    params: ExecuteWebhook,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> WebhookExecuteBuilder<'a> {
    /// Create a new WebhookExecuteBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set content.
    #[inline]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.params.content = Some(content.into());
        self
    }

    /// Set username override.
    #[inline]
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.params.username = Some(username.into());
        self
    }

    /// Set avatar URL override.
    #[inline]
    pub fn avatar_url(mut self, url: impl Into<String>) -> Self {
        self.params.avatar_url = Some(url.into());
        self
    }

    /// Set TTS.
    #[inline]
    pub fn tts(mut self, tts: bool) -> Self {
        self.params.tts = Some(tts);
        self
    }

    /// Add an embed.
    #[inline]
    pub fn embed(mut self, embed: impl Into<crate::Embed<'static>>) -> Self {
        self.params.embeds.push(embed.into());
        self
    }

    /// Add multiple embeds.
    #[inline]
    pub fn embeds(mut self, embeds: Vec<crate::Embed<'static>>) -> Self {
        self.params.embeds.extend(embeds);
        self
    }

    /// Build the payload.
    #[inline]
    pub fn build(self) -> ExecuteWebhook {
        self.params
    }
}

// Add missing Default implementations and inlines to other builders where applicable
impl<'a> Default for ButtonBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Default for ActionRowBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Default for SelectMenuBuilder<'a> {
    fn default() -> Self {
        Self::new("default_select") // Fallback, though usually ID is required
    }
}

#[cfg(test)]
mod modify_tests {
    use super::*;
    #[test]
    fn test_modify_guild_builder() {
        let payload = ModifyGuildBuilder::new()
            .name("New Guild Name")
            .region("us-west")
            .verification_level(1)
            .build();

        assert_eq!(
            payload.name,
            Some(TitanString::from("New Guild Name".to_string()))
        );
        assert_eq!(
            payload.region,
            Some(TitanString::from("us-west".to_string()))
        );
        assert_eq!(payload.verification_level, Some(1));
    }

    #[test]
    fn test_modify_member_builder() {
        let payload = ModifyMemberBuilder::new()
            .nick("New Nick")
            .mute(true)
            .deaf(false)
            .build();

        assert_eq!(
            payload.nick,
            Some(TitanString::from("New Nick".to_string()))
        );
        assert_eq!(payload.mute, Some(true));
        assert_eq!(payload.deaf, Some(false));
    }

    #[test]
    fn test_start_thread_builder() {
        let payload = StartThreadBuilder::new("Thread Name")
            .auto_archive_duration(60)
            .kind(11) // Public Thread
            .build();

        assert_eq!(payload.name, "Thread Name".to_string());
        assert_eq!(payload.auto_archive_duration, Some(60));
        assert_eq!(payload.type_, Some(11));
    }
}

// ============================================================================
// Create Invite Builder
// ============================================================================

/// Payload for creating an invite.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateInvite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<crate::Snowflake>,
}

/// Builder for creating an Invite.
#[derive(Debug, Clone, Default)]
pub struct CreateInviteBuilder {
    params: CreateInvite,
}

impl CreateInviteBuilder {
    /// Create a new CreateInviteBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set max age in seconds (0 = never expire).
    pub fn max_age(mut self, seconds: u32) -> Self {
        self.params.max_age = Some(seconds);
        self
    }

    /// Set max uses (0 = unlimited).
    pub fn max_uses(mut self, uses: u32) -> Self {
        self.params.max_uses = Some(uses);
        self
    }

    /// Set temporary (kick after disconnect).
    pub fn temporary(mut self, temp: bool) -> Self {
        self.params.temporary = Some(temp);
        self
    }

    /// Set unique (don't reuse similar invite).
    pub fn unique(mut self, unique: bool) -> Self {
        self.params.unique = Some(unique);
        self
    }

    /// Build the CreateInvite payload.
    pub fn build(self) -> CreateInvite {
        self.params
    }
}

// ============================================================================
// Create Emoji Builder
// ============================================================================

/// Payload for creating an emoji.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateEmoji {
    pub name: String,
    pub image: String, // Data URI
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<crate::Snowflake>,
}

/// Builder for creating an Emoji.
#[derive(Debug, Clone)]
pub struct CreateEmojiBuilder {
    params: CreateEmoji,
}

impl CreateEmojiBuilder {
    /// Create a new CreateEmojiBuilder.
    /// `image_data` should be a Data URI Scheme string (e.g. "data:image/jpeg;base64,...").
    pub fn new(name: impl Into<String>, image_data: impl Into<String>) -> Self {
        Self {
            params: CreateEmoji {
                name: name.into(),
                image: image_data.into(),
                roles: Vec::new(),
            },
        }
    }

    /// Add a role that can use this emoji.
    pub fn role(mut self, role_id: impl Into<crate::Snowflake>) -> Self {
        self.params.roles.push(role_id.into());
        self
    }

    /// Build the CreateEmoji payload.
    pub fn build(self) -> CreateEmoji {
        self.params
    }
}

#[cfg(test)]
mod final_tests {
    use super::*;
    use crate::Mention;
    use crate::Snowflake; // Ensure Mention trait is in scope

    #[test]
    fn test_create_invite_builder() {
        let payload = CreateInviteBuilder::new()
            .max_age(86400)
            .max_uses(10)
            .unique(true)
            .build();

        assert_eq!(payload.max_age, Some(86400));
        assert_eq!(payload.max_uses, Some(10));
        assert_eq!(payload.unique, Some(true));
    }

    #[test]
    fn test_create_emoji_builder() {
        let payload = CreateEmojiBuilder::new("test_emoji", "data:image/png;base64,...")
            .role(Snowflake(12345))
            .build();

        assert_eq!(payload.name, "test_emoji");
        assert_eq!(payload.roles.len(), 1);
    }

    #[test]
    fn test_mention_trait() {
        let user = crate::User {
            id: Snowflake(123),
            username: "test".to_string().into(),
            discriminator: "0000".to_string().into(),
            global_name: None,
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
            avatar_decoration_data: None,
        };

        // This test relies on Mention being implemented for User
        // Since Mention is in lib.rs, we might need to import it properly.
        // But here we are in builder.rs, so crate::Mention works if public.
        // Wait, builder.rs is a module, lib.rs creates the crate. crate::Mention is correct.

        let mention = user.mention();
        assert_eq!(mention, "<@123>");
    }

    #[test]
    fn test_add_file_builder() {
        let msg = MessageBuilder::new()
            .content("With file")
            .add_file("test.txt", vec![1, 2, 3])
            .build();

        assert_eq!(msg.files.len(), 1);
        assert_eq!(msg.files[0].filename, "test.txt");
        assert_eq!(msg.files[0].data, vec![1, 2, 3]);
    }
}
#[cfg(test)]
mod optimization_tests {
    use super::*;
    use TitanString;

    #[test]
    fn test_embed_builder_zero_allocation() {
        let title = "Static Title";
        // Should accept &str directly
        let embed = EmbedBuilder::new().title(title).build();
        match embed.title {
            Some(TitanString::Borrowed(t)) => assert_eq!(t, "Static Title"),
            _ => panic!("Expected Borrowed Cow for static string"),
        }
    }

    #[test]
    fn test_embed_builder_owned() {
        let title = String::from("Owned Title");
        let embed = EmbedBuilder::new().title(title).build();
        match embed.title {
            Some(TitanString::Owned(t)) => assert_eq!(t, "Owned Title"),
            _ => panic!("Expected Owned Cow for String"),
        }
    }

    #[test]
    fn test_component_builder_zero_allocation() {
        let id = "custom_id";
        let btn = ButtonBuilder::new().custom_id(id).build();
        if let crate::Component::Button(b) = btn {
            match b.custom_id {
                Some(TitanString::Borrowed(s)) => assert_eq!(s, "custom_id"),
                _ => panic!("Expected Borrowed Cow for button custom_id"),
            }
        }
    }
}

// ============================================================================
// Create Sticker Builder
// ============================================================================

/// Payload for creating a sticker.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateSticker {
    pub name: String,
    pub description: String,
    pub tags: String,
}

/// Builder for creating a Sticker.
#[derive(Debug, Clone)]
pub struct CreateStickerBuilder {
    params: CreateSticker,
}

impl CreateStickerBuilder {
    /// Create a new CreateStickerBuilder.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        tags: impl Into<String>,
    ) -> Self {
        Self {
            params: CreateSticker {
                name: name.into(),
                description: description.into(),
                tags: tags.into(),
            },
        }
    }

    /// Build the CreateSticker payload.
    pub fn build(self) -> CreateSticker {
        self.params
    }
}

// ============================================================================
// Modify Emoji Builder
// ============================================================================

/// Payload for modifying an emoji.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ModifyEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::Snowflake>>,
}

/// Builder for modifying an Emoji.
#[derive(Debug, Clone, Default)]
pub struct ModifyEmojiBuilder {
    params: ModifyEmoji,
}

impl ModifyEmojiBuilder {
    /// Create a new ModifyEmojiBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    /// Set roles.
    pub fn roles(mut self, roles: Vec<crate::Snowflake>) -> Self {
        self.params.roles = Some(roles);
        self
    }

    /// Build the ModifyEmoji payload.
    pub fn build(self) -> ModifyEmoji {
        self.params
    }
}

// ============================================================================
// Stage Instance Builder
// ============================================================================

/// Payload for creating a stage instance.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateStageInstance {
    pub channel_id: crate::Snowflake,
    pub topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<crate::stage::StagePrivacyLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_start_notification: Option<bool>,
}

/// Builder for creating a Stage Instance.
#[derive(Debug, Clone)]
pub struct StageInstanceBuilder {
    params: CreateStageInstance,
}

impl StageInstanceBuilder {
    /// Create a new StageInstanceBuilder.
    pub fn new(channel_id: impl Into<crate::Snowflake>, topic: impl Into<String>) -> Self {
        Self {
            params: CreateStageInstance {
                channel_id: channel_id.into(),
                topic: topic.into(),
                privacy_level: None,
                send_start_notification: None,
            },
        }
    }

    /// Set privacy level.
    #[inline]
    pub fn privacy_level(mut self, level: crate::stage::StagePrivacyLevel) -> Self {
        self.params.privacy_level = Some(level);
        self
    }

    /// Set send start notification.
    #[inline]
    pub fn send_start_notification(mut self, send: bool) -> Self {
        self.params.send_start_notification = Some(send);
        self
    }

    /// Build the CreateStageInstance payload.
    #[inline]
    pub fn build(self) -> CreateStageInstance {
        self.params
    }
}

// ============================================================================
// Guild Builders (Basic)
// ============================================================================

/// Payload for creating a Guild.
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateGuild {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<u8>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<crate::json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<crate::json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<crate::Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<u64>,
}

/// Builder for creating a Guild.
#[derive(Debug, Clone)]
pub struct CreateGuildBuilder {
    params: CreateGuild,
}

impl CreateGuildBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            params: CreateGuild {
                name: name.into(),
                ..Default::default()
            },
        }
    }

    #[inline]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.params.icon = Some(icon.into());
        self
    }

    pub fn verification_level(mut self, level: u8) -> Self {
        self.params.verification_level = Some(level);
        self
    }

    pub fn build(self) -> CreateGuild {
        self.params
    }
}
