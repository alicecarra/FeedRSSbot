use regex::Regex;
use teloxide::{requests::Requester, types::Message, utils::command::BotCommands, Bot};

use crate::{
    bot::{BotCommand, HandlerResult, MyDialogue, MyStorage, State},
    feed::Feed,
    user::User,
};

pub async fn main_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    _storage: MyStorage,
    mut user: User,
    cmd: BotCommand,
) -> HandlerResult {
    match cmd {
        BotCommand::Help => {
            bot.send_message(msg.chat.id, BotCommand::descriptions().to_string())
                .await?
        }
        BotCommand::GetAll => {
            if user.feeds.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "You don't have feeds. Add one with the command /add <feed_url>",
                )
                .await?;
            }
            for mut feed in user.clone().feeds {
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
            }
            bot.send_message(msg.chat.id, "Thats all :)").await?
        }
        BotCommand::Add(input_url) => match Feed::new(input_url).await {
            Ok(feed) => {
                user.add_feed(feed);
                bot.send_message(msg.chat.id, "Feed added.").await?
            }
            Err(_) => bot.send_message(msg.chat.id, "Invalid URL").await?,
        },
        BotCommand::List => {
            bot.send_message(msg.chat.id, format!("List of feeds: {:#?}", user.feeds))
                .await?
        }
    };

    dialogue.update(State::Main(user)).await?;

    Ok(())
}
