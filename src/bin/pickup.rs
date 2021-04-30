use env_logger;
use pickup::printer::Printer;
use pickup::reader::Reader;
use pickup::storage::FileStorage;
use pickup::{Pickup, PickupOpts};
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    let stdio = io::stdin();
    let stdin = stdio.lock();
    let stdout = io::stdout();

    let mut pickup = Pickup::new(Reader::new(stdin), Printer::new(stdout), FileStorage::new());
    let opts = PickupOpts {};

    if !pickup.storage.config_dir_exists() {
        pickup.storage.create_config_dir()?;
    }
    if !pickup.storage.items_file_exists() {
        pickup.storage.create_items_file()?;
    }

    pickup.run(opts)?;

    Ok(())
}
