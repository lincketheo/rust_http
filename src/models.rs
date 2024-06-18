use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader, Read};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A host is a combination of hostname and port.
/// Example localhost:8080. A host is _not_ a protocol (https://localhost:8080 is not a host)
#[derive(Clone, Debug)]
pub struct Host {
    pub hostname: String,
    pub port: u32,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hostname, self.port)
    }
}

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1
#[derive(Debug)]
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

/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1
#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub uri: String,
    pub v_major: u32,
    pub v_minor: u32,
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

/// There are some set header types with specific data types
/// but to support more than the standards, [others] is also included
/// https://datatracker.ietf.org/doc/html/rfc2616#section-5.3
/// This enum is used for compile time checking for various functions to check
/// all possible headers.
#[derive(EnumIter)]
pub enum RequestHeader {
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    Expect,
    From,
    Host,
    IfMatch,
    IfModifiedSince,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    MaxForwards,
    ProxyAuthorization,
    Range,
    Referer,
    TE,
    UserAgent,
}

#[derive(EnumIter)]
pub enum GeneralHeader {
    CacheControl,
    Connection,
    Date,
    Pragma,
    Trailer,
    TransferEncoding,
    Upgrade,
    Via,
    Warning,
}

#[derive(EnumIter)]
pub enum EntityHeader {
    Allow,
    ContentEncoding,
    ContentLanguages,
    ContentLength,
    ContentLocation,
    ContentMD5,
    ContentRange,
    ContentType,
    Expires,
    LastModified,
    Extension(String),
}

#[derive(EnumIter)]
pub enum ResponseHeader {
    AcceptRanges,
    Age,
    ETag,
    Location,
    ProxyAuthenticate,
    RetryAfter,
    Server,
    Vary,
    WWWAuthenticate,
}

impl RequestHeader {
    fn value(&self) -> &'static str {
        match self {
            RequestHeader::Accept => "Accept",
            RequestHeader::AcceptCharset => "Accept-Charset",
            RequestHeader::AcceptEncoding => "Accept-Encoding",
            RequestHeader::AcceptLanguage => "Accept-Language",
            RequestHeader::Authorization => "Authorization",
            RequestHeader::Expect => "Expect",
            RequestHeader::From => "From",
            RequestHeader::Host => "Host",
            RequestHeader::IfMatch => "If-Match",
            RequestHeader::IfModifiedSince => "If-Modified-Since",
            RequestHeader::IfNoneMatch => "If-None-Match",
            RequestHeader::IfRange => "If-Range",
            RequestHeader::IfUnmodifiedSince => "If-Unmodified-Since",
            RequestHeader::MaxForwards => "Max-Forwards",
            RequestHeader::ProxyAuthorization => "Proxy-Authorization",
            RequestHeader::Range => "Range",
            RequestHeader::Referer => "Referer",
            RequestHeader::TE => "TE",
            RequestHeader::UserAgent => "User-Agent",
        }
    }

    fn from(key: &str) -> Option<Self> {
        Some(match key {
            "Accept" => RequestHeader::Accept,
            "Accept-Charset" => RequestHeader::AcceptCharset,
            "Accept-Encoding" => RequestHeader::AcceptEncoding,
            "Accept-Language" => RequestHeader::AcceptLanguage,
            "Authorization" => RequestHeader::Authorization,
            "Expect" => RequestHeader::Expect,
            "From" => RequestHeader::From,
            "Host" => RequestHeader::Host,
            "If-Match" => RequestHeader::IfMatch,
            "If-Modified-Since" => RequestHeader::IfModifiedSince,
            "If-None-Match" => RequestHeader::IfNoneMatch,
            "If-Range" => RequestHeader::IfRange,
            "If-Unmodified-Since" => RequestHeader::IfUnmodifiedSince,
            "Max-Forwards" => RequestHeader::MaxForwards,
            "Proxy-Authorization" => RequestHeader::ProxyAuthorization,
            "Range" => RequestHeader::Range,
            "Referer" => RequestHeader::Referer,
            "TE" => RequestHeader::TE,
            "User-Agent" => RequestHeader::UserAgent,
            _ => return None,
        })
    }
}

