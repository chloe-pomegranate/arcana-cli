//! Home screen with main menu

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(
        Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("✨ ", Style::default().fg(app.theme.yellow)),
                Span::styled("ARCANA", Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD)),
                Span::styled(" ✨", Style::default().fg(app.theme.yellow)),
            ]),
            Line::from(""),
            Line::from(
                Span::styled(
                    "Tarot",
                    Style::default().fg(app.theme.subtext0),
                )
            ),
        ])
    )
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    // Menu
    let menu_items: Vec<ListItem> = app
        .menu_items
        .iter()
        .map(|item| {
            let style = Style::default().fg(app.theme.text);
            ListItem::new(item.as_str()).style(style)
        })
        .collect();

    let menu = List::new(menu_items)
        .block(
            Block::default()
                .title(" Menu ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.lavender))
                .title_style(Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD))
        )
        .highlight_style(
            Style::default()
                .fg(app.theme.mauve)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(menu, chunks[1], &mut app.menu_state);

    // Help text
    let help = Paragraph::new(
        Text::from(vec![
            Line::from(vec![
                Span::styled("↑/k", Style::default().fg(app.theme.yellow)),
                Span::styled(" up  ", Style::default().fg(app.theme.subtext0)),
                Span::styled("↓/j", Style::default().fg(app.theme.yellow)),
                Span::styled(" down  ", Style::default().fg(app.theme.subtext0)),
                Span::styled("Enter", Style::default().fg(app.theme.yellow)),
                Span::styled(" select  ", Style::default().fg(app.theme.subtext0)),
                Span::styled("q", Style::default().fg(app.theme.yellow)),
                Span::styled(" quit", Style::default().fg(app.theme.subtext0)),
            ]),
        ])
    )
    .alignment(Alignment::Center);

    f.render_widget(help, chunks[2]);
}
