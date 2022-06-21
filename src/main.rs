//mod bios;

//use bios::Bios;

use std::io;
use std::time::Duration;

mod apps;
mod int10h;
mod terminal;
mod ui;

use apps::*;
use int10h::*;
use terminal::*;

const DB_PATH: &str = "assets/db.json";

fn main() -> Result<(), io::Error> {
    let tick_rate = Duration::from_millis(250);
    Ok(Int10h::default().run(tick_rate))
}