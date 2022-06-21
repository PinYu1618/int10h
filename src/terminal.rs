use termion::event::Key;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;

use std::io::Stdout;
use std::sync::mpsc;
use std::thread;

use crate::*;

pub struct Terminal {
    _terminal: tui::Terminal<TermionBackend<Stdout>>,
}

pub enum Event {
    Key(Key),
    Tick,
}

struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let backend  = TermionBackend::new(stdout);
        Ok(Self {
            _terminal: tui::Terminal::new(backend)?,
        })
    }

    pub fn process_events(tick_rate: Duration) -> mpsc::Receiver<Event> {
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
}