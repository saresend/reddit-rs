extern crate reqwest;

use std::collections::HashMap;
use reqwest::Result;

pub struct Reddit {}

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
        match client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(client_id, Some(client_secret))
            .form(&map)
            .send()
        {
            Ok(mut response) => {
                println!("{}", response.status());
                println!("{}", response.url());
                println!("{}", response.text().unwrap());
                Ok(Reddit {})
            }
            Err(err) => Err(err),
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
}
