use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::{
    Event as TioEvent, Key as TioKey, MouseButton as TioMouseBtn, MouseEvent as TioMouse,
};
use termion::input::TermRead;

use crate::*;

#[derive(Debug)]
pub struct Tui<B: tui::backend::Backend> {
    terminal: tui::Terminal<B>,
    pub events: TermEventHandler,
}

impl<B: tui::backend::Backend> Tui<B> {
    pub fn new(terminal: tui::Terminal<B>, tick_rate: u64) -> Self {
        let events = TermEventHandler::new(tick_rate);
        Self { terminal, events }
    }

    pub fn init(&mut self) -> Result<(), io::Error> {
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| app.render(frame))?;
        Ok(())
    }

    pub fn run(&mut self, mut app: App) -> Result<(), Box<dyn Error>> {
        while !app.should_quit {
            self.draw(&mut app)?;

            match self.events.next()? {
                Event::Key(TioKey::Ctrl('c')) => app.on_ctrl_c(),
                Event::Mouse(TioMouse::Press(TioMouseBtn::Left, x, y)) => {
                    self.events.mouse_state.left_pressed = true;
                    app.on_press(x, y);
                }
                Event::Mouse(TioMouse::Release(x, y)) => {
                    if self.events.mouse_state.left_pressed {
                        app.on_release(x, y);
                        self.events.mouse_state.left_pressed = false;
                    }
                }
                Event::Mouse(TioMouse::Hold(x, y)) => {
                    if self.events.mouse_state.left_pressed {
                        app.on_hold(x, y);
                    }
                }
                Event::Tick => app.tick(),
                _ => {}
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), io::Error> {
        self.terminal.clear()?;
        Ok(())
    }
}

/// Terminal events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Key(TioKey),
    Mouse(TioMouse),
    Tick,
}

// We need mouse state to store its history since termion only
// tell us the (x, y) (i.e. don't know which button) of release
// event and hold event. And only left button is used in int10h.
#[derive(Debug, Default)]
pub struct MouseState {
    left_pressed: bool,
}

#[derive(Debug)]
pub struct TermEventHandler {
    receiver: mpsc::Receiver<Event>,
    mouse_state: MouseState,
}

impl TermEventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        Self {
            receiver: events(tick_rate),
            mouse_state: MouseState::default(),
        }
    }

    pub fn next(&self) -> Result<Event, Box<dyn Error>> {
        Ok(self.receiver.recv()?)
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
                    TioEvent::Key(key) => {
                        if let Err(e) = input_tx.send(Event::Key(key)) {
                            eprintln!("{}", e);
                            return;
                        }
                    }
                    TioEvent::Mouse(me) => {
                        if let Err(e) = input_tx.send(Event::Mouse(me)) {
                            eprintln!("{}", e);
                            return;
                        }
                    }
                    _ => {}
                },
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
