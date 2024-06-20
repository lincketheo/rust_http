use crate::models::{Header, Host, HttpRequest, Method, RequestLine};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

//////////// Public
pub fn parse_http_request<R: Read>(reader: &mut BufReader<R>) -> Result<HttpRequest, String> {
    // Parse the request line
    let mut request_line = String::new();
    let _ = reader
        .read_line(&mut request_line)
        .map_err(|e| e.to_string())?;
    request_line = request_line.trim().to_string();
    let request_line = parse_request_line(request_line)?;

    // Parse the headers
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut content_length = 0;
    loop {
        let mut header = String::new();
        let _ = reader.read_line(&mut header).map_err(|e| e.to_string())?;
        header = header.trim().to_string();
        if header.is_empty() {
            break;
        }
        let header = parse_header_from_wire(header)?;
        if let Header::ContentLength(n) = header {
            content_length = n;
        }
        let (key, value) = header.to_key_value();
        headers.insert(key, value);
    }

    // Parse the body
    let body = if content_length > 0 {
        let mut body = Vec::new();
        let _ = reader
            .by_ref()
            .take(content_length as u64)
            .read_to_end(&mut body)
            .map_err(|e| e.to_string())?;
        Some(String::from_utf8_lossy(&body).to_string())
    } else {
        None
    };

    return Ok(HttpRequest {
        request_line,
        headers,
        body,
    });
}

//////////// Private
fn parse_request_line(value: String) -> Result<RequestLine, String> {
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

fn parse_host_from_wire(content: &String) -> Result<Host, String> {
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

fn parse_header_from_wire(content: String) -> Result<Header, String> {
    let parts: Vec<&str> = content.splitn(2, ":").collect();
    if parts.len() != 2 {
        return Err(
            "Invalid header, expected a colon in header to deliminate key: value".to_string(),
        );
    }
    let key = parts[0].trim().to_string();
    let value = parts[1].trim().to_string();
    parse_header_from_key_value(key, value)
}

fn parse_header_from_key_value(key: String, value: String) -> Result<Header, String> {
    Ok(match key.as_str() {
        "Host" => Header::Host(parse_host_from_wire(&value)?),
        "User-Agent" => Header::UserAgent(value),
        "Accept" => Header::Accept(value),
        "Content-Type" => Header::ContentType(value),
        "Content-Length" => match value.parse::<usize>() {
            Ok(cl) => Header::ContentLength(cl),
            Err(msg) => return Err(msg.to_string()),
        },
        _ => Header::Other(key, value),
    })
}

