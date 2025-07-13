use alloc::{
    borrow::ToOwned,
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
    pub fn new(url: String) -> Result<Self, String> {
        let (scheme, other) = Self::pattern_match(url, "://", "".to_owned());
        let (host, other) = Self::pattern_match(other, "/", "".to_owned());
        let (host, port) = Self::pattern_match(host, ":", "80".to_owned());
        let (path, other) = Self::pattern_match(other, "?", "".to_owned());
        let (search_part, _fragment) = Self::pattern_match(other, "#", "".to_owned());

        if scheme == "http" && !host.is_empty() {
            Ok(Self {
                scheme,
                host,
                port,
                path,
                search_part,
            })
        } else if scheme != "http" {
            Err("unsupported scheme".to_string())
        } else {
            Err("invalid url".to_string())
        }
    }

    fn pattern_match(sentence: String, pattern: &str, default: String) -> (String, String) {
        let parsed: Vec<&str> = sentence.splitn(2, &pattern).collect();
        if parsed.len() == 2 {
            (parsed[0].to_string(), parsed[1].to_string())
        } else {
            (parsed[0].to_string(), default)
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
        assert_eq!(expected, URL::new(url).unwrap());
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
        assert_eq!(expected, URL::new(url).unwrap());
    }

    #[test]
    fn parse_short_path_specified_http_url() {
        let url = "http://example.com/index.html".to_string();
        let expected = URL {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            search_part: "".to_string(),
        };
        assert_eq!(expected, URL::new(url).unwrap());
    }

    #[test]
    fn parse_long_path_specified_http_url() {
        let url = "http://example.com/fizz/buzz/fizzbuzz/index.html".to_string();
        let expected = URL {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "fizz/buzz/fizzbuzz/index.html".to_string(),
            search_part: "".to_string(),
        };
        assert_eq!(expected, URL::new(url).unwrap());
    }

    #[test]
    fn parse_search_part_specified_http_url() {
        let url = "http://example.com/index.html?a=123&b=456".to_string();
        let expected = URL {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            search_part: "a=123&b=456".to_string(),
        };
        assert_eq!(expected, URL::new(url).unwrap());
    }

    #[test]
    fn parse_unsupported_scheme_specified_url() {
        let url: String = "https://example.com".to_string();
        let expected = Err("unsupported scheme".to_string());
        assert_eq!(expected, URL::new(url));
    }

    #[test]
    fn parse_empty_host_specified_url() {
        let url: String = "http://".to_string();
        let expected = Err("invalid url".to_string());
        assert_eq!(expected, URL::new(url));
    }
}
