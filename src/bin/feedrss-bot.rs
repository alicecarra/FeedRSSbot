use rssbot::{
    bot::routes::start::start,
    bot::{
        routes::{invalid_command::invalid_command, main_menu::main_menu},
        BotCommand, MyStorage, State,
    },
};
use teloxide::{
    dispatching::dialogue::{serializer::Json, ErasedStorage, SqliteStorage, Storage},
    prelude::*,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting FeedRSS bot...");

    let bot = Bot::from_env();
    let storage: MyStorage = SqliteStorage::open("db.sqlite", Json)
        .await
        .unwrap()
        .erase();

    let handler = Update::filter_message()
        .enter_dialogue::<Message, ErasedStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(
            dptree::case![State::Main(user)]
                .branch(
                    dptree::entry()
                        .filter_command::<BotCommand>()
                        .endpoint(main_menu),
                )
                .branch(dptree::endpoint(invalid_command)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

// async fn answer(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
//     match cmd {
//         BotCommand::Help => {
//             bot.send_message(msg.chat.id, BotCommand::descriptions().to_string())
//                 .await?
//         }
//         BotCommand::Get(url) => {
//             let mut feed = match Feed::new(url).await {
//                 Ok(feed) => feed,
//                 Err(_) => {
//                     bot.send_message(msg.chat.id, "Error with the url.").await?;

//                     return Ok(());
//                 }
//             };

//             for item in feed.get_unread_items().await.unwrap() {
//                 let regex = Regex::new(r"<[^>]*>").unwrap();

//                 bot.send_message(
//                     msg.chat.id,
//                     format!(
//                         "{}\n\n{}.",
//                         item.title().unwrap(),
//                         regex.replacen(item.description().unwrap(), 0, "")
//                     ),
//                 )
//                 .await
//                 .unwrap();
//             }
//             bot.send_message(msg.chat.id, "Thats all :)").await?
//         }
//     };

//     Ok(())
// }
