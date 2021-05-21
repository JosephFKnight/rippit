use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redditor {
    username: String,
    post_karma: i32,
    comment_karma: i32,
    active_since: String
}

