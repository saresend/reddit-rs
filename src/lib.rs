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
use reqwest::header::ContentType;

use serde::Serialize;

pub struct Reddit {
    token: String,
    user_agent: String,
    url: Url,
    credentials: HashMap<String, String>,
    client_id: String,
    client_secret: String,
}

impl Reddit {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        username: &str,
        password: &str,
    ) -> Result<Reddit> {
        let mut map = HashMap::new();
        map.insert("grant_type".to_string(), "password".to_string());
        map.insert("username".to_string(), username.to_string());
        map.insert("password".to_string(), password.to_string());
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
            credentials: map,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        })
    }
    fn refresh_token(&mut self) {
        let client = reqwest::Client::new();
        let mut result = client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(self.client_id.clone(), Some(self.client_secret.clone()))
            .form(&self.credentials)
            .send()
            .unwrap();
        let auth_response: AuthResponse = result.json().unwrap();
        self.token = auth_response.access_token;
    }

    fn execute<T: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        method: Method,
        data: Option<&T>,
    ) -> Result<String> {
        let client = reqwest::Client::new();
        let url = self.url.clone().into_string() + endpoint;
        println!("{}", url);
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer {
            token: self.token.clone(),
        }));
        match method {
            Method::Get => Ok(client
                .get(&url)
                .headers(headers)
                .query(&data)
                .send()?
                .text()?),
            Method::Post => Ok(client
                .post(&url)
                .headers(headers)
                .json(&data)
                .send()?
                .text()?),

            _ => unimplemented!(),
        }
    }

    pub fn me(&self) -> Result<RedditUser> {
        match self.execute("me", Method::Get, None as Option<&str>) {
            Ok(res) => Ok(serde_json::from_str(&res).unwrap()),
            Err(err) => Err(err),
        }
    }

    pub fn get_subreddits(&self, query: &str) {
        //Stuct to support search functionality for subreddit
        #[derive(Serialize)]
        struct Search {
            query: String,
            exact: bool,
            include_over_18: bool,
            include_unadvertisable: bool,
        }

        let query = Search {
            query: query.to_string(),
            exact: false,
            include_over_18: true,
            include_unadvertisable: true,
        };

        match self.execute("api/search_reddit_names", Method::Get, Some(&query)) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("{}", err),
        }
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
    #[test]
    fn test_me_endpoint() {
        dotenv::dotenv().ok();
        let reddit = Reddit::new(
            &env::var("client_id").unwrap(),
            &env::var("client_secret").unwrap(),
            &env::var("username").unwrap(),
            &env::var("password").unwrap(),
        ).unwrap();
        if let Ok(_) = reddit.me() {

        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_subreddit_search() {
        dotenv::dotenv().ok();
        let reddit = Reddit::new(
            &env::var("client_id").unwrap(),
            &env::var("client_secret").unwrap(),
            &env::var("username").unwrap(),
            &env::var("password").unwrap(),
        ).unwrap();

        reddit.get_subreddits("rust");
    }

}
