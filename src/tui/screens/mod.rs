//! TUI Screen rendering

use ratatui::{
    style::Color,
    Frame,
};

use crate::cards::{ArcanaType, Suit};
use crate::tui::app::{App, AppTheme, Screen};

mod home;
mod spread_selection;
mod shuffle;
mod card_reveal;
mod reading_complete;
mod reading_notes;
mod card_browser;
mod card_detail;
mod daily_card;
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
        Screen::ReadingNotes => reading_notes::draw(f, app),
        Screen::CardBrowser => card_browser::draw(f, app),
        Screen::CardDetail => card_detail::draw(f, app),
        Screen::DailyCard => daily_card::draw(f, app),
        Screen::Help => help::draw(f, app),
        Screen::Journal => journal::draw(f, app),
        Screen::JournalDetail => journal_detail::draw(f, app),
        Screen::Quit => {}
        Screen::NewReading => {}
    }
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
