use reqwest::{header, Client, Error};
use std::{collections::HashMap, convert::TryFrom};
use self::;

pub async fn get_reddit() -> Result<Submission, Error> {

    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("Bearer "),
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
        .json::<Submission>()
        .await;
    resp
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
