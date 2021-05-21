mod models;
use models::{RedditApiObject, Submission, Listing};
use chrono::{offset::Utc, DateTime};
use reqwest::{Client, Error};
use serde_derive::Deserialize;
use serde_json::Value;
use std::{cmp, fmt};

use crate::models::ListingApiObject;

#[derive(Debug, Clone, Deserialize)]
struct OauthToken {
    access_token: String,
    #[serde(skip_deserializing, default = "Utc::now")]
    time_acquired: DateTime<Utc>,
    expires_in: usize,
}

impl fmt::Display for OauthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.access_token)
    }
}


struct RedditConfig<S>
where
    S: fmt::Display,
{
    user: S,
    pass: Option<S>,
}

pub struct Reddit<S>
where
    S: fmt::Display,
{
    client: Client,
    config: RedditConfig<S>,
    token: Option<OauthToken>,
}

impl<S> Reddit<S>
where
    S: fmt::Display,
{
    pub fn new(user: S, pass: Option<S>) -> Self {
        let client = Client::builder()
            .user_agent("WSL(Ubuntu-20.3):rippit:0.1.0 (by user /u/i_ate_a_milkshake)")
            .build()
            .unwrap_or(Client::default());
        Self {
            client,
            config: RedditConfig { user, pass },
            token: None,
        }
    }

    pub async fn connect(&mut self) {
        self.fetch_token().await;
    }

    async fn fetch_token(&mut self) {
        self.token = self
            .client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(&self.config.user, self.config.pass.as_ref())
            .body("grant_type=https://oauth.reddit.com/grants/installed_client&device_id=DO_NOT_TRACK_THIS_DEVICE")
            .send()
            .await.unwrap()
            .json::<OauthToken>()
            .await.ok();
    }
    pub async fn get_hot(&mut self) -> Result<Vec<Submission>, Error> {
        let resp = &self
            .client
            .get("https://oauth.reddit.com/new")
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await?
        .json::<ListingApiObject>()
            .await?;
        Ok(resp.to_owned().get_listing())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_oauth_token_response() {
        let resp = r#"{
  "access_token": "-EJhAhhjL-xQFAZSRzHY4A-qVYsDhrA",
  "token_type": "bearer",
  "device_id": "DO_NOT_TRACK_THIS_DEVICE",
  "expires_in": 3600,
  "scope": "*"
}
"#;
        let now = Utc::now();
        let expected = OauthToken {
            access_token: "-EJhAhhjL-xQFAZSRzHY4A-qVYsDhrA".to_owned(),
            time_acquired: now,
            expires_in: 3600usize,
        };
        let deser = serde_json::from_str::<OauthToken>(resp).unwrap();
        assert_eq!(deser.access_token, expected.access_token);
        assert_eq!(deser.expires_in, 3600usize);
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
