use serde::{Deserialize, Serialize};

use crate::feed::Feed;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub chat_id: i64,
    pub username: Option<String>,
    pub name: Option<String>,
    pub feeds: Vec<Feed>,
}

impl User {
    pub fn new(chat_id: i64, username: Option<String>, name: Option<String>) -> Self {
        Self {
            chat_id,
            feeds: vec![],
            username,
            name,
        }
    }

    pub fn add_feed(&mut self, feed: Feed) {
        self.feeds.push(feed)
    }
}
