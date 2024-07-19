use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

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

    pub fn insert(&mut self, key: RequestHeader, value: &str) -> Result<(), String> {
        match key {
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
        Ok(())
    }
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

impl GeneralHeaders {
    pub fn new() -> Self {
        Self {
            cache_control: None,
            connection: None,
            date: None,
            pragma: None,
            trailer: None,
            transfer_encoding: None,
            upgrade: None,
            via: None,
            warning: None,
        }
    }

    pub fn insert(&mut self, key: GeneralHeader, value: &str) -> Result<(), String> {
        match key {
            GeneralHeader::CacheControl => {
                self.cache_control = Some(value.to_string());
            }
            GeneralHeader::Connection => {
                self.connection = Some(value.to_string());
            }
            GeneralHeader::Date => {
                self.date = Some(value.to_string());
            }
            GeneralHeader::Pragma => {
                self.pragma = Some(value.to_string());
            }
            GeneralHeader::Trailer => {
                self.trailer = Some(value.to_string());
            }
            GeneralHeader::TransferEncoding => {
                self.transfer_encoding = Some(value.to_string());
            }
            GeneralHeader::Upgrade => {
                self.upgrade = Some(value.to_string());
            }
            GeneralHeader::Via => {
                self.via = Some(value.to_string());
            }
            GeneralHeader::Warning => {
                self.warning = Some(value.to_string());
            }
        }
        Ok(())
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

#[derive(Debug)]
pub struct EntityHeaders {
    allow: Option<String>,
    content_encoding: Option<String>,
    content_languages: Option<String>,
    content_length: Option<usize>,
    content_location: Option<String>,
    content_md5: Option<String>,
    content_range: Option<String>,
    content_type: Option<String>,
    expires: Option<String>,
    last_modified: Option<String>,
    extensions: HashMap<String, String>,
}

impl EntityHeaders {
    pub fn new() -> Self {
        Self {
            allow: None,
            content_encoding: None,
            content_languages: None,
            content_length: None,
            content_location: None,
            content_md5: None,
            content_range: None,
            content_type: None,
            expires: None,
            last_modified: None,
            extensions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: EntityHeader, value: &str) -> Result<(), String> {
        match key {
            EntityHeader::Allow => {
                self.allow = Some(value.to_string());
            }
            EntityHeader::ContentEncoding => {
                self.content_encoding = Some(value.to_string());
            }
            EntityHeader::ContentLanguages => {
                self.content_languages = Some(value.to_string());
            }
            EntityHeader::ContentLength => {
                self.content_length = Some(value.parse::<usize>().map_err(|it| it.to_string())?);
            }
            EntityHeader::ContentLocation => {
                self.content_location = Some(value.to_string());
            }
            EntityHeader::ContentMD5 => {
                self.content_md5 = Some(value.to_string());
            }
            EntityHeader::ContentRange => {
                self.content_range = Some(value.to_string());
            }
            EntityHeader::ContentType => {
                self.content_type = Some(value.to_string());
            }
            EntityHeader::Expires => {
                self.expires = Some(value.to_string());
            }
            EntityHeader::LastModified => {
                self.last_modified = Some(value.to_string());
            }
            EntityHeader::Extension(s) => {
                self.extensions.insert(s, value.to_string());
            }
        }
        Ok(())
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

impl ResponseHeaders {
    pub fn new() -> Self {
        Self {
            accept_ranges: None,
            age: None,
            etag: None,
            location: None,
            proxy_authenticate: None,
            retry_after: None,
            server: None,
            vary: None,
            www_authenticate: None,
        }
    }

    pub fn insert(&mut self, key: ResponseHeader, value: &str) -> Result<(), String> {
        match key {
            ResponseHeader::AcceptRanges => {
                self.accept_ranges = Some(value.to_string());
            }
            ResponseHeader::Age => {
                self.age = Some(value.to_string());
            }
            ResponseHeader::ETag => {
                self.etag = Some(value.to_string());
            }
            ResponseHeader::Location => {
                self.location = Some(value.to_string());
            }
            ResponseHeader::ProxyAuthenticate => {
                self.proxy_authenticate = Some(value.to_string());
            }
            ResponseHeader::RetryAfter => {
                self.retry_after = Some(value.to_string());
            }
            ResponseHeader::Server => {
                self.server = Some(value.to_string());
            }
            ResponseHeader::Vary => {
                self.vary = Some(value.to_string());
            }
            ResponseHeader::WWWAuthenticate => {
                self.www_authenticate = Some(value.to_string());
            }
        }
        Ok(())
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
pub struct HttpRequest {
    pub request_line: RequestLine,
    pub request_headers: RequestHeaders,
    pub general_headers: GeneralHeaders,
    pub entity_headers: EntityHeaders,
    pub body: Option<String>,
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

fn parse_headers_from_reader<R: Read>(
    reader: &mut BufReader<R>,
) -> Result<(RequestHeaders, GeneralHeaders, EntityHeaders), String> {
    // Parse the headers
    let mut request_headers = RequestHeaders::new();
    let mut general_headers = GeneralHeaders::new();
    let mut entity_headers = EntityHeaders::new();

    loop {
        let mut header = String::new();
        let _ = reader.read_line(&mut header).map_err(|e| e.to_string())?;
        header = header.trim().to_string();

        if header.is_empty() {
            break;
        }

        let values: Vec<_> = header.splitn(2, ":").collect();
        dbg!(&values);

        if values.len() != 2 {
            return Err("Expecting 'key: value' in header".to_string());
        }
        let key = values[0];
        let value = values[1].trim();

        if let Some(rheader) = RequestHeader::from(key) {
            request_headers.insert(rheader, value)?;
        } else if let Some(gheader) = GeneralHeader::from(key) {
            general_headers.insert(gheader, value)?;
        } else if let Some(eheader) = EntityHeader::from(key) {
            entity_headers.insert(eheader, value)?;
        } else {
            panic!("Entity header extension should catch unkown headers");
        }
    }

    Ok((request_headers, general_headers, entity_headers))
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

    let (request_headers, general_headers, entity_headers) = parse_headers_from_reader(reader)?;

    let body = if let Some(content_length) = entity_headers.content_length {
        Some(parse_body_from_reader(content_length, reader)?)
    } else {
        None
    };

    return Ok(HttpRequest {
        request_line,
        request_headers,
        general_headers,
        entity_headers,
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
