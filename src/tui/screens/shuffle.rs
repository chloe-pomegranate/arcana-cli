//! Shuffle animation screen

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    
    // Clear the background
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(10),
            Constraint::Percentage(40),
        ])
        .split(area);

    // Shuffle animation
    let progress = app.shuffle_progress;
    let bar_width = 40;
    let filled = (progress * bar_width as f32) as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(empty)
    );

    // Cards animation
    let card_anim = if progress < 0.33 {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  🂠  ", Style::default().fg(app.theme.surface1)),
                Span::styled("  🂠  ", Style::default().fg(app.theme.surface1)),
                Span::styled("  🂠  ", Style::default().fg(app.theme.surface1)),
            ]),
            Line::from(""),
            Line::from(
                Span::styled("Shuffling...", Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD))
            ),
        ]
    } else if progress < 0.66 {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  🃟  ", Style::default().fg(app.theme.lavender)),
                Span::styled("  🃏  ", Style::default().fg(app.theme.mauve)),
                Span::styled("  🃟  ", Style::default().fg(app.theme.lavender)),
            ]),
            Line::from(""),
            Line::from(
                Span::styled("Shuffling...", Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD))
            ),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ✦  ", Style::default().fg(app.theme.yellow)),
                Span::styled("  ✦  ", Style::default().fg(app.theme.mauve)),
                Span::styled("  ✦  ", Style::default().fg(app.theme.yellow)),
            ]),
            Line::from(""),
            Line::from(
                Span::styled("Preparing your reading...", Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD))
            ),
        ]
    };

    let shuffle_text = Paragraph::new(Text::from(card_anim))
        .alignment(Alignment::Center);

    f.render_widget(shuffle_text, chunks[0]);

    // Progress bar
    let progress_text = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(bar, Style::default().fg(app.theme.lavender))),
        Line::from(""),
        Line::from(
            Span::styled(
                format!("{:.0}%", progress * 100.0),
                Style::default().fg(app.theme.sky),
            )
        ),
    ]))
    .alignment(Alignment::Center);

    f.render_widget(progress_text, chunks[1]);

    // Help text
    let help = Paragraph::new(
        Text::from(vec![Line::from(
            Span::styled(
                "Press 'q' to cancel",
                Style::default().fg(app.theme.subtext0),
            )
        )])
    )
    .alignment(Alignment::Center);

    f.render_widget(help, chunks[2]);
}
