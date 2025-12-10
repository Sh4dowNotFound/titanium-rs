//! Discord Poll Object structures.

// Import removed

use crate::builder::PollBuilder;
use crate::TitanString;
use serde::{Deserialize, Serialize};

/// A poll object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Poll<'a> {
    /// The question of the poll.
    pub question: PollMedia<'a>,

    /// Each of the answers available in the poll.
    pub answers: Vec<PollAnswer<'a>>,

    /// The expiration date of the poll.
    #[serde(default)]
    pub expiry: Option<TitanString<'a>>,

    /// Whether the poll allows multiple selections.
    #[serde(default)]
    pub allow_multiselect: bool,

    /// The layout type of the poll.
    #[serde(default)]
    pub layout_type: Option<u8>,

    /// The results of the poll.
    #[serde(default)]
    pub results: Option<PollResults>,
}

impl<'a> Poll<'a> {
    /// Create a builder for a Poll.
    pub fn builder(question: impl Into<TitanString<'a>>) -> PollBuilder<'a> {
        PollBuilder::new(question)
    }
}

/// A poll media object (question text/emoji).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollMedia<'a> {
    /// The text of the field.
    #[serde(default)]
    pub text: Option<TitanString<'a>>,

    /// The emoji of the field.
    #[serde(default)]
    pub emoji: Option<crate::member::Emoji<'a>>,
}

/// A poll answer object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollAnswer<'a> {
    /// The ID of the answer.
    #[serde(default)]
    // Read-only usually, but might be sent in some contexts? Docs say answer_id is integer.
    pub answer_id: Option<u32>, // ID is usually an integer for poll answers, not snowflake? API says "integer".

    /// The data of the answer.
    pub poll_media: PollMedia<'a>,
}

/// The results of a poll.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollResults {
    /// Whether the poll is finalized.
    #[serde(default)]
    pub is_finalized: bool,

    /// The counts for each answer.
    #[serde(default)]
    pub answer_counts: Vec<PollAnswerCount>,
}

/// Count for a specific answer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollAnswerCount {
    /// The ID of the answer.
    pub id: u32,

    /// The number of votes.
    pub count: u32,

    /// Whether the current user voted for this answer.
    #[serde(default)]
    pub me_voted: bool,
}
