use termion::event::Key;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::Terminal;
use tui::backend::TermionBackend;

use std::error::Error;
use std::io::Stdout;
use std::sync::mpsc;
use std::thread;

use crate::*;

pub type Console = Terminal<TermionBackend<MouseTerminal<RawTerminal<Stdout>>>>;
pub type ConsoleBackend = TermionBackend<MouseTerminal<RawTerminal<Stdout>>>;

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let backend  = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let int10h = Int10h::default();
    run_inner(&mut terminal, int10h, tick_rate)?;

    Ok(())
}


enum Event {
    Key(Key),
    Tick,
}

fn run_inner(terminal: &mut Console, mut int10h: Int10h, tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let events = events(tick_rate);

    loop {
        terminal.draw(|f| ui::draw(f, &mut int10h))?;

        match events.recv()? {
            Event::Key(_key) => {}
            Event::Tick => {}
        }
        if int10h.should_quit {
            return Ok(());
        }
    }
}

fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(Event::Key(key)) {
                eprintln!("{}", err);
                return;
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{}", err);
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}