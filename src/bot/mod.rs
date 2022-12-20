use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum BotCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Get posts from rss feed (usage: /get <url>).")]
    Get(String),
}
