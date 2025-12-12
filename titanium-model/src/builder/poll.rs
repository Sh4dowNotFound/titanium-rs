use crate::TitanString;

/// Builder for creating a Poll.
#[derive(Debug, Clone)]
pub struct PollBuilder<'a> {
    poll: crate::message::Poll<'a>,
}

impl<'a> PollBuilder<'a> {
    /// Create a new `PollBuilder`.
    pub fn new(question: impl Into<TitanString<'a>>) -> Self {
        Self {
            poll: crate::message::Poll {
                question: crate::message::PollMedia {
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
    pub fn answer(mut self, answer: impl Into<crate::message::PollAnswer<'a>>) -> Self {
        self.poll.answers.push(answer.into());
        self
    }

    /// Set expiry.
    pub fn expiry(mut self, expiry: impl Into<TitanString<'a>>) -> Self {
        self.poll.expiry = Some(expiry.into());
        self
    }

    /// Allow multiselect.
    #[must_use]
    pub fn allow_multiselect(mut self, allow: bool) -> Self {
        self.poll.allow_multiselect = allow;
        self
    }

    /// Build the Poll.
    #[must_use]
    pub fn build(self) -> crate::message::Poll<'a> {
        self.poll
    }
}
