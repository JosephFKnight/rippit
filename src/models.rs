use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Listing {
    children: Vec<SubmissionApiObject>,
}

#[derive(Debug, Clone, Deserialize)]
struct SubmissionApiObject {
    kind: String,
    data: Submission,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListingApiObject {
    kind: String,
    data: Listing,
}


#[derive(Debug, Clone, Deserialize)]
enum RedditApiObjectKind {
    Listing,
    SubmissionApiObject,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedditApiObject {
    kind: String,
    data: RedditApiObjectKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redditor {
    username: String,
    post_karma: i32,
    comment_karma: i32,
    active_since: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    author: String,
    //id: String,
    num_comments: usize,
    score: usize,
    subreddit: String,
    title: String,
    url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subreddit {
    name: String,
    subscriber_count: usize,
    mods: Vec<Redditor>,
}

#[cfg(test)]
mod model_tests {}
