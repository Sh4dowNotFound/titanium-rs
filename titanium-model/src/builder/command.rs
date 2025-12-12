use crate::TitanString;

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

    #[must_use]
    pub fn build(self) -> Self {
        self
    }
}
