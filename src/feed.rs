use anyhow::anyhow;
use chrono::{DateTime, Utc};
use rss::{Channel, Item};

#[derive(Debug)]
pub struct FeedUrl(String);

impl FeedUrl {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FeedType {
    Rss,
}

#[derive(Debug)]
pub struct Feed {
    pub url: FeedUrl,
    pub feed_type: FeedType,
    pub last_read: Option<DateTime<Utc>>,
}

impl Feed {
    pub async fn new(input_url: String) -> Result<Self, anyhow::Error> {
        let content = reqwest::get(input_url.clone()).await?.bytes().await?;
        match Channel::read_from(&content[..]) {
            Ok(_) => Ok(Self {
                url: FeedUrl(input_url),
                feed_type: FeedType::Rss,
                last_read: None,
            }),
            Err(_) => Err(anyhow!("Error creating a feed from this url")),
        }
    }

    pub async fn get_unread_items(&mut self) -> Result<Vec<Item>, anyhow::Error> {
        let content = reqwest::get(self.url.get()).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut unread_items = vec![];
        for item in channel.items() {
            match item.pub_date() {
                Some(pub_date) => match DateTime::parse_from_rfc2822(pub_date) {
                    Ok(pub_date) => {
                        let pub_date = DateTime::<Utc>::from(pub_date);
                        match self.last_read {
                            Some(last_read) if pub_date > last_read => {
                                unread_items.push(item.clone())
                            }
                            None => unread_items.push(item.clone()),
                            _ => (),
                        }
                    }
                    Err(_) => return Err(anyhow!("Error parsing publication date")),
                },
                None => unread_items.push(item.clone()),
            }
        }
        self.last_read = Some(Utc::now());

        Ok(unread_items)
    }
}

#[cfg(test)]
mod tests {
    use crate::feed::FeedType;

    use super::Feed;

    #[tokio::test]
    async fn creating_feed() {
        let feed = Feed::new("https://rss.art19.com/apology-line".to_owned())
            .await
            .unwrap();

        assert_eq!(feed.url.get(), "https://rss.art19.com/apology-line");
        assert_eq!(feed.last_read, None);
        assert_eq!(feed.feed_type, FeedType::Rss);
    }

    #[tokio::test]
    async fn fetch_unred_items() {
        let mut feed = Feed::new("https://rss.art19.com/apology-line".to_owned())
            .await
            .unwrap();

        let unread_items = feed.get_unread_items().await.unwrap();
        assert_eq!(unread_items.len(), 2);
        let unread_items = feed.get_unread_items().await.unwrap();
        assert_eq!(unread_items.len(), 0);
    }
}
