use reqwest::{header, Client, Error};
use serde_json::Value;
use std::{collections::HashMap, convert::TryFrom};

pub async fn get_reddit() -> Result<Value, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("Bearer -F93r1Ty6D7_b7-jnr8BJDHmcpYAGDQ"),
    );
    let client = Client::builder()
        .user_agent("WSL(Ubuntu-20.3):rippit:0.1.0 (by user /u/i_ate_a_milkshake)")
        .default_headers(headers)
        .build()
        .unwrap();
    let resp = client
        .get("https://oauth.reddit.com/hot?limit=1")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;
    resp
}

#[derive(Debug, Clone)]
pub struct Submission {
    author: Redditor,
    // id: String,
    // num_comments: i32,
    // score: i32,
    subreddit: Subreddit,
    title: String,
    // url: String,
}

impl TryFrom<Value> for Submission {
    type Error = &'static str;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Some(kind) = v["kind"].as_str() {
            let data = &v["data"];
            Ok(Submission {
                author: Redditor { username: String::from(data["author"].as_str().ok_or("ValueError")?) },
                subreddit: Subreddit { name: String::from(data["subreddit"].as_str().ok_or("ValueError")?) },
                title: String::from(data["title"].as_str().ok_or("ValueError")?)
            })
        } else {
            Err("This object is not a subreddit.")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Redditor {
    username: String,
    // post_karma: i32,
    // comment_karma: i32,
    // active_since: String,
}
#[derive(Debug, Clone)]
pub struct Subreddit {
    name: String,
    // subscriber_count: i32,
    // mods: Vec<Redditor>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
