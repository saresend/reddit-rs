extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;

extern crate url;

mod models;

use std::collections::HashMap;
use reqwest::{Request, Result};

use models::*;
use url::Url;
use reqwest::Method;
use hyper::header::{Authorization, Bearer, Headers};

pub struct Reddit {
    token: String,
    user_agent: String,
    url: Url,
}

impl Reddit {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        username: &str,
        password: &str,
    ) -> Result<Reddit> {
        let mut map = HashMap::new();
        map.insert("grant_type", "password");
        map.insert("username", username);
        map.insert("password", password);
        let client = reqwest::Client::new();
        let mut result = client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(client_id, Some(client_secret))
            .form(&map)
            .send()?;
        let auth_response: AuthResponse = result.json()?;
        Ok(Reddit {
            token: auth_response.access_token,
            user_agent: String::from(format!("reddit-rs/0.1 by {}", username)),
            url: Url::parse("https://oauth.reddit.com/api/v1/").unwrap(),
        })
    }

    fn execute(&self, endpoint: &str, http_method: Method) -> Result<String> {
        let url = self.url.join(endpoint).unwrap();
        let mut request = Request::new(http_method, url);

        {
            let headers = request.headers_mut();
            headers.set(Authorization(Bearer {
                token: self.token.clone(),
            }));
        }

        let client = reqwest::Client::new();
        let mut response = client.execute(request)?;
        Ok(response.text()?)
    }
}

#[cfg(test)]
extern crate dotenv;
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_authorization() {
        dotenv::dotenv().ok();
        if let Ok(reddit) = Reddit::new(
            &env::var("client_id").unwrap(),
            &env::var("client_secret").unwrap(),
            &env::var("username").unwrap(),
            &env::var("password").unwrap(),
        ) {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
