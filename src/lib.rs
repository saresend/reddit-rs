extern crate reqwest;

use std::io::Result;

pub struct Reddit {}

impl Reddit {
    pub fn new(client_id: &str) -> Result<Reddit> {
        let client = reqwest::Client::new();
        let url = client
            .get("https://www.reddit.com/api/v1/authorize")
            .query(&[("response_type", "code"), ("client_id", client_id)])
            .send()
            .unwrap();
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
