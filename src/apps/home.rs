use tui::layout::Alignment;
use tui::style::{Style, Color};
use tui::widgets::{Paragraph, Block, Borders, BorderType};
use tui::text::*;

use crate::*;

pub struct Home<'a>;

impl<'a> App for Home<'a> {
    type W = Paragraph<'a>;

    fn render() -> Self::W {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "int10h",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }
}