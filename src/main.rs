use reqwest::{header, Client};
use rippit::get_reddit;
use std::convert::TryFrom;

#[tokio::main]
async fn main() {
    let submission = get_reddit();
    println!("{:?}", submission);
}
