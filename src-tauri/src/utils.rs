use std::path::PathBuf;
use tracing::{debug, instrument};

const APP_DIR_NAME: &str = ".restore-sites";

/// Get the application data directory ($HOME/.restore-sites)
#[instrument]
pub fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not determine home directory")?;
    
    let data_dir = home_dir.join(APP_DIR_NAME);
    debug!("Data directory: {}", data_dir.display());
    
    Ok(data_dir)
}

/// Validate if a string is a valid URL
#[instrument]
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Clean and validate a URL
#[instrument]
pub fn clean_url(url: &str) -> Option<String> {
    let trimmed = url.trim();
    
    if trimmed.is_empty() {
        return None;
    }
    
    // Add https:// if no protocol is specified
    if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
        if trimmed.starts_with("www.") || trimmed.contains('.') {
            return Some(format!("https://{}", trimmed));
        }
    }
    
    if is_valid_url(trimmed) {
        Some(trimmed.to_string())
    } else {
        None
    }
}

/// Extract domain from URL for display purposes
#[instrument]
pub fn extract_domain(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return host.to_string();
        }
    }
    
    // Fallback: try to extract domain manually
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.")
        .split('/')
        .next()
        .unwrap_or(url)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(!is_valid_url("example.com"));
        assert!(!is_valid_url("ftp://example.com"));
    }

    #[test]
    fn test_clean_url() {
        assert_eq!(clean_url("https://example.com"), Some("https://example.com".to_string()));
        assert_eq!(clean_url("www.example.com"), Some("https://www.example.com".to_string()));
        assert_eq!(clean_url("example.com"), Some("https://example.com".to_string()));
        assert_eq!(clean_url(""), None);
        assert_eq!(clean_url("not-a-url"), None);
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://www.example.com/path"), "www.example.com");
        assert_eq!(extract_domain("http://example.com"), "example.com");
        assert_eq!(extract_domain("https://subdomain.example.com/path?query=1"), "subdomain.example.com");
    }
}