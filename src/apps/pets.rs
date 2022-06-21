use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};
use tui::{widgets::{ListState, Table, List, Block, Borders, BorderType, ListItem, Row}, style::{Color, Style, Modifier}, text::{Spans, Span}, layout::Constraint, buffer::Cell};
use crate::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

pub fn render_pets<'a>(pet_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Pets")
        .border_type(BorderType::Plain);

    let pet_list = read_db().expect("can fetch pet list");
    let items: Vec<_> = pet_list
        .iter()
        .map(|pet| {
            ListItem::new(Spans::from(vec![Span::styled(
                pet.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_pet = pet_list
        .get(
            pet_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(pets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let pet_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_pet.id.to_string())),
        Cell::from(Span::raw(selected_pet.name)),
        Cell::from(Span::raw(selected_pet.category)),
        Cell::from(Span::raw(selected_pet.age.to_string())),
        Cell::from(Span::raw(selected_pet.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    (list, pet_detail)
}

pub fn read_db() -> Result<Vec<Pet>, Error> {
    let db_content = std::fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn add_random_pet_to_db() -> Result<Vec<Pet>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = std::fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
    let catsdogs = match rng.gen() {
        0 => "cats",
        _ => "dogs",
    };

    let random_pet = Pet {
        id: rng.gen_range(0..9999999),
        name: (&mut rng).sample_iter(Alphanumeric).take(10).map(char::from).collect(),
        category: catsdogs.to_owned(),
        age: rng.gen_range(1..15),
        created_at: Utc::now(),
    };

    parsed.push(random_pet);
    std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_pet_at_index(pet_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = pet_list_state.selected() {
        let db_content = std::fs::read_to_string(DB_PATH)?;
        let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        pet_list_state.select(Some(selected - 1));
    }
    Ok(())
}