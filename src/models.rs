use std::collections::HashMap;
use std::fmt;

pub struct Host {
    pub hostname: String,
    pub port: u32,
}

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1
pub enum Method {
    OPTION,
    GET,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
    EXTENSION(String),
}

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1
pub struct RequestLine {
    pub method: Method,
    pub uri: String,
    pub v_major: u32,
    pub v_minor: u32,
}

pub enum Header {
    Host(Host),
    UserAgent(String),
    Accept(String),
    ContentType(String),
    ContentLength(usize),
    Other(String, String),
}

pub struct HttpRequest {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Header {
    pub fn to_key_value(&self) -> (String, String) {
        match self {
            Self::Host(v) => ("Host".to_string(), v.to_string()),
            Self::UserAgent(v) => ("UserAgent".to_string(), v.clone()),
            Self::Accept(v) => ("Accept".to_string(), v.clone()),
            Self::ContentType(v) => ("ContentType".to_string(), v.clone()),
            Self::ContentLength(v) => ("ContentLength".to_string(), v.to_string()),
            Self::Other(k, v) => (k.clone(), v.clone()),
        }
    }
}

//////////////////////////////////////// Display and Debug
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::OPTION => write!(f, "OPTION"),
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::TRACE => write!(f, "TRACE"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::EXTENSION(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hostname, self.port)
    }
}

impl fmt::Debug for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Host")
            .field("hostname", &self.hostname)
            .field("port", &self.port)
            .finish()
    }
}

impl fmt::Debug for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RequestLine")
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("v_major", &self.v_major)
            .field("v_minor", &self.v_minor)
            .finish()
    }
}

impl fmt::Display for RequestLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} HTTP/{}.{} {}",
            self.method, self.v_major, self.v_minor, self.uri
        )
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = write!(f, "{}\nHeaders:", self.request_line);
        for (key, value) in &self.headers {
            ret = write!(f, "\n{}: {}", key, value);
        }
        if let Some(body) = &self.body {
            ret = write!(f, "\nBody: {}", body);
        }
        ret
    }
}

impl fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
