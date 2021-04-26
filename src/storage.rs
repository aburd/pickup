use crate::user_config::UserConfig;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};
use std::path::{self, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub obtained: bool,
    pub created_at: String,
}

trait Store {
    fn load_items(&mut self) -> io::Result<()>;
    fn persist_items(&self) -> io::Result<()>;
    fn get_items(&self) -> Option<&Vec<Item>>;
    fn get_item(&self, id: u64) -> Option<Item>;
    fn add_item(&mut self, item: Item) -> io::Result<()>;
    fn remove_item(&mut self, id: u64) -> io::Result<()>;
}

pub struct Storage {
    storage_path: PathBuf,
    items: Vec<Item>,
}

impl Storage {
    pub fn new(storage_path: PathBuf) -> Self {
        Storage {
            storage_path,
            items: Vec::new(),
        }
    }

    pub fn from_user_config(user_config: &UserConfig) -> io::Result<Self> {
        let mut storage = Storage::new(user_config.path.clone());
        storage.load_items()?;
        Ok(storage)
    }

    fn storage_path(&self) -> io::Result<path::PathBuf> {
        if let Some(path) = self.storage_path.to_str() {
            let path = path::PathBuf::from(format!("{}/{}", path, "items.json"));
            Ok(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Invalid path to storage file",
            ))
        }
    }
}

impl Store for Storage {
    fn load_items(&mut self) -> io::Result<()> {
        trace!("Getting store from user config...");
        let path = self.storage_path()?;
        let mut file = fs::File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let items: Vec<Item> = serde_json::from_str(&buf)?;

        debug!("JSON file parsed. Adding storage...");
        self.items = items;
        Ok(())
    }
    fn persist_items(&self) -> io::Result<()> {
        trace!("Persisting items to disk...");
        let path = self.storage_path()?;
        let _file = fs::OpenOptions::new().write(true).open(path)?;
        Ok(())
    }
    fn get_items(&self) -> Option<&Vec<Item>> {
        if self.items.len() > 0 {
            Some(&self.items)
        } else {
            None
        }
    }
    fn get_item(&self, id: u64) -> Option<Item> {
        for item in &self.items {
            if item.id == id {
                return Some(item.clone());
            }
        }
        None
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

mod test {}
