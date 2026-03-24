//! Help screen - keyboard shortcuts

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
        .margin(4)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(
        Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("❓ ", Style::default()),
                Span::styled(
                    "Keyboard Shortcuts",
                    Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                ),
            ]),
        ])
    )
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    // Help content
    let help_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Global", Style::default().fg(app.theme.sky).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  q / Esc", Style::default().fg(app.theme.yellow)),
            Span::styled("      Quit or go back", Style::default().fg(app.theme.text)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation", Style::default().fg(app.theme.sky).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ↑ / k", Style::default().fg(app.theme.yellow)),
            Span::styled("        Move up", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  ↓ / j", Style::default().fg(app.theme.yellow)),
            Span::styled("        Move down", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(app.theme.yellow)),
            Span::styled("          Select / Confirm", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  Tab", Style::default().fg(app.theme.yellow)),
            Span::styled("            Cycle filter (in browser)", Style::default().fg(app.theme.text)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Screens", Style::default().fg(app.theme.sky).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Home", Style::default().fg(app.theme.lavender)),
            Span::styled("           Main menu with all options", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  New Reading", Style::default().fg(app.theme.lavender)),
            Span::styled("    Choose spread type", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  Card Browser", Style::default().fg(app.theme.lavender)),
            Span::styled("   Browse all 78 cards", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  Card Detail", Style::default().fg(app.theme.lavender)),
            Span::styled("    View full card meanings", Style::default().fg(app.theme.text)),
        ]),
        Line::from(vec![
            Span::styled("  Journal", Style::default().fg(app.theme.lavender)),
            Span::styled("        View past readings (coming soon)", Style::default().fg(app.theme.text)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Reading", Style::default().fg(app.theme.sky).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Space / →", Style::default().fg(app.theme.yellow)),
            Span::styled("    Reveal next card", Style::default().fg(app.theme.text)),
        ]),
    ]);

    let help_para = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.lavender))
        );

    f.render_widget(help_para, chunks[1]);

    // Footer
    let footer = Paragraph::new(
        Text::from(vec![Line::from(vec![
            Span::styled("Press ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Enter", Style::default().fg(app.theme.yellow)),
            Span::styled(" or ", Style::default().fg(app.theme.subtext0)),
            Span::styled("Esc", Style::default().fg(app.theme.yellow)),
            Span::styled(" to return", Style::default().fg(app.theme.subtext0)),
        ])])
    )
    .alignment(Alignment::Center);

    f.render_widget(footer, chunks[2]);
}
