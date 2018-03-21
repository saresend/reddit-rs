extern crate reqwest;

use std::io::Result;
use std::collections::HashMap;

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
        let url = client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(client_id, Some(password))
            .json(&map)
            .send()
            .unwrap();
        unimplemented!();
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
