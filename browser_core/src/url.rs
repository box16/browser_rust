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

#[derive(PartialEq, Debug)]
pub enum URLError {
    Invalid,
    Unsupported,
}

fn split(sentence: String, pattern: &str, second_default: String) -> (String, String) {
    let parsed: Vec<&str> = sentence.splitn(2, &pattern).collect();
    if parsed.len() == 2 {
        (parsed[0].to_string(), parsed[1].to_string())
    } else {
        (parsed[0].to_string(), second_default)
    }
}

impl URL {
    pub fn new(url: String) -> Result<Self, URLError> {
        let (scheme, other) = split(url, "://", "".to_owned());
        let (host, other) = split(other, "/", "".to_owned());
        let (host, port) = split(host, ":", "80".to_owned());
        let (path, other) = split(other, "?", "".to_owned());
        let (search_part, _fragment) = split(other, "#", "".to_owned());

        if host.is_empty() {
            return Err(URLError::Invalid);
        }
        if scheme != "http" {
            return Err(URLError::Unsupported);
        }
        Ok(Self {
            scheme,
            host,
            port,
            path,
            search_part,
        })
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
        let expected = Err(URLError::Unsupported);
        assert_eq!(expected, URL::new(url));
    }

    #[test]
    fn parse_empty_host_specified_url() {
        let url: String = "http://".to_string();
        let expected = Err(URLError::Invalid);
        assert_eq!(expected, URL::new(url));
    }

    #[test]
    fn parse_not_url() {
        let url: String = "aaabbbcc".to_string();
        let expected = Err(URLError::Invalid);
        assert_eq!(expected, URL::new(url));
    }
}