impl GeneralHeader {
    fn value(&self) -> &'static str {
        match self {
            GeneralHeader::CacheControl => "CacheControl",
            GeneralHeader::Connection => "Connection",
            GeneralHeader::Date => "Date",
            GeneralHeader::Pragma => "Pragma",
            GeneralHeader::Trailer => "Trailer",
            GeneralHeader::TransferEncoding => "TransferEncoding",
            GeneralHeader::Upgrade => "Upgrade",
            GeneralHeader::Via => "Via",
            GeneralHeader::Warning => "Warning",
        }
    }

    fn from(key: &str) -> Option<Self> {
        Some(match key {
            "CacheControl" => GeneralHeader::CacheControl,
            "Connection" => GeneralHeader::Connection,
            "Date" => GeneralHeader::Date,
            "Pragma" => GeneralHeader::Pragma,
            "Trailer" => GeneralHeader::Trailer,
            "TransferEncoding" => GeneralHeader::TransferEncoding,
            "Upgrade" => GeneralHeader::Upgrade,
            "Via" => GeneralHeader::Via,
            "Warning" => GeneralHeader::Warning,
            _ => return None,
        })
    }
}

impl EntityHeader {
    fn value(&self) -> &str {
        match self {
            EntityHeader::Allow => "Allow",
            EntityHeader::ContentEncoding => "ContentEncoding",
            EntityHeader::ContentLanguages => "ContentLanguages",
            EntityHeader::ContentLength => "ContentLength",
            EntityHeader::ContentLocation => "ContentLocation",
            EntityHeader::ContentMD5 => "ContentMD5",
            EntityHeader::ContentRange => "ContentRange",
            EntityHeader::ContentType => "ContentType",
            EntityHeader::Expires => "Expires",
            EntityHeader::LastModified => "LastModified",
            EntityHeader::Extension(s) => s,
        }
    }

    fn from(key: &str) -> Option<Self> {
        Some(match key {
            "Allow" => EntityHeader::Allow,
            "ContentEncoding" => EntityHeader::ContentEncoding,
            "ContentLanguages" => EntityHeader::ContentLanguages,
            "ContentLength" => EntityHeader::ContentLength,
            "ContentLocation" => EntityHeader::ContentLocation,
            "ContentMD5" => EntityHeader::ContentMD5,
            "ContentRange" => EntityHeader::ContentRange,
            "ContentType" => EntityHeader::ContentType,
            "Expires" => EntityHeader::Expires,
            "LastModified" => EntityHeader::LastModified,
            s => EntityHeader::Extension(s.to_string()),
        })
    }
}

