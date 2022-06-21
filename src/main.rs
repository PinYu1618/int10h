//mod bios;

//use bios::Bios;

use std::error::Error;
use std::io;
use std::time::Duration;

mod apps;
mod int10h;
mod terminal;
mod ui;

use int10h::*;
use terminal::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);
    run(tick_rate)?;
    Ok(())
}