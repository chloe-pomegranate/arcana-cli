//! Visual spread layout renderer

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    buffer::Buffer,
};

use crate::cards::DrawnCard;
use crate::spreads::SpreadLayout;

/// Render a visual spread layout
pub fn render_spread_layout(
    layout: SpreadLayout,
    cards: &[&DrawnCard],
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    match layout {
        SpreadLayout::Single => render_single(cards, area, buf, colors),
        SpreadLayout::Linear => render_linear(cards, area, buf, colors),
        SpreadLayout::Cross => render_cross(cards, area, buf, colors),
        SpreadLayout::CelticCross => render_celtic_cross(cards, area, buf, colors),
    }
}

/// Colors for layout rendering
pub struct LayoutColors {
    pub border: Color,
    pub card_border: Color,
    pub text: Color,
    pub number: Color,
}

fn render_single(
    cards: &[&DrawnCard],
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    if let Some(card) = cards.first() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.border));
        let inner = block.inner(area);
        block.render(area, buf);

        render_mini_card(card, 1, "The Card", inner, buf, colors);
    }
}

fn render_linear(
    cards: &[&DrawnCard],
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    let positions = vec!["Past", "Present", "Future"];
    let num_cards = cards.len().min(positions.len());

    let constraints: Vec<Constraint> = (0..num_cards)
        .map(|_| Constraint::Percentage((100 / num_cards) as u16))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(constraints)
        .split(area);

    for (i, (card, chunk)) in cards.iter().zip(chunks.iter()).enumerate() {
        let pos_name = positions.get(i).copied().unwrap_or("Card");
        render_mini_card(card, i + 1, pos_name, *chunk, buf, colors);
    }
}

fn render_cross(
    cards: &[&DrawnCard],
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    let _positions = vec!["Present", "Challenge", "Past", "Future", "Potential"];
    
    // Center: Present (1) with Challenge (2) crossing it
    // Left: Past (3)
    // Right: Future (4)
    // Top: Potential (5)
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(area);

    // Top row - Potential
    if let Some(card) = cards.get(4) {
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(main_chunks[0]);
        render_mini_card(card, 5, "Potential", top_chunks[1], buf, colors);
    }

    // Middle row - Past, Present/Challenge, Future
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(main_chunks[1]);

    if let Some(card) = cards.get(2) {
        render_mini_card(card, 3, "Past", middle_chunks[0], buf, colors);
    }
    if let Some(card) = cards.get(0) {
        render_mini_card(card, 1, "Present", middle_chunks[1], buf, colors);
    }
    if let Some(card) = cards.get(3) {
        render_mini_card(card, 4, "Future", middle_chunks[2], buf, colors);
    }

    // Bottom - just show Challenge info
    if let Some(card) = cards.get(1) {
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(main_chunks[2]);
        
        // Render challenge card
        let block = Block::default()
            .title("2. Challenge (crosses Present)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.card_border));
        let inner = block.inner(bottom_chunks[1]);
        block.render(bottom_chunks[1], buf);
        let text = format!("{} {}", 
            if card.reversed { "⟳" } else { "" },
            card.card.name
        );
        
        let para = Paragraph::new(Span::styled(
            text,
            Style::default().fg(colors.text)
        ));
        para.render(inner, buf);
    }
}

fn render_celtic_cross(
    cards: &[&DrawnCard],
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    // Celtic Cross layout:
    //         [5]
    //     [4][1][2][6]
    //         [3]
    // 
    // Staff on the right:
    // [10]
    // [9]
    // [8]
    // [7]

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);

    // Left side - the cross
    let cross_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(main_chunks[0]);

    // Row 2 of cross - [4][1][2][6]
    let cross_row2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(cross_chunks[1]);

    // Row 1 - Crown [5]
    if let Some(card) = cards.get(4) {
        render_tiny_card(card, 5, "Crown", cross_chunks[0], buf, colors);
    }

    // Row 2 - Recent Past [4], Present [1], Challenge [2], Near Future [6]
    if let Some(card) = cards.get(3) {
        render_tiny_card(card, 4, "Recent", cross_row2[0], buf, colors);
    }
    if let Some(card) = cards.get(0) {
        render_tiny_card(card, 1, "Present", cross_row2[1], buf, colors);
    }
    if let Some(card) = cards.get(1) {
        render_tiny_card(card, 2, "Challenge", cross_row2[2], buf, colors);
    }
    if let Some(card) = cards.get(5) {
        render_tiny_card(card, 6, "Near", cross_row2[3], buf, colors);
    }

    // Row 3 - Foundation [3]
    if let Some(card) = cards.get(2) {
        render_tiny_card(card, 3, "Foundation", cross_chunks[2], buf, colors);
    }

    // Right side - the staff [10], [9], [8], [7]
    let staff_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(main_chunks[1]);

    if let Some(card) = cards.get(9) {
        render_tiny_card(card, 10, "Outcome", staff_chunks[0], buf, colors);
    }
    if let Some(card) = cards.get(8) {
        render_tiny_card(card, 9, "Hopes", staff_chunks[1], buf, colors);
    }
    if let Some(card) = cards.get(7) {
        render_tiny_card(card, 8, "Env", staff_chunks[2], buf, colors);
    }
    if let Some(card) = cards.get(6) {
        render_tiny_card(card, 7, "Self", staff_chunks[3], buf, colors);
    }
}

fn render_mini_card(
    card: &DrawnCard,
    num: usize,
    position: &str,
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    let block = Block::default()
        .title(format!("{}. {}", num, position))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.card_border))
        .title_style(Style::default().fg(colors.number));

    let inner = block.inner(area);
    block.render(area, buf);

    let reversed_text = if card.reversed { " ⟳" } else { "" };
    let text = format!("{}{}", card.card.name, reversed_text);

    let lines = vec![
        Line::from(Span::styled(
            text,
            Style::default().fg(colors.text).add_modifier(Modifier::BOLD)
        )),
        Line::from(Span::styled(
            card.card.keywords[..card.card.keywords.len().min(2)].join(", "),
            Style::default().fg(colors.text)
        )),
    ];

    let para = Paragraph::new(Text::from(lines));
    para.render(inner, buf);
}

fn render_tiny_card(
    card: &DrawnCard,
    num: usize,
    position: &str,
    area: Rect,
    buf: &mut Buffer,
    colors: &LayoutColors,
) {
    let block = Block::default()
        .title(format!("{}. {}", num, position))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.card_border));

    let inner = block.inner(area);
    block.render(area, buf);

    let reversed_text = if card.reversed { "⟳ " } else { "" };
    let text = format!("{}{}", reversed_text, card.card.name);

    let para = Paragraph::new(Span::styled(
        text,
        Style::default().fg(colors.text)
    ));
    para.render(inner, buf);
}

/// A widget that renders a spread layout
pub struct SpreadLayoutWidget<'a> {
    layout: SpreadLayout,
    cards: &'a [&'a DrawnCard],
    colors: LayoutColors,
}

impl<'a> SpreadLayoutWidget<'a> {
    pub fn new(layout: SpreadLayout, cards: &'a [&'a DrawnCard], colors: LayoutColors) -> Self {
        Self { layout, cards, colors }
    }
}

impl<'a> Widget for SpreadLayoutWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_spread_layout(self.layout, self.cards, area, buf, &self.colors);
    }
}
