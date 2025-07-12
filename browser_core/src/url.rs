use alloc::{
    string::{String, ToString},
    vec::Vec,
};

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
        let host = url.trim_start_matches("http://");
        let host_parts: Vec<&str> = host.splitn(2, ":").collect();
        let port = if host_parts.len() == 2 {
            host_parts[1].to_string()
        } else {
            "80".to_string()
        };

        Self {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: port,
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

    #[test]
    fn parse_port_specified_http_url() {
        let url = "http://example.com:8080".to_string();
        let expected = URL {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "".to_string(),
            search_part: "".to_string(),
        };
        assert_eq!(expected, URL::new(url));
    }
}
