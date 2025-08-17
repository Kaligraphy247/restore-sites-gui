#![allow(dead_code)]
use crate::database::JsonStore;
use crate::models::{
    Browser, BrowserMode, BrowserProfile, CollectionConfig, CollectionData, CollectionRecord,
    SiteEntry,
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
        let existing = self
            .get_collection(id)?
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

pub struct ProfileService {
    db: JsonStore,
}

impl ProfileService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = JsonStore::new()?;
        Ok(Self { db })
    }

    // Browser Detection Utilities

    #[instrument]
    pub fn detect_chrome_installation() -> bool {
        #[cfg(target_os = "macos")]
        {
            std::path::Path::new("/Applications/Google Chrome.app").exists()
        }

        #[cfg(target_os = "windows")]
        {
            // Check common Chrome installation paths
            let paths = [
                r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
            ];
            paths.iter().any(|path| std::path::Path::new(path).exists())
        }

        #[cfg(target_os = "linux")]
        {
            // Check if Chrome is in PATH or common installation paths
            if std::process::Command::new("which")
                .arg("google-chrome")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                return true;
            }

            let paths = [
                "/opt/google/chrome/chrome",
                "/usr/bin/google-chrome",
                "/usr/bin/google-chrome-stable",
            ];
            paths.iter().any(|path| std::path::Path::new(path).exists())
        }
    }

    #[instrument]
    pub fn detect_firefox_installation() -> bool {
        #[cfg(target_os = "macos")]
        {
            std::path::Path::new("/Applications/Firefox.app").exists()
        }

        #[cfg(target_os = "windows")]
        {
            let paths = [
                r"C:\Program Files\Mozilla Firefox\firefox.exe",
                r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe",
            ];
            paths.iter().any(|path| std::path::Path::new(path).exists())
        }

        #[cfg(target_os = "linux")]
        {
            if std::process::Command::new("which")
                .arg("firefox")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                return true;
            }

            let paths = [
                "/usr/bin/firefox",
                "/usr/bin/firefox-esr",
                "/opt/firefox/firefox",
            ];
            paths.iter().any(|path| std::path::Path::new(path).exists())
        }
    }

    #[instrument]
    pub fn detect_safari_installation() -> bool {
        #[cfg(target_os = "macos")]
        {
            std::path::Path::new("/Applications/Safari.app").exists()
        }

        #[cfg(not(target_os = "macos"))]
        {
            false // Safari only available on macOS
        }
    }

    #[instrument]
    pub fn detect_edge_installation() -> bool {
        #[cfg(target_os = "macos")]
        {
            std::path::Path::new("/Applications/Microsoft Edge.app").exists()
        }

        #[cfg(target_os = "windows")]
        {
            let paths = [
                r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
                r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
            ];
            paths.iter().any(|path| std::path::Path::new(path).exists())
        }

        #[cfg(target_os = "linux")]
        {
            if std::process::Command::new("which")
                .arg("microsoft-edge")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
            {
                return true;
            }

            // Check common Edge installation paths on Linux
            let paths = ["/usr/bin/microsoft-edge", "/usr/bin/microsoft-edge-stable"];
            paths.iter().any(|path| std::path::Path::new(path).exists())
        }
    }

    #[instrument]
    pub fn check_custom_path(path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    // Update detection status for all profiles
    #[instrument(skip(self))]
    pub fn update_all_detection_status(
        &self,
    ) -> Result<Vec<BrowserProfile>, Box<dyn std::error::Error>> {
        let mut profiles = self.db.get_all_profiles()?;

        for profile in &mut profiles {
            let is_detected = match &profile.browser {
                Browser::Chrome => Self::detect_chrome_installation(),
                Browser::Firefox => Self::detect_firefox_installation(),
                Browser::Safari => Self::detect_safari_installation(),
                Browser::Edge => Self::detect_edge_installation(),
                Browser::Custom(path) => Self::check_custom_path(path),
            };

            if profile.is_detected != is_detected {
                profile.is_detected = is_detected;
                profile.updated_at = Utc::now();
                // Update the profile in database
                self.db.update_profile(&profile.id, profile.clone())?;
            }
        }

        info!("Updated detection status for {} profiles", profiles.len());
        Ok(profiles)
    }

    // Profile Resolution Logic
    #[instrument(skip(self))]
    pub fn resolve_browser_config(
        &self,
        collection_config: &CollectionConfig,
    ) -> Result<ResolvedBrowserConfig, Box<dyn std::error::Error>> {
        // 1. Collection profile reference
        if let Some(profile_id) = &collection_config.browser_profile_id {
            if let Some(profile) = self.db.get_profile(profile_id)? {
                return Ok(ResolvedBrowserConfig {
                    browser: profile.browser,
                    mode: profile.mode,
                    custom_path: profile.custom_path,
                });
            }
            warn!(
                "Profile '{}' not found, falling back to direct config",
                profile_id
            );
        }

        // 2. Collection direct config
        if collection_config.browser.is_some() {
            let browser = collection_config.browser.clone().unwrap();
            let mode = collection_config
                .mode
                .clone()
                .unwrap_or(BrowserMode::Normal);
            return Ok(ResolvedBrowserConfig {
                browser,
                mode,
                custom_path: collection_config.custom_path.clone(),
            });
        }

        // 3. System default profile
        let profiles = self.db.get_all_profiles()?;
        if let Some(default_profile) = profiles.iter().find(|p| p.is_default) {
            return Ok(ResolvedBrowserConfig {
                browser: default_profile.browser.clone(),
                mode: default_profile.mode.clone(),
                custom_path: default_profile.custom_path.clone(),
            });
        }

        // 4. Final fallback - use global default mode
        let default_mode = self.db.get_default_browser_mode()?;
        Ok(ResolvedBrowserConfig {
            browser: Browser::Chrome,
            mode: default_mode,
            custom_path: None,
        })
    }

    // Public methods for database operations (used by commands)
    
    #[instrument(skip(self, profile))]
    pub fn create_profile(&self, profile: BrowserProfile) -> Result<BrowserProfile, Box<dyn std::error::Error>> {
        self.db.create_profile(profile)
    }
    
    #[instrument(skip(self))]
    pub fn get_all_profiles(&self) -> Result<Vec<BrowserProfile>, Box<dyn std::error::Error>> {
        self.db.get_all_profiles()
    }
    
    #[instrument(skip(self))]
    pub fn get_profile(&self, id: &str) -> Result<Option<BrowserProfile>, Box<dyn std::error::Error>> {
        self.db.get_profile(id)
    }
    
    #[instrument(skip(self, profile))]
    pub fn update_profile(&self, id: &str, profile: BrowserProfile) -> Result<BrowserProfile, Box<dyn std::error::Error>> {
        self.db.update_profile(id, profile)
    }
    
    #[instrument(skip(self))]
    pub fn delete_profile(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        self.db.delete_profile(id)
    }
    
    #[instrument(skip(self))]
    pub fn get_default_browser_mode(&self) -> Result<BrowserMode, Box<dyn std::error::Error>> {
        self.db.get_default_browser_mode()
    }
    
    #[instrument(skip(self))]
    pub fn set_default_browser_mode(&self, mode: BrowserMode) -> Result<(), Box<dyn std::error::Error>> {
        self.db.set_default_browser_mode(mode)
    }
}

// Helper struct to hold resolved browser configuration
#[derive(Debug, Clone)]
pub struct ResolvedBrowserConfig {
    browser: Browser,
    mode: BrowserMode,
    custom_path: Option<String>,
}

pub struct BrowserService;

impl BrowserService {
    // Resolve browser configuration using ProfileService for full profile resolution
    fn resolve_browser_config(
        config: &CollectionConfig,
    ) -> Result<ResolvedBrowserConfig, Box<dyn std::error::Error>> {
        let profile_service = ProfileService::new()?;
        profile_service.resolve_browser_config(config)
    }

    #[instrument(skip(sites))]
    pub fn restore_sites_with_config(
        sites: Vec<SiteEntry>,
        config: &CollectionConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let resolved_config = Self::resolve_browser_config(config)?;

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
