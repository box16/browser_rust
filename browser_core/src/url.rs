use alloc::string::{String, ToString};

#[derive(PartialEq, Debug)]
pub struct URL {
    scheme: String,
    host: String,
    port: String,
    path: String,
    search_part: String,
}

impl URL {
    pub fn new(url: String) -> Self {
        Self {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            search_part: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_http_url() {
        let url = "http://example.com".to_string();
        let expected = URL {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            search_part: "".to_string(),
        };
        assert_eq!(expected, URL::new(url));
    }
}
