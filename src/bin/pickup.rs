use clap::{App, Arg, SubCommand};
use env_logger;
use pickup::printer::Printer;
use pickup::reader::Reader;
use pickup::storage::FileStorage;
use pickup::{Pickup, PickupCommand, PickupOpts};
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    let stdio = io::stdin();
    let stdin = stdio.lock();
    let stdout = io::stdout();

    let mut pickup = Pickup::new(Reader::new(stdin), Printer::new(stdout), FileStorage::new());

    if !pickup.storage.config_dir_exists() {
        println!(
            "Configuration directory was not found. Creating one at {}",
            pickup.storage.config_dir_path()?
        );
        pickup.storage.create_config_dir()?;
    }
    if !pickup.storage.items_file_exists() {
        println!(
            "Storage file not found. Creating one at {}",
            pickup.storage.json_file_path()?
        );
        pickup.storage.create_items_file()?;
    }

    let matches = App::new("Pickup")
        .version("0.1")
        .author("Aaron Burdick")
        .about("Helps you remember to pick things up")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("ls")
                .help("List all the things you need to pickup"),
        )
        .get_matches();

    let opts = PickupOpts {
        list_items: matches.index_of("list").is_some(),
    };
    pickup.run(opts)?;

    Ok(())
}
