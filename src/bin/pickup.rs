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
        .arg(
            Arg::with_name("show item")
                .short("s")
                .long("show")
                .takes_value(true)
                .help("Show an item from your list"),
        )
        .arg(
            Arg::with_name("remove item")
                .short("r")
                .long("remove")
                .takes_value(true)
                .help("Remove an item from your list"),
        )
        .arg(
            Arg::with_name("add item")
                .short("a")
                .long("add")
                .takes_value(true)
                .help("Add something you need to pickup"),
        )
        .get_matches();

    let opts = PickupOpts {
        list_items: matches.index_of("list").is_some(),
        show_item: (matches.index_of("show item").is_some(), matches.value_of("show item").map_or(0, |val| {
            val.parse::<usize>().unwrap()
        })),
        remove_item: (matches.index_of("show item").is_some(), matches.value_of("remove item").map_or(0, |val| {
            val.parse::<usize>().unwrap()
        })),
        add_item: (matches.index_of("add item").is_some(), matches.value_of("add item").map_or(String::new(), String::from)),
    };
    pickup.run(opts)?;

    Ok(())
}
