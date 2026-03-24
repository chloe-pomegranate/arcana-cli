//! Journal screen - past readings

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Load entries if empty
    if app.journal_entries.is_empty() {
        app.load_journal_entries();
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(
        Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("📓 ", Style::default()),
                Span::styled(
                    "Journal",
                    Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(
                Span::styled(
                    format!("{} saved readings", app.journal_entries.len()),
                    Style::default().fg(app.theme.subtext0),
                )
            ),
        ])
    )
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    // Entries list
    if app.journal_entries.is_empty() {
        let empty = Paragraph::new(
            Text::from(vec![
                Line::from(""),
                Line::from(
                    Span::styled("No saved readings yet.", Style::default().fg(app.theme.subtext0))
                ),
                Line::from(""),
                Line::from(
                    Span::styled(
                        "Complete a reading and press 's' to save it.",
                        Style::default().fg(app.theme.subtext0),
                    )
                ),
            ])
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.lavender))
        );

        f.render_widget(empty, chunks[1]);
    } else {
        let entries: Vec<ListItem> = app
            .journal_entries
            .iter()
            .map(|entry| {
                let line = Line::from(vec![
                    Span::styled(
                        entry.formatted_date(),
                        Style::default().fg(app.theme.sky),
                    ),
                    Span::styled(" — ", Style::default().fg(app.theme.subtext0)),
                    Span::styled(
                        &entry.spread_name,
                        Style::default().fg(app.theme.text).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" ({} cards)", entry.card_count),
                        Style::default().fg(app.theme.subtext0),
                    ),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(entries)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.theme.lavender))
            )
            .highlight_style(
                Style::default()
                    .fg(app.theme.mauve)
                    .bg(app.theme.surface0)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        f.render_stateful_widget(list, chunks[1], &mut app.journal_list_state);
    }

    // Footer
    let footer = Paragraph::new(
        Text::from(vec![Line::from(vec![
            Span::styled("↑/k", Style::default().fg(app.theme.yellow)),
            Span::styled(" up  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("↓/j", Style::default().fg(app.theme.yellow)),
            Span::styled(" down  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Enter", Style::default().fg(app.theme.yellow)),
            Span::styled(" view  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Esc/q", Style::default().fg(app.theme.yellow)),
            Span::styled(" back", Style::default().fg(app.theme.subtext0)),
        ])])
    )
    .alignment(Alignment::Center);

    f.render_widget(footer, chunks[2]);
}
