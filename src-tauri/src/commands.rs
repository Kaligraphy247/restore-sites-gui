use crate::models::{CollectionData, CollectionRecord, SaveCollectionRequest, SiteEntry};
use crate::services::{BrowserService, CollectionService};
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
pub fn restore_collection(sites: Vec<SiteEntry>) -> Result<(), String> {
    info!("Restoring collection with {} URLs", sites.len());

    match BrowserService::restore_sites(sites) {
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
