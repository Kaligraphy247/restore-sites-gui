#![allow(dead_code)]
use crate::models::{CollectionRecord, Database};
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
}
