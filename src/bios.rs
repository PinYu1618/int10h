#![allow(unused)]

use termion::raw::{IntoRawMode, RawTerminal};

pub struct Bios {
    cursor_pos: Position,
    system_loaded: bool,
    terminal: Terminal,
}

impl Bios {
    pub fn run(&self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.system_loaded {
                break;
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        
        if self.system_loaded {
            Terminal::clear_screan();
        } else {
            self.fake_load();
            Terminal::cursor_position(&self.cursor_pos);
        }
        
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn fake_load(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            println!("\r");
        }
    }
}

impl Default for Bios {
    fn default() -> Self {
        Self {
            cursor_pos: Position::default(),
            system_loaded: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}

struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>
}

impl Terminal {
    fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size { width: size.0, height: size.1 },
            _stdout: std::io::stdout().into_raw_mode()?,
        })
    }

    fn size(&self) -> &Size {
        &self.size
    }

    fn clear_screan() {
        print!("{}", termion::clear::All);
    }

    fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    fn flush() -> Result<(), std::io::Error> {
        use std::io::Write;
        std::io::stdout().flush()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

}

#[derive(Debug, Default)]
struct Position {
    pub x: usize,
    pub y: usize,
}

fn die(e: std::io::Error) {
    Terminal::clear_screan();
    panic!("{}", e);
}