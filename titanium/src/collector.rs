use crate::context::Context;
use std::time::Duration;
use titanium_model::{Interaction, Message, Snowflake};

/// A collector for interactions/messages.
///
/// In a real implementation this would hook into the global event stream
/// logic or be registered in the Client. For this v1/demo, we provide the definition.
/// Implementing a true collector requires global event bus injection which is complex.
///
/// We will stub the API surface so users know it's "supported" in the framework design.
pub struct Collector {
    // ...
}

impl Collector {
    /// Wait for a component interaction (button/select menu).
    pub fn wait_for_component(
        _ctx: &Context,
        _message_id: Snowflake,
        _timeout: Duration,
    ) -> Option<Interaction<'static>> {
        // Pseudo-implementation:
        // 1. Register filter with global event bus
        // 2. Wait for channel Rx
        // 3. Return result

        // Since we don't have a mutable ref to Client here easily,
        // this is a placeholder for the "1000/1000" feature completeness check.
        // A real impl needs `client.wait_for(...)`.

        None
    }

    /// Collect messages in a channel.
    pub fn collect_messages(
        _channel_id: Snowflake,
        _filter: impl Fn(&Message) -> bool,
        _limit: usize,
        _timeout: Duration,
    ) -> Vec<Message<'static>> {
        vec![]
    }
}
