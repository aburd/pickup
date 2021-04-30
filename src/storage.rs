use dirs::home_dir;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub obtained: bool,
    pub created_at: String,
}

trait Store {
    fn persist_items(&self) -> io::Result<()>;
    fn load_items(&mut self) -> io::Result<()>;
    fn get_item(&self, id: u64) -> io::Result<&Item>;
    fn add_item(&mut self, item: Item) -> io::Result<()>;
    fn remove_item(&mut self, id: u64) -> io::Result<()>;
}

pub struct FileStorage {
    items: Vec<Item>,
}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage { items: Vec::new() }
    }

    fn json_file_path(&self) -> io::Result<String> {
        let config_path = self.config_dir_path()?;
        Ok(format!("{}/{}", config_path, "items.json"))
    }

    fn config_dir_path(&self) -> io::Result<String> {
        home_dir()
            .map(|home| format!("{}/{}", home.display(), ".pickup"))
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    }
}

impl Store for FileStorage {
    fn load_items(&mut self) -> io::Result<()> {
        trace!("Getting store from user config...");
        let path = self.json_file_path()?;
        let mut file = fs::File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let items: Vec<Item> = serde_json::from_str(&buf)?;
        self.items = items;

        debug!("JSON file parsed. Adding storage...");
        Ok(())
    }
    fn persist_items(&self) -> io::Result<()> {
        trace!("Persisting items to disk...");
        let path = self.json_file_path()?;
        let mut file = fs::OpenOptions::new().write(true).open(path)?;
        let buf = serde_json::to_string(&self.items)?;
        file.write(buf.as_bytes())?;
        Ok(())
    }
    fn get_item(&self, id: u64) -> io::Result<&Item> {
        for item in &self.items {
            if item.id == id {
                return Ok(&item);
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Invalid path to storage file",
        ))
    }
    fn add_item(&mut self, item: Item) -> io::Result<()> {
        self.items.push(item);
        self.persist_items()?;
        Ok(())
    }
    fn remove_item(&mut self, id: u64) -> io::Result<()> {
        if let Some(pos) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(pos);
            self.persist_items()?;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "ID to item is invalid",
            ))
        }
    }
}

mod test {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io;
    use std::path::PathBuf;
    use tempfile::TempDir;

    type TestResult = io::Result<()>;

    #[test]
    fn test_config_path() -> TestResult {
        let tmp_dir = set_config_dir()?;
        let storage = FileStorage::new();

        let actual = storage.config_dir_path()?;
        let expected = format!("{}/{}", tmp_dir.path().display(), ".pickup");

        assert_eq!(actual, expected);

        Ok(())
    }

    fn test_json_file_path() -> TestResult {
        let tmp_dir = set_config_dir()?;
        let storage = FileStorage::new();

        let actual = storage.json_file_path()?;
        let expected = format!(
            "{}/{}/{}",
            tmp_dir.path().display(),
            ".pickup",
            "items.json"
        );

        assert_eq!(actual, expected);

        Ok(())
    }

    fn set_config_dir() -> io::Result<TempDir> {
        let tmp_dir = TempDir::new()?;
        env::set_var("HOME", tmp_dir.path());
        Ok(tmp_dir)
    }
}
