//! Catppuccin Mocha color theme for terminal output

use crossterm::style::Color;

/// Catppuccin Mocha color palette
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub subtext0: Color,  // #a6adc8 — tertiary text
    pub lavender: Color,  // #b4befe — highlights, selected items
    pub mauve: Color,     // #cba6f7 — Major Arcana accent
    pub pink: Color,      // #f5c2e7 — Cups accent
    pub red: Color,       // #f38ba8 — Swords accent
    pub peach: Color,     // #fab387 — Wands accent
    pub green: Color,     // #a6e3a1 — Pentacles accent
    pub yellow: Color,    // #f9e2af — reversed card indicator
    pub sky: Color,       // #89dceb — card number/title
}

impl Theme {
    /// The default Catppuccin Mocha theme
    pub const fn catppuccin_mocha() -> Self {
        Self {
            subtext0: Color::Rgb { r: 166, g: 173, b: 200 },
            lavender: Color::Rgb { r: 180, g: 190, b: 254 },
            mauve: Color::Rgb { r: 203, g: 166, b: 247 },
            pink: Color::Rgb { r: 245, g: 194, b: 231 },
            red: Color::Rgb { r: 243, g: 139, b: 168 },
            peach: Color::Rgb { r: 250, g: 179, b: 135 },
            green: Color::Rgb { r: 166, g: 227, b: 161 },
            yellow: Color::Rgb { r: 249, g: 226, b: 175 },
            sky: Color::Rgb { r: 137, g: 220, b: 235 },
        }
    }

    /// Get color for a suit
    pub fn suit_color(&self, suit: crate::cards::Suit) -> Color {
        use crate::cards::Suit;
        match suit {
            Suit::Wands => self.peach,
            Suit::Cups => self.pink,
            Suit::Swords => self.red,
            Suit::Pentacles => self.green,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

/// Global theme instance
pub fn theme() -> Theme {
    Theme::default()
}

/// Trait for styling strings with theme colors
pub trait ColorStyle {
    fn with_color(self, color: Color) -> String;
    fn mauve(self) -> String;
    fn lavender(self) -> String;
    fn pink(self) -> String;
    fn peach(self) -> String;
    fn sky(self) -> String;
    fn subtext(self) -> String;
}

impl<T: AsRef<str>> ColorStyle for T {
    fn with_color(self, color: Color) -> String {
        use crossterm::style::Stylize;
        self.as_ref().with(color).to_string()
    }

    fn mauve(self) -> String {
        self.with_color(theme().mauve)
    }

    fn lavender(self) -> String {
        self.with_color(theme().lavender)
    }

    fn pink(self) -> String {
        self.with_color(theme().pink)
    }

    fn peach(self) -> String {
        self.with_color(theme().peach)
    }

    fn sky(self) -> String {
        self.with_color(theme().sky)
    }

    fn subtext(self) -> String {
        self.with_color(theme().subtext0)
    }
}

/// Styled text helpers
pub mod style {
    use super::*;
    use crossterm::style::Stylize;

    /// Style a card name with appropriate colors
    pub fn card_name(card: &crate::cards::Card) -> String {
        let t = theme();
        
        match card.arcana {
            crate::cards::ArcanaType::Major => {
                card.display_name().with(t.mauve).bold().to_string()
            }
            crate::cards::ArcanaType::Minor => {
                if let Some(suit) = card.suit {
                    card.display_name().with(t.suit_color(suit)).bold().to_string()
                } else {
                    card.display_name().with(t.lavender).bold().to_string()
                }
            }
        }
    }

    /// Style reversed indicator
    pub fn reversed() -> String {
        let t = theme();
        "⟳ REVERSED".with(t.yellow).bold().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let t = Theme::catppuccin_mocha();
        assert!(matches!(t.mauve, Color::Rgb { .. }));
    }

    #[test]
    fn test_suit_colors() {
        use crate::cards::Suit;
        let t = Theme::catppuccin_mocha();
        
        assert_eq!(t.suit_color(Suit::Wands), t.peach);
        assert_eq!(t.suit_color(Suit::Cups), t.pink);
        assert_eq!(t.suit_color(Suit::Swords), t.red);
        assert_eq!(t.suit_color(Suit::Pentacles), t.green);
    }

    #[test]
    fn test_color_style() {
        use super::ColorStyle;
        let styled = "test".mauve();
        assert!(styled.contains("test"));
    }
}
