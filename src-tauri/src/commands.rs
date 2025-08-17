use crate::models::{BrowserProfile, BrowserMode, CollectionConfig, CollectionData, CollectionRecord, SaveCollectionRequest, SiteEntry};
use crate::services::{BrowserService, CollectionService, ProfileService};
use chrono::Utc;
use tracing::{info, instrument};

#[tauri::command]
#[instrument]
pub fn greet(name: &str) -> String {
    info!("Greeting user: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[instrument(skip(request), fields(site_count = request.sites.len()))]
pub fn save_collection(request: SaveCollectionRequest) -> Result<CollectionData, String> {
    info!("Starting collection save operation");

    let collection_data = CollectionData {
        sites: request.sites,
        created_at: Utc::now(),
        name: None,                                 // Could be set later
        config: request.config.unwrap_or_default(), // Use provided config or default
    };

    match CollectionService::new() {
        Ok(service) => match service.save_collection(collection_data.clone()) {
            Ok(_) => {
                info!("Collection saved successfully");
                Ok(collection_data)
            }
            Err(e) => {
                tracing::error!("Failed to save collection: {}", e);
                Err(format!("Failed to save collection: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize collection service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn load_collections() -> Result<Vec<CollectionRecord>, String> {
    info!("Loading all collections");

    match CollectionService::new() {
        Ok(service) => match service.load_all_collections() {
            Ok(records) => {
                let collections: Vec<CollectionRecord> = records
                    .into_iter()
                    .map(|record| CollectionRecord {
                        id: record.id,
                        sites: record.sites,
                        created_at: record.created_at,
                        updated_at: record.updated_at,
                        name: record.name,
                        config: record.config,
                    })
                    .collect();
                Ok(collections)
            }
            Err(e) => {
                tracing::error!("Failed to load collections: {}", e);
                Err(format!("Failed to load collections: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize collection service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument(skip(sites), fields(url_count = sites.len()))]
pub fn restore_collection(sites: Vec<SiteEntry>, config: Option<CollectionConfig>) -> Result<(), String> {
    info!("Restoring collection with {} URLs", sites.len());

    let collection_config = config.unwrap_or_default();
    match BrowserService::restore_sites_with_config(sites, &collection_config) {
        Ok(_) => {
            info!("Successfully restored all sites");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to restore sites: {}", e);
            Err(format!("Failed to restore sites: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn get_collection(id: u64) -> Result<Option<CollectionRecord>, String> {
    info!("Getting collection with ID: {}", id);

    match CollectionService::new() {
        Ok(service) => match service.get_collection(id) {
            Ok(collection) => {
                if collection.is_some() {
                    info!("Collection found with ID: {}", id);
                } else {
                    info!("Collection not found with ID: {}", id);
                }
                Ok(collection)
            }
            Err(e) => {
                tracing::error!("Failed to get collection: {}", e);
                Err(format!("Failed to get collection: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize collection service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument(skip(collection_data))]
pub fn update_collection(id: u64, collection_data: CollectionData) -> Result<CollectionRecord, String> {
    info!("Updating collection with ID: {}", id);

    match CollectionService::new() {
        Ok(service) => match service.update_collection(id, collection_data) {
            Ok(updated_record) => {
                info!("Collection updated successfully with ID: {}", id);
                Ok(updated_record)
            }
            Err(e) => {
                tracing::error!("Failed to update collection: {}", e);
                Err(format!("Failed to update collection: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize collection service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn delete_collection(id: u64) -> Result<bool, String> {
    info!("Deleting collection with ID: {}", id);

    match CollectionService::new() {
        Ok(service) => match service.delete_collection(id) {
            Ok(deleted) => {
                if deleted {
                    info!("Collection deleted successfully with ID: {}", id);
                } else {
                    info!("Collection not found for deletion with ID: {}", id);
                }
                Ok(deleted)
            }
            Err(e) => {
                tracing::error!("Failed to delete collection: {}", e);
                Err(format!("Failed to delete collection: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize collection service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

// Browser Profile Management Commands

#[tauri::command]
#[instrument(skip(profile))]
pub fn create_browser_profile(profile: BrowserProfile) -> Result<BrowserProfile, String> {
    info!("Creating browser profile: {}", profile.id);

    match ProfileService::new() {
        Ok(service) => match service.create_profile(profile) {
            Ok(created_profile) => {
                info!("Browser profile created successfully: {}", created_profile.id);
                Ok(created_profile)
            }
            Err(e) => {
                tracing::error!("Failed to create browser profile: {}", e);
                Err(format!("Failed to create browser profile: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn get_browser_profiles() -> Result<Vec<BrowserProfile>, String> {
    info!("Loading all browser profiles");

    match ProfileService::new() {
        Ok(service) => match service.get_all_profiles() {
            Ok(profiles) => {
                info!("Loaded {} browser profiles", profiles.len());
                Ok(profiles)
            }
            Err(e) => {
                tracing::error!("Failed to load browser profiles: {}", e);
                Err(format!("Failed to load browser profiles: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn get_browser_profile(id: String) -> Result<Option<BrowserProfile>, String> {
    info!("Getting browser profile with ID: {}", id);

    match ProfileService::new() {
        Ok(service) => match service.get_profile(&id) {
            Ok(profile) => {
                if profile.is_some() {
                    info!("Browser profile found with ID: {}", id);
                } else {
                    info!("Browser profile not found with ID: {}", id);
                }
                Ok(profile)
            }
            Err(e) => {
                tracing::error!("Failed to get browser profile: {}", e);
                Err(format!("Failed to get browser profile: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument(skip(profile))]
pub fn update_browser_profile(id: String, profile: BrowserProfile) -> Result<BrowserProfile, String> {
    info!("Updating browser profile with ID: {}", id);

    match ProfileService::new() {
        Ok(service) => match service.update_profile(&id, profile) {
            Ok(updated_profile) => {
                info!("Browser profile updated successfully: {}", id);
                Ok(updated_profile)
            }
            Err(e) => {
                tracing::error!("Failed to update browser profile: {}", e);
                Err(format!("Failed to update browser profile: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn delete_browser_profile(id: String) -> Result<bool, String> {
    info!("Deleting browser profile with ID: {}", id);

    match ProfileService::new() {
        Ok(service) => match service.delete_profile(&id) {
            Ok(deleted) => {
                if deleted {
                    info!("Browser profile deleted successfully: {}", id);
                } else {
                    info!("Browser profile not found for deletion: {}", id);
                }
                Ok(deleted)
            }
            Err(e) => {
                tracing::error!("Failed to delete browser profile: {}", e);
                Err(format!("Failed to delete browser profile: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

// Browser Detection Commands

#[tauri::command]
#[instrument]
pub fn check_browser_detection() -> Result<Vec<BrowserProfile>, String> {
    info!("Checking browser detection status for all profiles");

    match ProfileService::new() {
        Ok(service) => match service.update_all_detection_status() {
            Ok(profiles) => {
                info!("Browser detection check completed for {} profiles", profiles.len());
                Ok(profiles)
            }
            Err(e) => {
                tracing::error!("Failed to check browser detection: {}", e);
                Err(format!("Failed to check browser detection: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

// Default Browser Mode Management

#[tauri::command]
#[instrument]
pub fn get_default_browser_mode() -> Result<BrowserMode, String> {
    info!("Getting default browser mode");

    match ProfileService::new() {
        Ok(service) => match service.get_default_browser_mode() {
            Ok(mode) => {
                info!("Default browser mode: {:?}", mode);
                Ok(mode)
            }
            Err(e) => {
                tracing::error!("Failed to get default browser mode: {}", e);
                Err(format!("Failed to get default browser mode: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}

#[tauri::command]
#[instrument]
pub fn set_default_browser_mode(mode: BrowserMode) -> Result<(), String> {
    info!("Setting default browser mode to: {:?}", mode);

    match ProfileService::new() {
        Ok(service) => match service.set_default_browser_mode(mode) {
            Ok(_) => {
                info!("Default browser mode updated successfully");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to set default browser mode: {}", e);
                Err(format!("Failed to set default browser mode: {}", e))
            }
        },
        Err(e) => {
            tracing::error!("Failed to initialize profile service: {}", e);
            Err(format!("Failed to initialize service: {}", e))
        }
    }
}
