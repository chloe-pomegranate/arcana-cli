//! Reading complete - show all cards with visual layout

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::tui::app::App;
use crate::tui::screens::card_color;
use crate::tui::widgets::{LayoutColors, SpreadLayoutWidget};

pub fn draw(f: &mut Frame, app: &mut App) {
    if let Some(ref reading) = app.reading {
        let area = f.area();
        let color = app.theme.mauve;

        // Clear background
        f.render_widget(Clear, area);

        let has_layout = reading.spread.layout != crate::spreads::SpreadLayout::Single
            && reading.card_count() > 1;

        let constraints = if has_layout {
            vec![
                Constraint::Length(5),
                Constraint::Percentage(45),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
        } else {
            vec![
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(constraints)
            .split(area);

        // Header
        let header = Paragraph::new(
            Text::from(vec![
                Line::from(vec![
                    Span::styled("✨ ", Style::default().fg(app.theme.yellow)),
                    Span::styled(
                        "Reading Complete",
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" ✨", Style::default().fg(app.theme.yellow)),
                ]),
                Line::from(
                    Span::styled(reading.spread.name, Style::default().fg(app.theme.lavender))
                ),
            ])
        )
        .alignment(Alignment::Center);

        f.render_widget(header, chunks[0]);

        let mut chunk_idx = 1;

        // Visual layout (if applicable)
        if has_layout {
            let cards_refs: Vec<_> = reading.drawn.iter().collect();
            let layout_colors = LayoutColors {
                border: app.theme.lavender,
                card_border: app.theme.surface1,
                text: app.theme.text,
                number: app.theme.sky,
            };

            let layout_widget = SpreadLayoutWidget::new(
                reading.spread.layout,
                &cards_refs,
                layout_colors,
            );

            f.render_widget(layout_widget, chunks[chunk_idx]);
            chunk_idx += 1;
        }

        // Cards list
        draw_cards_list(f, reading, app, chunks[chunk_idx]);
        chunk_idx += 1;

        // Help
        let help = Paragraph::new(
            Text::from(vec![Line::from(vec![
                Span::styled("Enter/Esc/q", Style::default().fg(app.theme.yellow)),
                Span::styled(" return to menu  ", Style::default().fg(app.theme.subtext0)),
                Span::styled("s", Style::default().fg(app.theme.yellow)),
                Span::styled(" save to journal", Style::default().fg(app.theme.subtext0)),
            ])])
        )
        .alignment(Alignment::Center);

        f.render_widget(help, chunks[chunk_idx]);
    }
}

fn draw_cards_list(
    f: &mut Frame,
    reading: &crate::spreads::Reading,
    app: &App,
    area: Rect,
) {
    // Split area horizontally for cards
    let num_cards = reading.card_count();
    let constraints: Vec<Constraint> = if num_cards <= 3 {
        // Horizontal layout for few cards
        (0..num_cards)
            .map(|_| Constraint::Percentage((100 / num_cards) as u16))
            .collect()
    } else {
        // Grid layout for many cards
        let cols = 3;
        let _rows = (num_cards + cols - 1) / cols;
        (0..num_cards)
            .map(|_| Constraint::Percentage((100 / cols) as u16))
            .collect()
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, ((card, position), chunk)) in reading
        .drawn
        .iter()
        .zip(reading.spread.positions.iter())
        .zip(chunks.iter())
        .enumerate()
    {
        draw_mini_card(f, card, position.name, i + 1, app, *chunk);
    }
}

fn draw_mini_card(
    f: &mut Frame,
    drawn: &crate::cards::DrawnCard,
    position_name: &str,
    position_num: usize,
    app: &App,
    area: Rect,
) {
    let card = drawn.card;
    let color = card_color(card, &app.theme);

    let title = format!("{}. {}", position_num, position_name);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![];

    // Card name
    let name_line = if drawn.reversed {
        Line::from(vec![
            Span::styled(
                card.display_name(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ⟳", Style::default().fg(app.theme.yellow)),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                card.display_name(),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
        ])
    };
    lines.push(name_line);
    lines.push(Line::from(""));

    // Keywords
    lines.push(Line::from(
        Span::styled(card.keywords_string(), Style::default().fg(app.theme.subtext0))
    ));
    lines.push(Line::from(""));

    // Meaning preview (truncated)
    let meaning = if drawn.reversed {
        card.reversed
    } else {
        card.upright
    };
    let preview: String = meaning.chars().take(80).collect::<String>() + "...";
    lines.push(Line::from(
        Span::styled(preview, Style::default().fg(app.theme.text))
    ));

    let para = Paragraph::new(Text::from(lines));
    f.render_widget(para, inner);
}
