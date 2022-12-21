use teloxide::{requests::Requester, types::Message, utils::command::BotCommands, Bot};

use crate::{
    bot::{BotCommand, HandlerResult, MyDialogue, State},
    user::User,
};

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let chat_id = msg.chat.id.0;
    let user = User::new(
        chat_id,
        msg.chat.username().map(|u| u.to_string()),
        msg.chat.first_name().map(|n| n.to_string()),
    );

    if user.name.is_some() {
        bot.send_message(
            msg.chat.id,
            format!("Hello {}.", user.name.clone().unwrap()),
        )
        .await?;
    }

    bot.send_message(msg.chat.id, "Please select a command")
        .await?;
    bot.send_message(msg.chat.id, BotCommand::descriptions().to_string())
        .await?;

    dialogue.update(State::Main(user)).await?;

    Ok(())
}
