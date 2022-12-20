use crate::feed::Feed;

pub struct User {
    pub chat_id: i64,
    pub feeds: Vec<Feed>,
}
