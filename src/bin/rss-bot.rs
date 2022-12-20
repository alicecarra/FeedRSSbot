use rss::{Channel, ChannelBuilder};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let content = reqwest::get("https://rss.art19.com/apology-line")
    //     .await
    //     .unwrap()
    //     .bytes()
    //     .await
    //     .unwrap();
    // //println!("{content:?}");

    // let channel = Channel::read_from(&content[..])?;

    // //println!("\nChannel:\n{channel:?}");

    // let items = channel.items();

    // for item in items {
    //     println!("\nItems:\n{item:?}");
    // }

    Ok(())
}
