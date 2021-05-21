use serde::{Deserialize, Serialize};
use rippit::{Redditor, Subreddit};

mod models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    author: Redditor,
    id: String,
    num_comments: i32,
    score: i32,
    subreddit: Subreddit,
    title: String,
    url: String,
}

