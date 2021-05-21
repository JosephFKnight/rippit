use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subreddit {
    name: String,
    subscriber_count: i32,
    mods: Vec<Redditor>,
}
