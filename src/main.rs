use reqwest::{header, Client};
use rippit::{get_reddit, Submission};
use serde_json::Value;
use std::convert::TryFrom;

#[tokio::main]
async fn main() {
    let v = get_reddit().await.unwrap();
    let submission = Submission::try_from(v);
    println!("{:?}", submission);
}
