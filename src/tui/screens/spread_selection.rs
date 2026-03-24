//! Spread selection screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(8),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(
        Text::from(vec![
            Line::from(vec![
                Span::styled("🔮 ", Style::default()),
                Span::styled(
                    "Choose Your Spread",
                    Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                ),
            ]),
        ])
    )
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    // Spread list
    let spread_items: Vec<ListItem> = app
        .spread_options
        .iter()
        .map(|spread| {
            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        format!("{} ", spread.name),
                        Style::default().fg(app.theme.lavender).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("({} cards)", spread.card_count()),
                        Style::default().fg(app.theme.subtext0),
                    ),
                ]),
                Line::from(
                    Span::styled(
                        spread.description,
                        Style::default().fg(app.theme.subtext0),
                    )
                ),
            ];
            ListItem::new(Text::from(lines))
        })
        .collect();

    let spread_list = List::new(spread_items)
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

    f.render_stateful_widget(spread_list, chunks[1], &mut app.spread_list_state);

    // Position preview
    if let Some(i) = app.spread_list_state.selected() {
        let spread = app.spread_options[i];
        let mut position_lines: Vec<Line> = vec![
            Line::from(vec![
                Span::styled("Positions:", Style::default().fg(app.theme.sky).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
        ];

        for (idx, pos) in spread.positions.iter().enumerate() {
            position_lines.push(Line::from(vec![
                Span::styled(
                    format!("{}. ", idx + 1),
                    Style::default().fg(app.theme.yellow),
                ),
                Span::styled(
                    format!("{} — ", pos.name),
                    Style::default().fg(app.theme.text).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    pos.description,
                    Style::default().fg(app.theme.subtext0),
                ),
            ]));
        }

        let position_block = Paragraph::new(Text::from(position_lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.theme.lavender))
            )
            .wrap(Wrap { trim: true });

        f.render_widget(position_block, chunks[2]);
    }

    // Help
    let help = Paragraph::new(
        Text::from(vec![Line::from(vec![
            Span::styled("↑/k", Style::default().fg(app.theme.yellow)),
            Span::styled(" up  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("↓/j", Style::default().fg(app.theme.yellow)),
            Span::styled(" down  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Enter", Style::default().fg(app.theme.yellow)),
            Span::styled(" select  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Esc/q", Style::default().fg(app.theme.yellow)),
            Span::styled(" back", Style::default().fg(app.theme.subtext0)),
        ])])
    )
    .alignment(Alignment::Center);

    f.render_widget(help, chunks[2]);
}
