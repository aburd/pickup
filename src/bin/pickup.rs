use env_logger;
use pickup::printer::Printer;
use pickup::reader::Reader;
use pickup::{Pickup, PickupOpts};
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();

    let stdio = io::stdin();
    let stdin = stdio.lock();
    let stdout = io::stdout();

    let mut pickup = Pickup::new(Reader::new(stdin), Printer::new(stdout));
    let opts = PickupOpts {};

    pickup.run(opts)?;

    Ok(())
}
