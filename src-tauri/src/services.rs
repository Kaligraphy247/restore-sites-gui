#![allow(dead_code)]
use crate::database::JsonStore;
use crate::models::{
    Browser, BrowserMode, CollectionConfig, CollectionData, CollectionRecord, SiteEntry,
};
use chrono::Utc;
use tracing::{info, instrument, warn};

pub struct CollectionService {
    db: JsonStore,
}

impl CollectionService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = JsonStore::new()?;
        Ok(Self { db })
    }

    #[instrument(skip(self, collection_data))]
    pub fn save_collection(
        &self,
        collection_data: CollectionData,
    ) -> Result<CollectionRecord, Box<dyn std::error::Error>> {
        let record = CollectionRecord {
            id: 0, // Will be auto-assigned by database
            name: collection_data
                .name
                .unwrap_or_else(|| format!("Collection {}", Utc::now().timestamp())),
            sites: collection_data.sites,
            config: collection_data.config,
            created_at: collection_data.created_at,
            updated_at: Utc::now(),
        };

        let saved_record = self.db.insert(record)?;
        info!("Collection saved with ID: {}", saved_record.id);
        Ok(saved_record)
    }

    #[instrument(skip(self))]
    pub fn load_all_collections(
        &self,
    ) -> Result<Vec<CollectionRecord>, Box<dyn std::error::Error>> {
        self.db.get_all()
    }

    #[instrument(skip(self))]
    pub fn get_collection(
        &self,
        id: u64,
    ) -> Result<Option<CollectionRecord>, Box<dyn std::error::Error>> {
        self.db.get_by_id(id)
    }

    #[instrument(skip(self))]
    pub fn search_collections(
        &self,
        query: &str,
    ) -> Result<Vec<CollectionRecord>, Box<dyn std::error::Error>> {
        self.db.search_by_name(query)
    }

    #[instrument(skip(self, collection_data))]
    pub fn update_collection(
        &self,
        id: u64,
        collection_data: CollectionData,
    ) -> Result<CollectionRecord, Box<dyn std::error::Error>> {
        // Get existing record first
        let existing = self.get_collection(id)?
            .ok_or_else(|| format!("Collection with id {} not found", id))?;

        let updated_record = CollectionRecord {
            id,
            name: collection_data.name.unwrap_or(existing.name),
            sites: collection_data.sites,
            config: collection_data.config,
            created_at: existing.created_at, // Preserve original creation time
            updated_at: Utc::now(),
        };

        self.db.update(updated_record)
    }

    #[instrument(skip(self))]
    pub fn delete_collection(&self, id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        self.db.delete_by_id(id)
    }
}

// Helper struct to hold resolved browser configuration
#[derive(Debug, Clone)]
struct ResolvedBrowserConfig {
    browser: Browser,
    mode: BrowserMode,
    custom_path: Option<String>,
}

pub struct BrowserService;

impl BrowserService {
    // Resolve browser configuration from collection config with fallbacks
    fn resolve_browser_config(config: &CollectionConfig) -> ResolvedBrowserConfig {
        // For now, use simple fallbacks until profiles system is fully implemented
        // TODO: Add profile resolution logic here in Phase 1.3
        
        let browser = config.browser.clone().unwrap_or(Browser::Chrome);
        let mode = config.mode.clone().unwrap_or(BrowserMode::Incognito);
        let custom_path = config.custom_path.clone();
        
        ResolvedBrowserConfig {
            browser,
            mode, 
            custom_path,
        }
    }