impl ResponseHeader {
    fn value(&self) -> &'static str {
        match self {
            ResponseHeader::AcceptRanges => "AcceptRanges",
            ResponseHeader::Age => "Age",
            ResponseHeader::ETag => "ETag",
            ResponseHeader::Location => "Location",
            ResponseHeader::ProxyAuthenticate => "ProxyAuthenticate",
            ResponseHeader::RetryAfter => "RetryAfter",
            ResponseHeader::Server => "Server",
            ResponseHeader::Vary => "Vary",
            ResponseHeader::WWWAuthenticate => "WWWAuthenticate",
        }
    }

    fn from(key: &str) -> Option<Self> {
        Some(match key {
            "AcceptRanges" => ResponseHeader::AcceptRanges,
            "Age" => ResponseHeader::Age,
            "ETag" => ResponseHeader::ETag,
            "Location" => ResponseHeader::Location,
            "ProxyAuthenticate" => ResponseHeader::ProxyAuthenticate,
            "RetryAfter" => ResponseHeader::RetryAfter,
            "Server" => ResponseHeader::Server,
            "Vary" => ResponseHeader::Vary,
            "WWWAuthenticate" => ResponseHeader::WWWAuthenticate,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct RequestHeaders {
    accept: Option<String>,
    accept_charset: Option<String>,
    accept_encoding: Option<String>,
    accept_language: Option<String>,
    authorization: Option<String>,
    expect: Option<String>,
    from: Option<String>,
    host: Option<Host>,
    if_match: Option<String>,
    if_modified_since: Option<String>,
    if_none_match: Option<String>,
    if_range: Option<String>,
    if_unmodified_since: Option<String>,
    max_forwards: Option<String>,
    proxy_authorization: Option<String>,
    range: Option<String>,
    referer: Option<String>,
    te: Option<String>,
    user_agent: Option<String>,
}

#[derive(Debug)]
pub struct GeneralHeaders {
    cache_control: Option<String>,
    connection: Option<String>,
    date: Option<String>,
    pragma: Option<String>,
    trailer: Option<String>,
    transfer_encoding: Option<String>,
    upgrade: Option<String>,
    via: Option<String>,
    warning: Option<String>,
}

#[derive(Debug)]
pub struct EntityHeaders {
    allow: Option<String>,
    content_encoding: Option<String>,
    content_languages: Option<String>,
    content_length: Option<String>,
    content_location: Option<String>,
    content_md5: Option<String>,
    content_range: Option<String>,
    content_type: Option<String>,
    expires: Option<String>,
    last_modified: Option<String>,
    extensions: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ResponseHeaders {
    accept_ranges: Option<String>,
    age: Option<String>,
    etag: Option<String>,
    location: Option<String>,
    proxy_authenticate: Option<String>,
    retry_after: Option<String>,
    server: Option<String>,
    vary: Option<String>,
    www_authenticate: Option<String>,
}

impl RequestHeaders {
    pub fn new() -> Self {
        Self {
            accept: None,
            accept_charset: None,
            accept_encoding: None,
            accept_language: None,
            authorization: None,
            expect: None,
            from: None,
            host: None,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_range: None,
            if_unmodified_since: None,
            max_forwards: None,
            proxy_authorization: None,
            range: None,
            referer: None,
            te: None,
            user_agent: None,
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Result<(), String> {
        if let Some(header) = RequestHeader::from(key) {
            match header {
                RequestHeader::Accept => {
                    self.accept = Some(value.to_string());
                }
                RequestHeader::AcceptCharset => {
                    self.accept_charset = Some(value.to_string());
                }
                RequestHeader::AcceptEncoding => {
                    self.accept_encoding = Some(value.to_string());
                }
                RequestHeader::AcceptLanguage => {
                    self.accept_language = Some(value.to_string());
                }
                RequestHeader::Authorization => {
                    self.authorization = Some(value.to_string());
                }
                RequestHeader::Expect => {
                    self.expect = Some(value.to_string());
                }
                RequestHeader::From => {
                    self.from = Some(value.to_string());
                }
                RequestHeader::Host => {
                    self.host = Some(parse_host_from_wire(value)?);
                }
                RequestHeader::IfMatch => {
                    self.if_match = Some(value.to_string());
                }
                RequestHeader::IfModifiedSince => {
                    self.if_modified_since = Some(value.to_string());
                }
                RequestHeader::IfNoneMatch => {
                    self.if_none_match = Some(value.to_string());
                }
                RequestHeader::IfRange => {
                    self.if_range = Some(value.to_string());
                }
                RequestHeader::IfUnmodifiedSince => {
                    self.if_unmodified_since = Some(value.to_string());
                }
                RequestHeader::MaxForwards => {
                    self.max_forwards = Some(value.to_string());
                }
                RequestHeader::ProxyAuthorization => {
                    self.proxy_authorization = Some(value.to_string());
                }
                RequestHeader::Range => {
                    self.range = Some(value.to_string());
                }
                RequestHeader::Referer => {
                    self.referer = Some(value.to_string());
                }
                RequestHeader::TE => {
                    self.te = Some(value.to_string());
                }
                RequestHeader::UserAgent => {
                    self.user_agent = Some(value.to_string());
                }
            }
        } else {
            self.others.insert(key.to_string(), value.to_string());
        }

        Ok(())
    }

    pub fn string_value(&self, header: &RequestHeaderString) -> Option<String> {
        match header {
            RequestHeaderString::Host => self.host.clone().map(|h| h.to_string()),
            RequestHeaderString::UserAgent => self.user_agent.clone(),
            RequestHeaderString::Accept => self.accept.clone(),
            RequestHeaderString::ContentType => self.content_type.clone(),
            RequestHeaderString::ContentLength => {
                self.content_length.clone().map(|h| h.to_string())
            }
        }
    }

    pub fn insert_from_full_string(&mut self, full_str: &str) -> Result<(), String> {
        let parts: Vec<&str> = full_str.splitn(2, ":").collect();
        if parts.len() != 2 {
            return Err("Invalid header, expected a colon in header
to deliminate key: value"
                .to_string());
        }
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();

        self.insert(&key, &value)?;

        Ok(())
    }
}

impl fmt::Display for RequestHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for header in RequestHeaderString::iter() {
            if let Some(val) = self.string_value(&header) {
                write!(f, "\n{}: {}", header.to_string(), val)?;
            }
        }
        for (key, value) in self.others.iter() {
            write!(f, "\n{}: {}", key, value)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub request_line: RequestLine,
    pub headers: RequestHeaders,
    pub body: Option<String>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nRequestHeaders:{}", self.request_line, self.headers)?;
        if let Some(body) = &self.body {
            write!(f, "\nBody: {}", body)?;
        }
        Ok(())
    }
}

fn parse_request_line_from_reader<R: Read>(
    reader: &mut BufReader<R>,
) -> Result<RequestLine, String> {
    let mut request_line = String::new();

    let _ = reader
        .read_line(&mut request_line)
        .map_err(|e| e.to_string())?;

    request_line = request_line.trim().to_string();

    parse_request_line(&request_line)
}

fn parse_headers_from_reader<R: Read>(reader: &mut BufReader<R>) -> Result<RequestHeaders, String> {
    // Parse the headers
    let mut headers = RequestHeaders::new();
    loop {
        let mut header = String::new();
        let _ = reader.read_line(&mut header).map_err(|e| e.to_string())?;
        header = header.trim().to_string();

        if header.is_empty() {
            break;
        }

        dbg!(&header);
        headers.insert_from_full_string(&header)?;
    }

    Ok(headers)
}

fn parse_body_from_reader<R: Read>(
    content_length: usize,
    reader: &mut BufReader<R>,
) -> Result<String, String> {
    let mut body = Vec::new();
    let _ = reader
        .by_ref()
        .take(content_length as u64)
        .read_to_end(&mut body)
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&body).to_string())
}

pub fn parse_http_request<R: Read>(reader: &mut BufReader<R>) -> Result<HttpRequest, String> {
    let request_line = parse_request_line_from_reader(reader)?;

    let headers = parse_headers_from_reader(reader)?;

    let body = if let Some(content_length) = headers.content_length {
        Some(parse_body_from_reader(content_length, reader)?)
    } else {
        None
    };

    return Ok(HttpRequest {
        request_line,
        headers,
        body,
    });
}

fn parse_request_line(value: &str) -> Result<RequestLine, String> {
    let values: Vec<_> = value.split(" ").collect();

    if values.len() != 3 {
        return Err(
            "Expecting 3 values in request line: Method SP URI SP HTTP/Version".to_string(),
        );
    }

    let method = parse_method_from_wire(values[0].to_string())?;
    let uri = values[1].to_string();
    let (v_major, v_minor) = parse_version_numbers(&values[2].to_string())?;

    Ok(RequestLine {
        method,
        uri,
        v_major,
        v_minor,
    })
}

fn parse_version_numbers(content: &String) -> Result<(u32, u32), String> {
    let parts: Vec<_> = content.split("/").collect();

    if parts.len() != 2 {
        return Err("Expecting 2 values in version line: HTTP/Version".to_string());
    }

    if parts[0] != "HTTP" {
        return Err(format!("Unsupported version string: {}", parts[0]));
    }

    let version_parts: Vec<_> = parts[1].split(".").collect();
    if version_parts.len() != 2 {
        return Err(format!(
            "Expecting version format: <u32>.<u32>. Instead, got: {}",
            parts[1]
        ));
    }

    let v_major = version_parts[0].parse::<u32>().map_err(|e| e.to_string())?;
    let v_minor = version_parts[1].parse::<u32>().map_err(|e| e.to_string())?;

    Ok((v_major, v_minor))
}

fn parse_host_from_wire(content: &str) -> Result<Host, String> {
    let parts: Vec<&str> = content.split(":").collect();

    // No reason you'd see more than one ":" in a hostname
    if parts.len() > 2 {
        return Err("Expected hostname:port or hostname".to_string());
    }

    let hostname = parts[0].to_string();
    let mut port = 80;

    if parts.len() == 2 {
        if let Ok(p) = parts[1].parse::<u32>() {
            port = p;
        } else {
            return Err(format!("Failed to parse port: {}", parts[1]));
        }
    }

    Ok(Host { hostname, port })
}

fn parse_method_from_wire(content: String) -> Result<Method, String> {
    Ok(match content.as_str() {
        "OPTION" => Method::OPTION,
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "TRACE" => Method::TRACE,
        "CONNECT" => Method::CONNECT,
        _ => {
            if is_valid_extension_method(&content) {
                Method::EXTENSION(content.clone())
            } else {
                return Err(format!("Invalid Extension Method: {}", content));
            }
        }
    })
}

fn is_valid_extension_method(content: &String) -> bool {
    content.chars().all(|c| c.is_ascii_alphabetic())
}

//////////////////////////////////////// Display and Debug
