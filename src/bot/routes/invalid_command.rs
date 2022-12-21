use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    bot::{HandlerResult, MyDialogue},
    user::User,
};

pub async fn invalid_command(
    bot: Bot,
    _dialogue: MyDialogue,
    msg: Message,
    _user: User,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Invalid Command").await?;

    Ok(())
}
