use ahash::AHashMap;
use std::future::Future;
use std::pin::Pin;

use crate::context::Context;

use crate::error::TitaniumError;

type CommandHandler = Box<
    dyn Fn(Context) -> Pin<Box<dyn Future<Output = Result<(), TitaniumError>> + Send>>
        + Send
        + Sync,
>;
type ErrorHandler =
    Box<dyn Fn(TitaniumError, Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct Framework {
    pub commands: AHashMap<String, CommandHandler>,
    pub on_error: Option<ErrorHandler>,
}

impl Default for Framework {
    fn default() -> Self {
        Self::new()
    }
}

impl Framework {
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: AHashMap::new(),
            on_error: None,
        }
    }

    pub fn command<F, Fut>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), TitaniumError>> + Send + 'static,
    {
        self.commands.insert(
            name.to_string(),
            Box::new(move |ctx| Box::pin(handler(ctx))),
        );
        self
    }

    /// Set a global error handler for commands.
    pub fn on_error<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(TitaniumError, Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_error = Some(Box::new(move |err, ctx| Box::pin(handler(err, ctx))));
        self
    }

    /// Dispatch a command.
    pub async fn dispatch(&self, name: &str, ctx: Context) {
        if let Some(handler) = self.commands.get(name) {
            // We need to clone context for error handler if needed,
            // but Context is cheap to clone (Arc internal).
            let ctx_clone = ctx.clone();

            if let Err(err) = handler(ctx).await {
                if let Some(error_handler) = &self.on_error {
                    error_handler(err, ctx_clone).await;
                } else {
                    // Default error logging
                    eprintln!("Command execution failed: {:?}", err);
                }
            }
        }
    }
}
