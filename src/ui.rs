use tui::{layout::{Layout, Direction, Constraint, Alignment, Rect}, widgets::{Tabs, Paragraph, Block, Borders, BorderType}, style::{Style, Color}, text::{Spans, Span}};
use crate::*;

pub fn draw(f: &mut tui::Frame<TerminalBackend>, int10h: &mut Int10h) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)].as_ref())
        .split(f.size());
    
    let titles = Int10h::TITLES.iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .select(int10h.activate_tab.into())
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    f.render_widget(tabs, chunks[0]);

    match int10h.activate_tab {
        MenuItem::Home => draw_home(f, int10h, chunks[1]),
        MenuItem::MenuItem1 => {},
    }

    let copyright = Paragraph::new(format!("int10 v{} - all rights reserved", VERSION))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain)
        );
    
    f.render_widget(copyright, chunks[2]);
}

fn draw_home(f: &mut tui::Frame<TerminalBackend>, _int10h: &mut Int10h, area: Rect) {
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
            .border_type(BorderType::Plain),
    );
    f.render_widget(home, area);
}