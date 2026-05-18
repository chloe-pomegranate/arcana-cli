//! Daily Card screen - shows the card of the day

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::App;
use crate::tui::screens::{card_color, suit_symbol, arcana_symbol};

pub fn draw(f: &mut Frame, app: &mut App) {
    if let Some(card) = app.daily_card {
        let area = f.area();

        // Clear background
        f.render_widget(Clear, area);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(6),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        // Header with date
        let today = chrono::Local::now().date_naive();
        let header = Paragraph::new(
            Text::from(vec![
                Line::from(vec![
                    Span::styled("🌅 ", Style::default().fg(app.theme.yellow)),
                    Span::styled(
                        "Card of the Day",
                        Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(
                    Span::styled(
                        today.format("%A, %B %d, %Y").to_string(),
                        Style::default().fg(app.theme.lavender),
                    )
                ),
            ])
        )
        .alignment(Alignment::Center);

        f.render_widget(header, main_chunks[0]);

        // Card info + meanings
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ])
            .split(main_chunks[1]);

        draw_card_info(f, card, &app.theme, content_chunks[0]);

        let meanings_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(content_chunks[1]);

        draw_meaning_box(f, card, false, &app.theme, meanings_chunks[0]);
        draw_meaning_box(f, card, true, &app.theme, meanings_chunks[1]);

        // Help at bottom
        let help = Paragraph::new(
            Text::from(vec![Line::from(vec![
                Span::styled("Enter/Esc/q", Style::default().fg(app.theme.yellow)),
                Span::styled(" back", Style::default().fg(app.theme.subtext0)),
            ])])
        )
        .alignment(Alignment::Center);
        f.render_widget(help, main_chunks[2]);
    }
}

fn draw_card_info(f: &mut Frame, card: &crate::cards::Card, theme: &crate::tui::app::AppTheme, area: ratatui::layout::Rect) {
    let color = card_color(card, theme);
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

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![
        Line::from(vec![
            Span::styled(symbol, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(" ", Style::default()),
            Span::styled(
                card.display_name(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];

    if card.arcana == crate::cards::ArcanaType::Major {
        lines.push(Line::from(
            Span::styled("Major Arcana", Style::default().fg(theme.mauve))
        ));
    } else {
        lines.push(Line::from(vec![
            Span::styled(
                format!("Minor Arcana — {:?}", card.suit.unwrap()),
                Style::default().fg(theme.lavender),
            ),
        ]));
        lines.push(Line::from(
            Span::styled(
                format!("Element: {:?}", card.suit.unwrap().element()),
                Style::default().fg(theme.subtext0),
            )
        ));
    }

    if let Some(astrology) = card.astrology {
        lines.push(Line::from(
            Span::styled(format!("Astrology: {}", astrology), Style::default().fg(theme.subtext0))
        ));
    }
    if let Some(numerology) = card.numerology {
        lines.push(Line::from(
            Span::styled(format!("Numerology: {}", numerology), Style::default().fg(theme.subtext0))
        ));
    }
    lines.push(Line::from(
        Span::styled(format!("Yes/No: {}", card.yes_or_no), Style::default().fg(theme.subtext0))
    ));

    lines.push(Line::from(""));
    lines.push(Line::from(
        Span::styled(
            format!("Keywords: {}", card.keywords_string()),
            Style::default().fg(theme.subtext0),
        )
    ));

    let para = Paragraph::new(Text::from(lines)).alignment(Alignment::Center).wrap(Wrap { trim: true });
    f.render_widget(para, inner);
}

fn draw_meaning_box(
    f: &mut Frame,
    card: &crate::cards::Card,
    reversed: bool,
    theme: &crate::tui::app::AppTheme,
    area: ratatui::layout::Rect,
) {
    let (title, meaning, color) = if reversed {
        (" Reversed ", card.reversed, theme.yellow)
    } else {
        (" Upright ", card.upright, theme.green)
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD));

    let para = Paragraph::new(Text::from(vec![Line::from(
        Span::styled(meaning, Style::default().fg(theme.text))
    )]))
    .wrap(Wrap { trim: true })
    .block(block);

    f.render_widget(para, area);
}
