//! Journal detail screen - view a saved reading

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    if let Some((entry, content)) = &app.selected_journal_entry {
        let area = f.area();

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
                        &entry.spread_name,
                        Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(
                    Span::styled(
                        entry.formatted_date(),
                        Style::default().fg(app.theme.subtext0),
                    )
                ),
            ])
        )
        .alignment(Alignment::Center);

        f.render_widget(title, chunks[0]);

        // Content
        let text: Vec<Line> = content
            .lines()
            .map(|line| {
                // Simple markdown-ish formatting
                if line.starts_with("# ") {
                    // Main heading
                    Line::from(vec![
                        Span::styled(
                            line.trim_start_matches("# ").trim(),
                            Style::default()
                                .fg(app.theme.mauve)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else if line.starts_with("## ") {
                    // Sub heading
                    Line::from(vec![
                        Span::styled(
                            line.trim_start_matches("## ").trim(),
                            Style::default()
                                .fg(app.theme.lavender)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else if line.starts_with("### ") {
                    // Card heading - add empty line before for spacing
                    Line::from(vec![
                        Span::styled(
                            line.trim_start_matches("### ").trim(),
                            Style::default()
                                .fg(app.theme.sky)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else if line.starts_with("**") && line.ends_with("**") {
                    // Bold text (like Upright/Reversed)
                    let text = line.trim_start_matches("**").trim_end_matches("**");
                    let color = if text == "Reversed" {
                        app.theme.yellow
                    } else if text == "Upright" {
                        app.theme.green
                    } else {
                        app.theme.text
                    };
                    Line::from(vec![
                        Span::styled(
                            text,
                            Style::default()
                                .fg(color)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else if line.starts_with("**") {
                    // Keywords line
                    Line::from(
                        Span::styled(line, Style::default().fg(app.theme.text))
                    )
                } else if line.starts_with("- **") {
                    // Metadata bullet
                    Line::from(
                        Span::styled(line, Style::default().fg(app.theme.subtext0))
                    )
                } else if line.trim().is_empty() {
                    Line::from("")
                } else {
                    Line::from(
                        Span::styled(line, Style::default().fg(app.theme.text))
                    )
                }
            })
            .collect();

        let content_widget = Paragraph::new(Text::from(text))
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.theme.lavender))
            );

        f.render_widget(content_widget, chunks[1]);

        // Footer
        let footer = Paragraph::new(
            Text::from(vec![Line::from(vec![
                Span::styled("Enter/Esc/q", Style::default().fg(app.theme.yellow)),
                Span::styled(" back", Style::default().fg(app.theme.subtext0)),
            ])])
        )
        .alignment(Alignment::Center);

        f.render_widget(footer, chunks[2]);
    }
}
