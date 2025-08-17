#![allow(dead_code)]
use crate::models::{BrowserProfile, BrowserMode, CollectionRecord, Database};
use crate::utils::get_data_dir;
use chrono::Utc;
use std::fs;
use tracing::{debug, info, instrument, warn};

const DB_FILE_NAME: &str = "db.json";

#[derive(Debug)]
pub struct JsonStore {
    file_path: std::path::PathBuf,
}

impl JsonStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = get_data_dir()?;
        let file_path = data_dir.join(DB_FILE_NAME);

        // Create directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self { file_path })
    }

    #[instrument]
    pub fn load(&self) -> Result<Database, Box<dyn std::error::Error>> {
        if !self.file_path.exists() {
            info!("Database file not found, creating new database");
            return Ok(Database::default());
        }

        let content = fs::read_to_string(&self.file_path)?;
        let database: Database = serde_json::from_str(&content)?;

        debug!("Loaded database with {} records", database.data.len());
        Ok(database)
    }

    #[instrument(skip(database))]
    pub fn save(&self, database: &Database) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(database)?;
        fs::write(&self.file_path, content)?;

        info!("Saved database with {} records", database.data.len());
        Ok(())
    }

    #[instrument(skip(self, record))]
    pub fn insert(
        &self,
        mut record: CollectionRecord,
    ) -> Result<CollectionRecord, Box<dyn std::error::Error>> {
        let mut database = self.load()?;

        // Auto-assign ID
        database.meta.max_id += 1;
        record.id = database.meta.max_id;
        record.updated_at = Utc::now();

        database.data.push(record.clone());
        database.meta.record_count = database.data.len();
        database.meta.last_updated = Utc::now();
        database.meta.last_updated_id = record.id;

        self.save(&database)?;
        info!("Inserted new record with ID: {}", record.id);

        Ok(record)
    }

    #[instrument]
    pub fn get_all(&self) -> Result<Vec<CollectionRecord>, Box<dyn std::error::Error>> {
        let database = self.load()?;
        Ok(database.data)
    }

    #[instrument]
    pub fn get_by_id(
        &self,
        id: u64,
    ) -> Result<Option<CollectionRecord>, Box<dyn std::error::Error>> {
        let database = self.load()?;
        let record = database.data.into_iter().find(|r| r.id == id);
        Ok(record)
    }

    #[instrument]
    pub fn search_by_name(
        &self,
        query: &str,
    ) -> Result<Vec<CollectionRecord>, Box<dyn std::error::Error>> {
        let database = self.load()?;
        let results: Vec<CollectionRecord> = database
            .data
            .into_iter()
            .filter(|record| record.name.to_lowercase().contains(&query.to_lowercase()))
            .collect();

        debug!("Search for '{}' returned {} results", query, results.len());
        Ok(results)
    }

    #[instrument(skip(self, updated_record))]
    pub fn update(
        &self,
        updated_record: CollectionRecord,
    ) -> Result<CollectionRecord, Box<dyn std::error::Error>> {
        let mut database = self.load()?;
        
        // Find and replace the record with the given ID
        let mut found = false;
        for record in &mut database.data {
            if record.id == updated_record.id {
                *record = updated_record.clone();
                record.updated_at = Utc::now(); // Ensure updated timestamp
                found = true;
                break;
            }
        }

        if !found {
            return Err(format!("Record with ID {} not found for update", updated_record.id).into());
        }

        database.meta.last_updated = Utc::now();
        database.meta.last_updated_id = updated_record.id;
        self.save(&database)?;
        
        info!("Updated record with ID: {}", updated_record.id);
        Ok(updated_record)
    }

    #[instrument]
    pub fn delete_by_id(&self, id: u64) -> Result<bool, Box<dyn std::error::Error>> {
        let mut database = self.load()?;
        let initial_len = database.data.len();

        database.data.retain(|record| record.id != id);

        if database.data.len() < initial_len {
            database.meta.record_count = database.data.len();
            database.meta.last_updated = Utc::now();
            self.save(&database)?;
            info!("Deleted record with ID: {}", id);
            Ok(true)
        } else {
            warn!("Record with ID {} not found for deletion", id);
            Ok(false)
        }
    }

    // Browser Profile Management
    
    #[instrument(skip(self, profile))]
    pub fn create_profile(
        &self,
        profile: BrowserProfile,
    ) -> Result<BrowserProfile, Box<dyn std::error::Error>> {
        let mut database = self.load()?;
        
        // Check for duplicate profile ID
        if database.profiles.iter().any(|p| p.id == profile.id) {
            return Err(format!("Profile with ID '{}' already exists", profile.id).into());
        }
        
        database.profiles.push(profile.clone());
        database.meta.last_updated = Utc::now();
        
        self.save(&database)?;
        info!("Created new browser profile: {}", profile.id);
        
        Ok(profile)
    }
    
    #[instrument]
    pub fn get_profile(
        &self,
        id: &str,
    ) -> Result<Option<BrowserProfile>, Box<dyn std::error::Error>> {
        let database = self.load()?;
        let profile = database.profiles.into_iter().find(|p| p.id == id);
        Ok(profile)
    }
    
    #[instrument]
    pub fn get_all_profiles(&self) -> Result<Vec<BrowserProfile>, Box<dyn std::error::Error>> {
        let database = self.load()?;
        Ok(database.profiles)
    }
    
    #[instrument(skip(self, updated_profile))]
    pub fn update_profile(
        &self,
        id: &str,
        updated_profile: BrowserProfile,
    ) -> Result<BrowserProfile, Box<dyn std::error::Error>> {
        let mut database = self.load()?;
        
        // Find and replace the profile with the given ID
        let mut found = false;
        for profile in &mut database.profiles {
            if profile.id == id {
                *profile = updated_profile.clone();
                profile.updated_at = Utc::now(); // Ensure updated timestamp
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(format!("Profile with ID '{}' not found for update", id).into());
        }
        
        database.meta.last_updated = Utc::now();
        self.save(&database)?;
        
        info!("Updated browser profile: {}", id);
        Ok(updated_profile)
    }
    
    #[instrument]
    pub fn delete_profile(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut database = self.load()?;
        let initial_len = database.profiles.len();
        
        database.profiles.retain(|profile| profile.id != id);
        
        if database.profiles.len() < initial_len {
            database.meta.last_updated = Utc::now();
            self.save(&database)?;
            info!("Deleted browser profile: {}", id);
            Ok(true)
        } else {
            warn!("Profile with ID '{}' not found for deletion", id);
            Ok(false)
        }
    }
    
    // Default Browser Mode Management
    
    #[instrument]
    pub fn get_default_browser_mode(&self) -> Result<BrowserMode, Box<dyn std::error::Error>> {
        let database = self.load()?;
        Ok(database.meta.default_browser_mode)
    }
    
    #[instrument]
    pub fn set_default_browser_mode(
        &self,
        mode: BrowserMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut database = self.load()?;
        database.meta.default_browser_mode = mode.clone();
        database.meta.last_updated = Utc::now();
        
        self.save(&database)?;
        info!("Updated default browser mode to: {:?}", mode);
        
        Ok(())
    }
}
