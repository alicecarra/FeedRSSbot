pub mod routes;

use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::ErasedStorage, macros::BotCommands, prelude::Dialogue};

use crate::user::User;

pub type MyDialogue = Dialogue<State, ErasedStorage<State>>;
pub type MyStorage = std::sync::Arc<ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start,
    Main(User),
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum BotCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Get posts from all feeds in your list.")]
    GetAll,
    #[command(description = "Add a feed in your list (usage: /add <FEED_URL>).")]
    Add(String),
    #[command(description = "Feed list.")]
    List,
}
