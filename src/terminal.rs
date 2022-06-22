use termion::event::{Key, MouseButton, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend;

use std::error::Error;
use std::io::Stdout;
use std::sync::mpsc;
use std::thread;

use crate::*;

pub type Terminal = tui::Terminal<TerminalBackend>;
pub type TerminalBackend = backend::TermionBackend<MouseTerminal<RawTerminal<Stdout>>>;

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let backend  = backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    let mut mouse_state = MouseState::default();

    let int10h = Int10h::default();
    run_inner(&mut terminal, int10h, tick_rate, &mut mouse_state)?;

    Ok(())
}


enum Event {
    Key(Key),
    Mouse(MouseEvent),
    Tick,
}

pub struct MouseState {
    left_pressed: bool,
}

impl Default for MouseState {
    fn default() -> Self {
        Self { left_pressed: false }
    }
}

fn run_inner(terminal: &mut Terminal, mut int10h: Int10h, tick_rate: Duration, mouse_state: &mut MouseState) -> Result<(), Box<dyn Error>> {
    // don't know why but it seems like if we add mouse support,
    // terminal is not cleared automatically after initialization
    terminal.clear()?;

    let events = events(tick_rate);

    loop {
        terminal.draw(|f| ui::draw(f, &mut int10h))?;

        match events.recv()? {
            Event::Key(key) => match key {
                Key::Ctrl('c') => int10h.on_ctrl_c(),
                _ => {}
            }
            Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
                mouse_state.left_pressed = true;
                int10h.on_press(x, y);
            }
            Event::Mouse(MouseEvent::Release(x, y)) => {
                if mouse_state.left_pressed {
                    int10h.on_release(x, y);
                    mouse_state.left_pressed = false;
                }
            },
            Event::Mouse(MouseEvent::Hold(x, y)) => {
                if mouse_state.left_pressed {
                    int10h.on_hold(x, y);
                }
            }
            Event::Tick => {}
            _ => {}
        }

        if int10h.should_quit {
            terminal.clear()?;
            return Ok(());
        }
    }
}

fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for input in stdin.events() {
            match input {
                Ok(event) => match event {
                    termion::event::Event::Key(key) => {
                        if let Err(e) = input_tx.send(Event::Key(key)) {
                            eprintln!("{}", e);
                            return;
                        }
                    }
                    termion::event::Event::Mouse(me) => {
                        if let Err(e) = input_tx.send(Event::Mouse(me)) {
                            eprintln!("{}", e);
                            return;
                        }
                    }
                    _ => {}
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
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