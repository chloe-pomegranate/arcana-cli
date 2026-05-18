//! Reading notes screen - type notes before saving to journal

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Clear background
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    if let Some(ref reading) = app.reading {
        let header = Paragraph::new(
            Text::from(vec![
                Line::from(vec![
                    Span::styled("📝 ", Style::default().fg(app.theme.yellow)),
                    Span::styled(
                        "Add Notes",
                        Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(
                    Span::styled(
                        format!("{} — {} cards", reading.spread.name, reading.card_count()),
                        Style::default().fg(app.theme.lavender),
                    )
                ),
            ])
        )
        .alignment(Alignment::Center);

        f.render_widget(header, chunks[0]);
    }

    // Text input box
    let input_text = format!("{}{}", app.reading_notes, "▏");
    let input_block = Block::default()
        .title(" Notes ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.yellow))
        .title_style(Style::default().fg(app.theme.yellow).add_modifier(Modifier::BOLD));

    let input_para = Paragraph::new(Text::from(vec![Line::from(
        Span::styled(input_text, Style::default().fg(app.theme.text))
    )]))
    .block(input_block);

    f.render_widget(input_para, chunks[1]);

    // Help
    let help = Paragraph::new(
        Text::from(vec![Line::from(vec![
            Span::styled("Enter", Style::default().fg(app.theme.yellow)),
            Span::styled(" save  ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Esc", Style::default().fg(app.theme.yellow)),
            Span::styled(" cancel", Style::default().fg(app.theme.subtext0)),
        ])])
    )
    .alignment(Alignment::Center);

    f.render_widget(help, chunks[2]);
}
