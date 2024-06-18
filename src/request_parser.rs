use crate::models::Host;
use std::io::{Read, BufReader, BufRead};
use std::fmt;

/// An HttpElemIterator implements `http_elems` which is
/// similar to BufReader.lines(), but for HttpElems instead of 
/// Lines
pub trait HttpElemIterator<R: Read> {
    fn http_elems(self) -> HttpElems<R>;
}

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1
enum Method {
    OPTION,
    GET,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
    EXTENSION(String),
}

impl Method {
    /// Converts a method string to a method. If no method is found
    /// creates an Extension method
    ///
    /// TODO - validate string for extension method (in rfc, it's a "token")
    /// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1
    ///
    /// * `data`: The method string
    fn from(data: &String) -> Method {
        match data.as_str() {
            "OPTION" => Self::OPTION,
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "TRACE" => Self::TRACE,
            "CONNECT" => Self::CONNECT,
            _ => Self::EXTENSION(data.clone()),
        }
    }
}

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

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1
pub struct RequestLine {
    method: Method,
    uri: String,
    v_major: usize,
    v_minor: usize,
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

pub struct GeneralHeader {
    key: String,
    value: String,
}

impl fmt::Debug for GeneralHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GeneralHeader")
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}

impl fmt::Display for GeneralHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

/// TODO - follow the spec - https://datatracker.ietf.org/doc/html/rfc2616#section-5
/*
enum RequestHeader {
    Accept(String),
    AcceptCharset(String),
    AcceptEncoding(String),
    AcceptLanguage(String),
    Authorization(String),
    Expect(String),
    From(String),
    Host(Host),
    IfMatch(String)
}
*/

pub enum Header {
    Host(Host),
    UserAgent(String),
    Accept(String),
    ContentType(String),
    ContentLength(usize),
    Other(GeneralHeader),
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::Host(host) => write!(f, "HOST {}", host),
            Header::UserAgent(string) => write!(f, "USER_AGENT {}", string),
            Header::Accept(string) => write!(f, "ACCEPT {}", string),
            Header::ContentType(string) => write!(f, "CONTENT TYPE {}", string),
            Header::ContentLength(usize) => write!(f, "CONTENT LENGTH {}", usize),
            Header::Other(genericheader) => write!(f, "GENERIC {}", genericheader),
        }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Header {
    fn from(key: String, value: String) -> Result<Header, String> {
        Ok(match key.as_str() {
            "Host" => match Host::from(value) {
                Ok(host) => Self::Host(host),
                Err(msg) => return Err(msg),
            },
            "User-Agent" => Self::UserAgent(value),
            "Accept" => Self::Accept(value),
            "Content-Type" => Self::ContentType(value),
            "Content-Length" => match value.parse::<usize>() {
                Ok(cl) => Self::ContentLength(cl),
                Err(msg) => return Err(msg.to_string()),
            },
            _ => Self::Other(GeneralHeader { key, value }),
        })
    }
}

pub enum HttpElem {
    RequestLine(RequestLine),
    Header(Header),
    Body(String),
}

impl fmt::Display for HttpElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RequestLine(rl) => {
                write!(f, "Request Line: {}", rl)
            }
            Self::Header(header) => {
                write!(f, "Header: {}", header)
            }
            Self::Body(body) => {
                write!(f, "Body: {}", body)
            }
        }
    }
}

impl fmt::Debug for HttpElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// `HttpElems` is very similar to the `Lines` object,
/// but instead of iterating through lines, it iterates through
/// lines up until the end of all the headers. If there's more
/// data to read, then it reads that data.
///
/// The amount of data to read is infered by the Content-Length
/// header
pub struct HttpElems<R: Read> {
    reader: BufReader<R>,
    content_length: usize,
    done: bool,
    first: bool,
}

impl<R: Read> HttpElems<R> {
    fn read_body_if_exists(&mut self) -> Option<Result<HttpElem, String>> {
        if self.content_length == 0 {
            self.done = true;
            return None;
        }

        let mut body = Vec::new();
        match self
            .reader
            .by_ref()
            .take(self.content_length as u64)
            .read_to_end(&mut body)
        {
            Ok(_) => {
                // TODO - what should happen if the number of
                // bytes read isn't the number of bytes
                // intended to be read
                self.done = true;
                Some(Ok(HttpElem::Body(
                    String::from_utf8_lossy(&body).to_string(),
                )))
            }
            Err(e) => Some(Err(e.to_string())),
        }
    }
}

impl<R: Read> HttpElemIterator<R> for BufReader<R> {
    fn http_elems(self) -> HttpElems<R> {
        HttpElems {
            reader: self,
            content_length: 0,
            done: false,
            first: true,
        }
    }
}

impl<R: Read> Iterator for HttpElems<R> {
    type Item = Result<HttpElem, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => {
                buf = buf.trim_end().to_string();
                if buf.is_empty() {
                    self.read_body_if_exists()
                } else if self.first {
                    self.first = false;
                    Some(parse_request_line(&buf).map(|rl| HttpElem::RequestLine(rl)))
                } else {
                    let ret = Some(parse_header(&buf).map(|header| HttpElem::Header(header)));
                    if let Some(Ok(HttpElem::Header(Header::ContentLength(len)))) = ret {
                        self.content_length = len;
                    }
                    ret
                }
            }
            Err(e) => Some(Err(e.to_string())),
        }
    }
}

fn parse_request_line(value: &String) -> Result<RequestLine, String> {
    let values: Vec<_> = value.split(" ").collect();
    if values.len() != 3 {
        return Err(
            "Expecting 3 values in request line: Method SP URI SP HTTP/Version".to_string(),
        );
    }
    Ok(RequestLine {
        method: Method::from(&values[0].to_string()),
        uri: values[1].to_string(),
        v_major: 1,
        v_minor: 1,
    })
}

fn parse_header(header: &String) -> Result<Header, String> {
    let parts: Vec<&str> = header.splitn(2, ":").collect();

    if parts.len() != 2 {
        return Err(
            "Invalid header, expected a colon in header to deliminate key: value".to_string(),
        );
    }

    Header::from(parts[0].trim().to_string(), parts[1].trim().to_string())
}

