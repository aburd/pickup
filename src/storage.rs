use crate::user_config::UserConfig;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub obtained: bool,
    pub created_at: String,
}

trait Store {
    fn load_items(user_config: &UserConfig) -> io::Result<()>;
    fn get_items() -> io::Result<Vec<Item>>;
    fn get_item(id: u64) -> io::Result<Item>;
    fn set_item(item: Item) -> io::Result<Item>;
    fn add_item(item: Item) -> io::Result<Item>;
    fn remove_item(id: u64) -> bool;
}

pub struct Storage {
    items: Vec<Item>,
}

impl Storage {
    pub fn new() -> Self {
        Storage { items: Vec::new() }
    }

    pub fn from_user_config(user_config: &UserConfig) -> io::Result<Self> {
        trace!("Creating store from user config...");
        let mut file = fs::File::open(user_config.path.as_os_str())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let items: Vec<Item> = serde_json::from_str(&buf)?;

        debug!("JSON file parsed. Adding storage...");
        Ok(Storage { items })
    }
}

impl Store for Storage {
    fn load_items(user_config: &UserConfig) -> io::Result<()> {
        unimplemented!();
    }
    fn get_items() -> io::Result<Vec<Item>> {
        unimplemented!();
    }
    fn get_item(id: u64) -> io::Result<Item> {
        unimplemented!();
    }
    fn set_item(item: Item) -> io::Result<Item> {
        unimplemented!();
    }
    fn add_item(item: Item) -> io::Result<Item> {
        unimplemented!();
    }
    fn remove_item(id: u64) -> bool {
        unimplemented!();
    }
}

mod test {}
