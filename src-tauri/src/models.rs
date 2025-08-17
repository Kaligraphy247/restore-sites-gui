use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteEntry {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveCollectionRequest {
    pub sites: Vec<SiteEntry>,
    pub config: Option<CollectionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub sites: Vec<SiteEntry>,
    pub created_at: DateTime<Utc>,
    pub name: Option<String>,
    pub config: CollectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub browser_profile_id: Option<String>,  // PRIMARY: Reference to profile
    // Fallback fields for direct config
    pub browser: Option<Browser>,
    pub mode: Option<BrowserMode>,
    pub custom_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Browser {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserMode {
    Normal,
    Incognito,
    Private,
}

// Browser Profile Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfile {
    pub id: String,                    // "default-chrome", "work-firefox-1"
    pub name: String,                  // Max 64 UTF-8 chars, user-editable
    pub browser: Browser,              // Chrome | Firefox | Safari | Edge | Custom(String)
    pub mode: BrowserMode,             // Normal | Incognito | Private
    pub custom_path: Option<String>,   // Optional custom browser path
    pub is_default: bool,              // One profile marked as system default
    pub is_detected: bool,             // Currently detected on system
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// JSON Database Schema Structures
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseMeta {
    pub version: u32,                  // Schema version (v2 for profiles)
    pub last_updated_id: u64,
    pub last_updated: DateTime<Utc>,
    pub max_id: u64,
    pub record_count: usize,
    pub created_at: DateTime<Utc>,
    pub default_browser_mode: BrowserMode,  // Global default mode
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub meta: DatabaseMeta,
    pub profiles: Vec<BrowserProfile>,  // Browser profiles array
    pub data: Vec<CollectionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRecord {
    pub id: u64,
    pub name: String,
    pub sites: Vec<SiteEntry>,
    pub config: CollectionConfig,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for DatabaseMeta {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            version: 2,  // Start with schema v2
            last_updated_id: 0,
            last_updated: now,
            max_id: 0,
            record_count: 0,
            created_at: now,
            default_browser_mode: BrowserMode::Incognito,  // Default to Incognito
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            meta: DatabaseMeta::default(),
            profiles: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            browser_profile_id: None,  // No profile by default
            browser: None,             // No direct config by default
            mode: None,
            custom_path: None,
        }
    }
}

// Validation functions for Browser Profiles
impl BrowserProfile {
    pub fn validate_name(name: &str) -> Result<(), String> {
        let char_count = name.chars().count(); // UTF-8 safe counting
        if char_count == 0 {
            return Err("Profile name cannot be empty".to_string());
        }
        if char_count > 64 {
            return Err("Profile name cannot exceed 64 characters".to_string());
        }
        Ok(())
    }
    
    pub fn new(id: String, name: String, browser: Browser, mode: BrowserMode) -> Result<Self, String> {
        Self::validate_name(&name)?;
        let now = Utc::now();
        Ok(Self {
            id,
            name,
            browser,
            mode,
            custom_path: None,
            is_default: false,
            is_detected: false,
            created_at: now,
            updated_at: now,
        })
    }
}
