use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::sync::{Arc, Mutex};

use crate::app::{get_app_state, AppState};

pub fn draw(f: &mut Frame) {
    let app_state = get_app_state();
    let state = app_state.lock().unwrap();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Render title
    let title = Paragraph::new(state.menu_title.clone())
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, left_chunks[0]);

    // Render menu
    let menu_items: Vec<ListItem> = state
        .current_menu
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == state.current_menu.selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![Span::styled(&item.label, style)]))
        })
        .collect();

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">");

    f.render_widget(menu, left_chunks[1]);

    // Render right panels
    render_content(f, right_chunks[0], &state.log_data.right_top, "Right Top");
    render_content(
        f,
        right_chunks[1],
        &state.log_data.right_bottom,
        "Right Bottom",
    );
}

fn render_content(f: &mut Frame, area: Rect, content: &[String], title: &str) {
    let inner_area = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .inner(area);

    let height = inner_area.height as usize;
    let content = if content.len() > height {
        content
            .iter()
            .skip(content.len() - height)
            .cloned()
            .collect()
    } else {
        content.to_vec()
    };

    let paragraph = Paragraph::new(content.join("\n"))
        .wrap(Wrap { trim: true })
        .scroll((0, 0));

    f.render_widget(Block::default().borders(Borders::ALL).title(title), area);
    f.render_widget(paragraph, inner_area);
}
