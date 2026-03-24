//! TUI Screen rendering

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders,
    },
    Frame,
};

use crate::cards::{ArcanaType, Suit};
use crate::tui::app::{App, AppTheme, CardFilter, Screen};

mod home;
mod spread_selection;
mod shuffle;
mod card_reveal;
mod reading_complete;
mod card_browser;
mod card_detail;
mod help;
mod journal;
mod journal_detail;

/// Main draw function that routes to the appropriate screen
pub fn draw(f: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::Home => home::draw(f, app),
        Screen::SpreadSelection => spread_selection::draw(f, app),
        Screen::ShuffleAnimation => shuffle::draw(f, app),
        Screen::CardReveal => card_reveal::draw(f, app),
        Screen::ReadingComplete => reading_complete::draw(f, app),
        Screen::CardBrowser => card_browser::draw(f, app),
        Screen::CardDetail => card_detail::draw(f, app),
        Screen::Help => help::draw(f, app),
        Screen::Journal => journal::draw(f, app),
        Screen::JournalDetail => journal_detail::draw(f, app),
        Screen::Quit => {}
        Screen::NewReading => {}
    }
}

/// Create a styled block with the app theme
#[allow(dead_code)]
fn styled_block<'a>(title: &'a str, theme: &'a AppTheme) -> Block<'a> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.lavender))
        .title_style(Style::default().fg(theme.mauve).add_modifier(ratatui::style::Modifier::BOLD))
}

/// Create a centered rect for popups
#[allow(dead_code)]
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Get color for a card based on its suit or arcana
fn card_color(card: &crate::cards::Card, theme: &AppTheme) -> Color {
    match card.arcana {
        ArcanaType::Major => theme.mauve,
        ArcanaType::Minor => {
            if let Some(suit) = card.suit {
                theme.suit_color(suit)
            } else {
                theme.lavender
            }
        }
    }
}

/// Get suit symbol
fn suit_symbol(suit: Suit) -> &'static str {
    match suit {
        Suit::Wands => "🔥",
        Suit::Cups => "💧",
        Suit::Swords => "⚔️",
        Suit::Pentacles => "🌿",
    }
}

/// Get arcana symbol
fn arcana_symbol(arcana: ArcanaType) -> &'static str {
    match arcana {
        ArcanaType::Major => "✦",
        ArcanaType::Minor => "○",
    }
}

/// Format card filter for display
#[allow(dead_code)]
fn filter_name(filter: &CardFilter) -> String {
    match filter {
        CardFilter::All => "All Cards".to_string(),
        CardFilter::Major => "Major Arcana".to_string(),
        CardFilter::Suit(suit) => format!("{:?}", suit),
    }
}
