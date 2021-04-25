use std::io;
use pickup::{Pickup, PickupOpts};
use pickup::reader::{Reader};
use pickup::printer::{Printer};

fn main() -> io::Result<()> {
    let stdio = io::stdin();
    let stdin = stdio.lock();
    let stdout = io::stdout();

    let mut pickup = Pickup::new(
        Reader::new(stdin),
        Printer::new(stdout),
    );
    let opts = PickupOpts {};

    pickup.run(opts)?;

    Ok(())
}