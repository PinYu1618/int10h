use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
};

// State and root rendering
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub activate_tab: MenuItem,
    pub current_screen: ScreenId,
}

#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Home,
    MenuItem1,
}

pub type ScreenId = usize;

impl MenuItem {
    pub fn as_u32(&self) -> u32 {
        match self {
            Self::Home => 0,
            Self::MenuItem1 => 1,
        }
    }
}

impl App {
    pub const TITLES: [&'static str; 2] = ["Tab0", "Tab1"];

    pub fn on_ctrl_c(&mut self) {
        self.should_quit = true;
    }

    pub fn on_press(&mut self, _x: u16, _y: u16) {}

    pub fn on_release(&self, _x: u16, _y: u16) {}

    pub fn on_hold(&self, _x: u16, _y: u16) {}

    pub fn tick(&self) {}

    pub fn render<B: tui::backend::Backend>(&mut self, frame: &mut tui::Frame<'_, B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(2), Constraint::Length(3)].as_ref())
            .split(frame.size());

        match self.current_screen {
            _ => {}
        }

        let titles = Self::TITLES.iter().cloned().map(Spans::from).collect();
        let tabs = Tabs::new(titles)
            .select(self.activate_tab.into())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));
        frame.render_widget(tabs, chunks[1]);
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

impl Default for MenuItem {
    fn default() -> Self {
        Self::Home
    }
}
