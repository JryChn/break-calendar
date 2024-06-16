use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;

use anyhow::bail;
use anyhow::Result;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tracing::error;

use crate::cache::Cache;
use crate::common::exception::InternalError::DataPersistenceError;
use crate::model::EventCommonTrait;
use crate::persistent::{Persistent, PersistentModel};

mod test;

pub const DEFAULT_FILE_NAME: &str = "metadata";

pub struct FilePersistenceSystem;

impl FilePersistenceSystem {
    pub async fn save(cache: &Cache, file_path: Option<String>) -> Result<()> {
        let file_name = file_path.unwrap_or(DEFAULT_FILE_NAME.to_string());
        let file = check_if_file_exists(file_name)?;
        let event_cache = cache
            .get_all_raw_events()
            .iter()
            .map(|e| {
                e.convert_to(
                    e.get_generator_instance()
                        .map_or(None, |id| cache.get_instances(id)),
                )
            })
            .collect::<Vec<PersistentModel>>();
        let cache = serde_json::to_vec(&event_cache).unwrap();

        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder
            .write_all(&*cache)
            .map_err(|_| DataPersistenceError)?;
        encoder.finish().map_err(|_| DataPersistenceError)?;
        Ok(())
    }

    pub async fn load(file_path: Option<String>) -> Result<Cache> {
        let file_name = file_path.unwrap_or(DEFAULT_FILE_NAME.to_string());
        let file = check_if_file_exists(file_name)?;
        let mut decoder = GzDecoder::new(file);
        let mut cache = Vec::new();
        decoder
            .read_to_end(&mut cache)
            .map_err(|_| DataPersistenceError)?;
        let cache: Vec<PersistentModel> =
            serde_json::from_slice(&*cache).map_err(|_| DataPersistenceError)?;
        let instance_vec = cache
            .iter()
            .filter_map(|e| e.generator_instance.clone())
            .collect();
        let event_cache: Vec<Box<dyn EventCommonTrait>> =
            cache.iter().map(|e| e.convert_to()).collect();
        let mut cache = Cache::init();
        cache.insert_events(event_cache).unwrap();
        cache.add_or_update_instances(instance_vec);
        Ok(cache)
    }
}

fn check_if_file_exists(file_name: String) -> Result<File> {
    if !Path::new(file_name.as_str()).exists() {
        let file = File::create(file_name.as_str());
        if file.is_ok() {
            return Ok(file.unwrap());
        }
    } else {
        let file = File::open(file_name.as_str());
        if file.is_ok() {
            return Ok(file.unwrap());
        }
    }
    error!("File read or create error");
    bail!(DataPersistenceError)
}
