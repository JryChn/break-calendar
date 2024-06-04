#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use chrono::{DateTime, Utc};
    use tempfile::tempdir;

    use crate::cache::Cache;
    use crate::model::event::Event;
    use crate::model::EventCommonTrait;
    use crate::persistent::file_system::{DEFAULT_FILE_NAME, FilePersistenceSystem};

    #[test]
    fn save_load_cache_successfully() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(DEFAULT_FILE_NAME);
        let file_path = Some(file_path.to_str().unwrap().to_string());
        let mut cache = Cache::init();
        let mut event = Event::init(None);
        let id = event.get_id();
        let start_time = DateTime::from(Utc::now());
        let end_time = DateTime::from(Utc::now());
        event.set_duration(start_time, end_time);
        cache.insert_events(vec![Box::new(event)]).unwrap();
        FilePersistenceSystem::save(&cache, file_path.clone()).unwrap();

        let loaded_cache = FilePersistenceSystem::load(file_path).unwrap();

        assert_eq!(id, loaded_cache.get_events_by_id::<Event>(id).unwrap().get_id());
        assert_eq!(
            id,
            loaded_cache
                .get_events_by_day::<Event>(start_time.date_naive())
                .first()
                .unwrap()
                .get_id()
        );
    }

    #[test]
    fn save_fails_when_cannot_write_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(DEFAULT_FILE_NAME);
        fs::write(&file_path, "unwritable").unwrap();
        let mut permissions = fs::metadata(&file_path).unwrap().permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&file_path, permissions).unwrap();
        let file_path = Some(file_path.to_str().unwrap().to_string());

        let cache = Cache::init();
        let result = FilePersistenceSystem::save(&cache, file_path);

        assert!(result.is_err());
    }

    #[test]
    fn load_fails_when_cannot_read_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(DEFAULT_FILE_NAME);
        fs::write(&file_path, "unreadable").unwrap();
        let mut permissions = fs::metadata(&file_path).unwrap().permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&file_path, permissions).unwrap();
        let file_path = Some(file_path.to_str().unwrap().to_string());

        let result = FilePersistenceSystem::load(file_path);

        assert!(result.is_err());
    }
}
