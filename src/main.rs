//mod bios;

//use bios::Bios;

use std::error::Error;
use std::io;

mod app;
mod models;
mod observer;
mod rt;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;

use self::app::*;
use self::rt::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::default();

    let stdout = MouseTerminal::from(io::stdout().into_raw_mode()?); // don't want to use alternate screen because it is ugly :-(
    let backend = TermionBackend::new(stdout);
    let terminal = tui::Terminal::new(backend)?;

    let mut tui = Tui::new(terminal, 250);
    tui.init()?;

    tui.run(app)?;

    tui.exit()?;
    Ok(())
}
