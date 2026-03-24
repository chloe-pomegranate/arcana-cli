//! Card browser screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::app::{App, CardFilter};
use crate::tui::screens::{card_color, suit_symbol, arcana_symbol};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Header with filter
    let filter_text = match app.card_filter {
        CardFilter::All => "All Cards (78)".to_string(),
        CardFilter::Major => "Major Arcana (22)".to_string(),
        CardFilter::Suit(suit) => format!("{:?} (14)", suit),
    };

    let header = Paragraph::new(
        Text::from(vec![
            Line::from(vec![
                Span::styled("📖 ", Style::default()),
                Span::styled(
                    "Card Browser",
                    Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(
                Span::styled(filter_text, Style::default().fg(app.theme.lavender))
            ),
        ])
    )
    .alignment(Alignment::Center);

    f.render_widget(header, chunks[0]);

    // Card list
    let card_items: Vec<ListItem> = app
        .cards
        .iter()
        .map(|card| {
            let color = card_color(card, &app.theme);
            let symbol = match card.arcana {
                crate::cards::ArcanaType::Major => arcana_symbol(crate::cards::ArcanaType::Major),
                crate::cards::ArcanaType::Minor => {
                    if let Some(suit) = card.suit {
                        suit_symbol(suit)
                    } else {
                        arcana_symbol(crate::cards::ArcanaType::Minor)
                    }
                }
            };

            let line = Line::from(vec![
                Span::styled(symbol, Style::default().fg(color)),
                Span::styled(" ", Style::default()),
                Span::styled(
                    card.display_name(),
                    Style::default().fg(app.theme.text),
                ),
                Span::styled(
                    format!("  — {}", card.keywords[..card.keywords.len().min(3)].join(", ")),
                    Style::default().fg(app.theme.subtext0),
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let card_list = List::new(card_items)
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

    f.render_stateful_widget(card_list, chunks[1], &mut app.card_list_state);

    // Help
    let help = Paragraph::new(
        Text::from(vec![Line::from(vec![
            Span::styled("↑/k", Style::default().fg(app.theme.yellow)),
            Span::styled(" up  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("↓/j", Style::default().fg(app.theme.yellow)),
            Span::styled(" down  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Tab", Style::default().fg(app.theme.yellow)),
            Span::styled(" filter  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Enter", Style::default().fg(app.theme.yellow)),
            Span::styled(" view  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Esc/q", Style::default().fg(app.theme.yellow)),
            Span::styled(" back", Style::default().fg(app.theme.subtext0)),
        ])])
    )
    .alignment(Alignment::Center);

    f.render_widget(help, chunks[2]);
}
