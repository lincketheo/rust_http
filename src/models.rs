use std::fmt;

pub struct Host {
    pub hostname: String,
    pub port: u32,
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

impl Host {
    /// Parses a host from a host string
    ///
    /// * `host`: A host string. Examples include:
    /// localhost:1234
    /// localhost
    /// 127.0.0.1:1234
    /// 9.0.1.2:1234
    ///
    /// TODO - check for valid hostname
    pub fn from(host: String) -> Result<Host, String> {
        let parts: Vec<&str> = host.split(":").collect();

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
}
