#![allow(unused)]

use std::io::Stdout;

use tui::{layout::{Layout, Direction, Constraint, Alignment}, widgets::{Tabs, Paragraph, Block, Borders, BorderType}, backend::TermionBackend, style::{Style, Color}};

pub fn draw(f: &mut tui::Frame<TermionBackend<Stdout>>) {
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
        .split(f.size());

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
}