    #[instrument(skip(sites))]
    pub fn restore_sites_with_config(
        sites: Vec<SiteEntry>,
        config: &CollectionConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let resolved_config = Self::resolve_browser_config(config);
        
        info!(
            "Starting browser restoration for {} sites with {:?} in {:?} mode",
            sites.len(),
            resolved_config.browser,
            resolved_config.mode
        );

        for (index, site) in sites.iter().enumerate() {
            if let Err(e) = Self::open_url_with_resolved_config(&site.url, &resolved_config) {
                warn!("Failed to open URL {}: {}", site.url, e);
            } else {
                info!("Opened URL {}: {}", index + 1, site.url);
            }

            // Small delay to prevent overwhelming the system (same as Python script)
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        info!("Browser restoration completed");
        Ok(())
    }

    #[instrument(skip(config))]
    fn open_url_with_resolved_config(
        url: &str,
        config: &ResolvedBrowserConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let quoted_url = shlex::try_quote(url).unwrap_or_else(|_| url.into());

        #[cfg(target_os = "macos")]
        {
            // Mimic the Python script: open -na "Google Chrome" --args --incognito {url}
            let mut cmd = std::process::Command::new("open");
            cmd.arg("-na");

            match &config.browser {
                Browser::Chrome => cmd.arg("Google Chrome"),
                Browser::Firefox => cmd.arg("Firefox"),
                Browser::Safari => cmd.arg("Safari"),
                Browser::Edge => cmd.arg("Microsoft Edge"),
                Browser::Custom(name) => cmd.arg(name),
            };

            cmd.arg("--args");

            match config.mode {
                BrowserMode::Incognito => {
                    if matches!(config.browser, Browser::Chrome) {
                        cmd.arg("--incognito");
                    } else if matches!(config.browser, Browser::Firefox) {
                        cmd.arg("--private-window");
                    }
                }
                BrowserMode::Private => {
                    if matches!(config.browser, Browser::Safari) {
                        cmd.arg("--private");
                    } else if matches!(config.browser, Browser::Firefox) {
                        cmd.arg("--private-window");
                    }
                }
                BrowserMode::Normal => {}
            }

            cmd.arg(&quoted_url.to_string());
            cmd.spawn()?;
        }

        #[cfg(target_os = "windows")]
        {
            // Mimic the Python script for Windows
            let browser_exe = match &config.browser {
                Browser::Chrome => "chrome.exe",
                Browser::Firefox => "firefox.exe",
                Browser::Edge => "msedge.exe",
                Browser::Custom(path) => path,
                _ => "chrome.exe", // Default to Chrome
            };

            let mut cmd = std::process::Command::new(browser_exe);

            match config.mode {
                BrowserMode::Incognito => {
                    if matches!(config.browser, Browser::Chrome | Browser::Edge) {
                        cmd.arg("--incognito");
                    } else if matches!(config.browser, Browser::Firefox) {
                        cmd.arg("--private-window");
                    }
                }
                BrowserMode::Private => {
                    if matches!(config.browser, Browser::Firefox) {
                        cmd.arg("--private-window");
                    }
                }
                BrowserMode::Normal => {}
            }

            cmd.arg(&quoted_url.to_string());
            cmd.spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            // Mimic the Python script for Linux (using Chrome path)
            let browser_path = match &config.browser {
                Browser::Chrome => "/opt/google/chrome/chrome",
                Browser::Firefox => "firefox",
                Browser::Custom(path) => path,
                _ => "/opt/google/chrome/chrome",
            };

            let mut cmd = std::process::Command::new(browser_path);

            match config.mode {
                BrowserMode::Incognito => {
                    if matches!(config.browser, Browser::Chrome) {
                        cmd.arg("--incognito");
                    } else if matches!(config.browser, Browser::Firefox) {
                        cmd.arg("--private-window");
                    }
                }
                BrowserMode::Private => {
                    if matches!(config.browser, Browser::Firefox) {
                        cmd.arg("--private-window");
                    }
                }
                BrowserMode::Normal => {}
            }

            cmd.arg(&quoted_url.to_string());
            cmd.spawn()?;
        }

        Ok(())
    }

    // Backward compatibility - defaults to Chrome incognito
    #[instrument(skip(sites))]
    pub fn restore_sites(sites: Vec<SiteEntry>) -> Result<(), Box<dyn std::error::Error>> {
        let default_config = CollectionConfig::default();
        Self::restore_sites_with_config(sites, &default_config)
    }
}
