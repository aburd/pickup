use dirs::home_dir;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{self, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub obtained: bool,
    pub created_at: String,
}

trait Store {
    fn persist_items(&self, items: &Vec<Item>) -> io::Result<()>;
    fn get_items(&self) -> io::Result<Vec<Item>>;
    fn get_item(&self, id: u64) -> io::Result<Item>;
    fn add_item(&mut self, item: Item) -> io::Result<()>;
    fn remove_item(&mut self, id: u64) -> io::Result<()>;
}

pub struct FileStorage {}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {}
    }

    fn json_file_path(&self) -> io::Result<path::PathBuf> {
        let config_path = self.config_dir_path()?;
        format!("{}/{}", config_path, "items.json")
    }

    fn config_dir_path(&self) -> io::Result<String> {
        home_dir()
            .map(|home| format!("{}/{}", home.display(), ".pickup"))
            .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
    }
}

impl Store for FileStorage {
    fn get_items(&self) -> io::Result<Vec<Item>> {
        trace!("Getting store from user config...");
        let path = self.storage_path()?;
        let mut file = fs::File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let items: Vec<Item> = serde_json::from_str(&buf)?;

        debug!("JSON file parsed. Adding storage...");
        Ok(items)
    }
    fn persist_items(&self, items: &Vec<Item>) -> io::Result<()> {
        trace!("Persisting items to disk...");
        let path = self.storage_path()?;
        let mut file = fs::OpenOptions::new().write(true).open(path)?;
        let buf = serde_json::to_string(items)?;
        file.write(buf.as_bytes())?;
        Ok(())
    }
    fn get_item(&self, id: u64) -> io::Result<Item> {
        let items = self.get_items()?;
        for item in items {
            if item.id == id {
                return Ok(item);
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Invalid path to storage file",
        ))
    }
    fn add_item(&mut self, item: Item) -> io::Result<()> {
        let mut items = self.get_items()?;
        items.push(item);
        self.persist_items(&items)?;
        Ok(())
    }
    fn remove_item(&mut self, id: u64) -> io::Result<()> {
        let mut items = self.get_items()?;
        if let Some(pos) = items.iter().position(|i| i.id == id) {
            items.remove(pos);
            self.persist_items(&items)?;
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
    fn test_storage_path() -> TestResult {
        let test_home_path = "/home/aaron";
        let path = PathBuf::from(test_home_path);
        let file_storage = FileStorage::new(path);

        let s = format!("{}/{}", test_home_path, "items.json");
        let expected = PathBuf::from(s);
        let actual = file_storage.storage_path()?;

        assert_eq!(expected, actual);
        Ok(())
    }

    fn test_with_storage(func: &dyn Fn(FileStorage) -> TestResult) -> TestResult {
        let mut file = File::create("test_file")?;

        let item_json = b"[{\"id\": 1,
        \"name\": \"ayano\",
        \"obtained\": false,
        \"created_at\": \"2021-04-26T15:38:04.341Z\"}]";

        file.write(item_json)?;

        let mut f_storage = FileStorage::new("test_file".into());

        func(f_storage)?;
        fs::remove_file("test_file")?;

        Ok(())
    }

    fn test_get_items(f_storage: FileStorage) -> TestResult {
        let items = f_storage.get_items()?;
        assert_eq!(items.len(), 1);
        Ok(())
    }

    #[test]
    fn test_get_items_len() {
        test_with_storage(&test_get_items).unwrap();
    }

    fn set_config_dir() -> io::Result<(PathBuf, TempDir)> {
        let tmp_dir = TempDir::new()?;
        let config_dir = tmp_dir.path().join(".pickup");

        env::set_var("HOME", tmp_dir.path());

        Ok((config_dir, tmp_dir))
    }

    fn set_and_create_config_dir() -> io::Result<(PathBuf, TempDir)> {
        let (config_dir, tmp_dir) = set_config_dir()?;

        fs::create_dir_all(&config_dir)?;

        Ok((config_dir, tmp_dir))
    }
}
