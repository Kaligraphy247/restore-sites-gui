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
    pub browser: Browser,
    pub mode: BrowserMode,
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

// JSON Database Schema Structures
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseMeta {
    pub last_updated_id: u64,
    pub last_updated: DateTime<Utc>,
    pub max_id: u64,
    pub record_count: usize,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub meta: DatabaseMeta,
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
            last_updated_id: 0,
            last_updated: now,
            max_id: 0,
            record_count: 0,
            created_at: now,
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            meta: DatabaseMeta::default(),
            data: Vec::new(),
        }
    }
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            browser: Browser::Chrome,
            mode: BrowserMode::Incognito,
            custom_path: None,
        }
    }
}