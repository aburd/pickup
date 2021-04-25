use std::io;
use std::collections::BTreeMap;
use chrono::prelude::*;
use crate::user_config::UserConfig;

struct Item {
    id: u64,
    name: String,
    obtained: bool,
    created_at: DateTime<Utc>,
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
    items: BTreeMap<u64, Item>
}

impl Storage {
  pub fn new() -> Self {
      Storage { items: BTreeMap::new() }
  }
  pub fn from_user_config(user_config: &UserConfig) -> io::Result<Self> {
    unimplemented!();
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

mod test {
    
}