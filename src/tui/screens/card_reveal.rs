//! Card reveal screen - reveals cards one by one

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::App;
use crate::tui::screens::{card_color, suit_symbol, arcana_symbol};

pub fn draw(f: &mut Frame, app: &mut App) {
    if let Some(ref reading) = app.reading {
        let area = f.area();

        // Header
        let header = Paragraph::new(
            Text::from(vec![
                Line::from(vec![
                    Span::styled("🔮 ", Style::default()),
                    Span::styled(
                        reading.spread.name,
                        Style::default().fg(app.theme.mauve).add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(
                    Span::styled(
                        format!("Card {} of {}", app.reveal_index + 1, reading.card_count()),
                        Style::default().fg(app.theme.subtext0),
                    )
                ),
            ])
        )
        .alignment(Alignment::Center);

        let header_area = Rect::new(area.x, area.y + 1, area.width, 4);
        f.render_widget(header, header_area);

        // Current card
        if let Some((card, position)) = reading.get_card_at(app.reveal_index) {
            let card_area = Rect::new(
                area.x + area.width / 4,
                area.y + 6,
                area.width / 2,
                area.height.saturating_sub(10),
            );

            draw_card_widget(f, card, position.name, &app.theme, card_area);
        }

        // Progress indicator
        let progress_y = area.y + area.height - 4;
        let progress_area = Rect::new(area.x, progress_y, area.width, 3);
        
        let mut progress_spans = vec![
            Span::styled("Revealed: ", Style::default().fg(app.theme.subtext0)),
        ];

        for i in 0..reading.card_count() {
            if i <= app.reveal_index {
                progress_spans.push(Span::styled(" ● ", Style::default().fg(app.theme.mauve)));
            } else {
                progress_spans.push(Span::styled(" ○ ", Style::default().fg(app.theme.surface1)));
            }
        }

        let progress = Paragraph::new(Text::from(vec![Line::from(progress_spans)]))
            .alignment(Alignment::Center);
        f.render_widget(progress, progress_area);

        // Help
        let help = Paragraph::new(
            Text::from(vec![Line::from(vec![
                Span::styled("Enter/Space/→", Style::default().fg(app.theme.yellow)),
                Span::styled(" reveal next  ", Style::default().fg(app.theme.subtext0)),
                Span::styled("Esc/q", Style::default().fg(app.theme.yellow)),
                Span::styled(" quit", Style::default().fg(app.theme.subtext0)),
            ])])
        )
        .alignment(Alignment::Center);

        let help_area = Rect::new(area.x, progress_y + 3, area.width, 1);
        f.render_widget(help, help_area);
    }
}

fn draw_card_widget(
    f: &mut Frame,
    drawn: &crate::cards::DrawnCard,
    position_name: &str,
    theme: &crate::tui::app::AppTheme,
    area: Rect,
) {
    let card = drawn.card;
    let color = card_color(card, theme);

    // Card block
    let block = Block::default()
        .title(vec![
            Span::styled(position_name, Style::default().fg(theme.sky).add_modifier(Modifier::BOLD)),
        ])
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(3),
        ])
        .split(inner);

    // Card name and symbol
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

    let name_text = if drawn.reversed {
        Text::from(vec![
            Line::from(vec![
                Span::styled(symbol, Style::default().fg(color)),
                Span::styled(" ", Style::default()),
                Span::styled(
                    card.display_name(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(
                Span::styled("⟳ REVERSED", Style::default().fg(theme.yellow).add_modifier(Modifier::BOLD))
            ),
        ])
    } else {
        Text::from(vec![Line::from(vec![
            Span::styled(symbol, Style::default().fg(color)),
            Span::styled(" ", Style::default()),
            Span::styled(
                card.display_name(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
        ])])
    };

    let name_para = Paragraph::new(name_text).alignment(Alignment::Center);
    f.render_widget(name_para, chunks[0]);

    // Keywords
    let keywords = Paragraph::new(
        Text::from(vec![Line::from(
            Span::styled(card.keywords_string(), Style::default().fg(theme.subtext0))
        )])
    )
    .alignment(Alignment::Center);
    f.render_widget(keywords, chunks[1]);

    // Meaning
    let meaning = if drawn.reversed {
        card.reversed
    } else {
        card.upright
    };

    let meaning_color = if drawn.reversed { theme.yellow } else { theme.green };
    let meaning_title = if drawn.reversed { "Reversed" } else { "Upright" };

    let meaning_text = Text::from(vec![
        Line::from(vec![
            Span::styled(meaning_title, Style::default().fg(meaning_color).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled(meaning, Style::default().fg(theme.text))),
    ]);

    let meaning_para = Paragraph::new(meaning_text)
        .wrap(Wrap { trim: true });
    f.render_widget(meaning_para, chunks[2]);
}
