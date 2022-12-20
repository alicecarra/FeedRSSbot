use regex::Regex;
use rssbot::{bot::BotCommand, feed::Feed};
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    BotCommand::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    match cmd {
        BotCommand::Help => {
            bot.send_message(msg.chat.id, BotCommand::descriptions().to_string())
                .await?
        }
        BotCommand::Get(url) => {
            let mut feed = match Feed::new(url).await {
                Ok(feed) => feed,
                Err(_) => {
                    bot.send_message(msg.chat.id, "Error with the url.").await?;

                    return Ok(());
                }
            };

            for item in feed.get_unread_items().await.unwrap() {
                let regex = Regex::new(r"<[^>]*>").unwrap();

                bot.send_message(
                    msg.chat.id,
                    format!(
                        "{}\n\n{}.",
                        item.title().unwrap(),
                        regex.replacen(item.description().unwrap(), 0, "")
                    ),
                )
                .await
                .unwrap();
            }
            bot.send_message(msg.chat.id, "Thats all :)").await?
        }
    };

    Ok(())
}
