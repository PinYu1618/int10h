use termion::event::Key;
use tui::text::{Spans, Span};
use tui::{widgets::Block, layout::Alignment};
use tui::style::{Style, Color, Modifier};
use tui::widgets::{Paragraph, Borders, BorderType, Tabs};

use thiserror::Error;
use tui::{widgets::ListState, layout::{Layout, Direction, Constraint}};
use crate::*;

pub struct Int10h {
    should_quit: bool,
    terminal: Terminal,
    activate_menu_item: MenuItem,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Home,
    MenuItem1,
}

impl Int10h {
    pub fn run(&mut self, tick_rate: Duration) {
        let events = Terminal::process_events(tick_rate);

        let menu_titles = vec!["Home", "MenuItem1", "MenuItem2"];
        let mut active_menu_item = MenuItem::Home;
        let mut pet_list_state = ListState::default();
        pet_list_state.select(Some(0));

        loop {
            self.terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(2),
                            Constraint::Length(3),
                        ].as_ref()
                    )
                    .split(size);
                
                let copyright = Paragraph::new("int10 2022 - all rights reserved")
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .title("Copyright")
                            .border_type(BorderType::Plain)
                    );
                
                let menu = menu_titles
                    .iter()
                    .map(|t| {
                        let (first, rest) = t.split_at(1);
                        Spans::from(vec![
                            Span::styled(
                                first,
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::UNDERLINED),
                            ),
                            Span::styled(rest, Style::default().fg(Color::White)),
                        ])
                    })
                    .collect();
                
                let tabs = Tabs::new(menu)
                    .select(self.active_menu_item.into())
                    .block(Block::default().title("Menu").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .divider(Span::raw("|"));
                
                rect.render_widget(tabs, chunks[0]);
                match self.active_menu_item {
                    MenuItem::Home => rect.render_widget(Home::render(), chunks[1]),
                    MenuItem::MenuItem1 => {
                        let pets_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                            )
                            .split(chunks[1]);
                        let (left, right) = render_pets(&pet_list_state);
                        rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                        rect.render_widget(right, pets_chunks[1]);
                    }
                }
                rect.render_widget(copyright, chunks[2]);
            })?;

            match events.recv()? {
                Event::Key(event) => match event {
                    Key::Char('q') => {
                        self.terminal.clear()?;
                        self.terminal.show_cursor()?;
                        break;
                    }
                    Key::Char('h') => self.active_menu_item = MenuItem::Home,
                    Key::Char('p') => self.active_menu_item = MenuItem::MenuItem1,
                    Key::Char('a') => {
                        add_random_pet_to_db().expect("can add new random pet");
                    }
                    Key::Char('d') => {
                        remove_pet_at_index(&mut pet_list_state).expect("can remove pet");
                    }
                    Key::Down => {
                        if let Some(selected) = pet_list_state.selected() {
                            let amount_pets = read_db().expect("can fetch pet list").len();
                            if selected >= amount_pets - 1 {
                                pet_list_state.select(Some(0));
                            } else {
                                pet_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    Key::Up => {
                        if let Some(selected) = pet_list_state.selected() {
                            let amount_pets = read_db().expect("can fetch pet list").len();
                            if selected > 0 {
                                pet_list_state.select(Some(selected - 1));
                            } else {
                                pet_list_state.select(Some(amount_pets - 1));
                            }
                        }
                    }
                    _ => {}
                },
                Event::Tick => {}
            }
            if self.should_quit {
                break;
            }
        }
    }
}

impl Default for Int10h {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default()?,
            activate_menu_item: MenuItem::Home,
        }
    }
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::MenuItem1 => 1,
        }
    }
}