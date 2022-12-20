use anyhow::anyhow;
use rss::Channel;

pub enum FeedType {
    Rss,
}

pub struct Feed {
    pub channel: Channel,
    pub feed_type: FeedType,
}

impl Feed {
    pub async fn new(input_link: String) -> Result<Self, anyhow::Error> {
        let content = reqwest::get(input_link.clone()).await?.bytes().await?;
        match Channel::read_from(&content[..]) {
            Ok(channel) => Ok(Self {
                channel,
                feed_type: FeedType::Rss,
            }),
            Err(_) => Err(anyhow!("Error creating a feed from this url")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Feed;

    #[tokio::test]
    async fn creating_feed() {
        let feed = Feed::new("https://rss.art19.com/apology-line".to_owned())
            .await
            .unwrap();

        assert_eq!(feed.channel.title(), "The Apology Line");
    }
}
