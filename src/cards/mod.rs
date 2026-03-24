pub mod major;
pub mod minor;

use std::fmt;

/// The type of arcana - Major or Minor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcanaType {
    Major,
    Minor,
}

impl fmt::Display for ArcanaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArcanaType::Major => write!(f, "Major Arcana"),
            ArcanaType::Minor => write!(f, "Minor Arcana"),
        }
    }
}

/// The four suits of the Minor Arcana
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Wands,
    Cups,
    Swords,
    Pentacles,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Wands => write!(f, "Wands"),
            Suit::Cups => write!(f, "Cups"),
            Suit::Swords => write!(f, "Swords"),
            Suit::Pentacles => write!(f, "Pentacles"),
        }
    }
}

impl Suit {
    /// Get the element associated with this suit
    pub fn element(&self) -> Element {
        match self {
            Suit::Wands => Element::Fire,
            Suit::Cups => Element::Water,
            Suit::Swords => Element::Air,
            Suit::Pentacles => Element::Earth,
        }
    }
}

/// The four classical elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Fire,
    Water,
    Air,
    Earth,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Fire => write!(f, "Fire"),
            Element::Water => write!(f, "Water"),
            Element::Air => write!(f, "Air"),
            Element::Earth => write!(f, "Earth"),
        }
    }
}

/// Yes/No/Maybe for quick card pulls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YesOrNo {
    Yes,
    No,
    Maybe,
}

impl fmt::Display for YesOrNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YesOrNo::Yes => write!(f, "Yes"),
            YesOrNo::No => write!(f, "No"),
            YesOrNo::Maybe => write!(f, "Maybe"),
        }
    }
}

/// A single tarot card with all its metadata
#[derive(Debug, Clone)]
pub struct Card {
    pub name: &'static str,
    pub arcana: ArcanaType,
    pub suit: Option<Suit>,
    pub number: u8,
    pub keywords: &'static [&'static str],
    pub upright: &'static str,
    pub reversed: &'static str,
    #[allow(dead_code)]
    pub description: &'static str,
    pub element: Option<Element>,
    pub astrology: Option<&'static str>,
    pub numerology: Option<&'static str>,
    pub yes_or_no: YesOrNo,
}

impl Card {
    /// Get the Roman numeral representation for Major Arcana
    pub fn roman_numeral(&self) -> Option<&'static str> {
        if self.arcana != ArcanaType::Major {
            return None;
        }

        const NUMERALS: &[&str] = &[
            "0", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII",
            "XIII", "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX", "XXI",
        ];

        NUMERALS.get(self.number as usize).copied()
    }

    /// Get the full display name including number/roman numeral
    pub fn display_name(&self) -> String {
        match self.arcana {
            ArcanaType::Major => {
                if let Some(numeral) = self.roman_numeral() {
                    format!("{} ({})", self.name, numeral)
                } else {
                    self.name.to_string()
                }
            }
            ArcanaType::Minor => {
                let number_name = match self.number {
                    1 => "Ace".to_string(),
                    11 => "Page".to_string(),
                    12 => "Knight".to_string(),
                    13 => "Queen".to_string(),
                    14 => "King".to_string(),
                    n => n.to_string(),
                };
                if let Some(suit) = self.suit {
                    format!("{} of {}", number_name, suit)
                } else {
                    self.name.to_string()
                }
            }
        }
    }

    /// Get the keywords as a comma-separated string
    pub fn keywords_string(&self) -> String {
        self.keywords.join(", ")
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// A card that has been drawn, with reversal status and position
#[derive(Debug, Clone)]
pub struct DrawnCard {
    pub card: &'static Card,
    pub reversed: bool,
    #[allow(dead_code)]
    pub position: Option<SpreadPosition>,
}

impl DrawnCard {
    /// Create a new drawn card
    pub fn new(card: &'static Card, reversed: bool) -> Self {
        Self {
            card,
            reversed,
            position: None,
        }
    }

    /// Get the meaning (upright or reversed based on orientation)
    pub fn meaning(&self) -> &'static str {
        if self.reversed {
            self.card.reversed
        } else {
            self.card.upright
        }
    }

    /// Get the keywords (same for both orientations, but could be customized)
    pub fn keywords(&self) -> String {
        self.card.keywords_string()
    }
}

/// Position in a spread layout
#[derive(Debug, Clone)]
pub struct SpreadPosition {
    pub name: &'static str,
    pub description: &'static str,
    #[allow(dead_code)]
    pub index: usize,
}

impl SpreadPosition {
    /// Create a new spread position
    #[allow(dead_code)]
    pub const fn new(name: &'static str, description: &'static str, index: usize) -> Self {
        Self {
            name,
            description,
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_numeral() {
        let fool = Card {
            name: "The Fool",
            arcana: ArcanaType::Major,
            suit: None,
            number: 0,
            keywords: &["beginnings", "innocence", "spontaneity"],
            upright: "test",
            reversed: "test",
            description: "test",
            element: None,
            astrology: None,
            numerology: None,
            yes_or_no: YesOrNo::Maybe,
        };
        assert_eq!(fool.roman_numeral(), Some("0"));

        let tower = Card {
            name: "The Tower",
            arcana: ArcanaType::Major,
            suit: None,
            number: 16,
            keywords: &["upheaval", "sudden change"],
            upright: "test",
            reversed: "test",
            description: "test",
            element: None,
            astrology: None,
            numerology: None,
            yes_or_no: YesOrNo::No,
        };
        assert_eq!(tower.roman_numeral(), Some("XVI"));
    }

    #[test]
    fn test_suit_element() {
        assert_eq!(Suit::Wands.element(), Element::Fire);
        assert_eq!(Suit::Cups.element(), Element::Water);
        assert_eq!(Suit::Swords.element(), Element::Air);
        assert_eq!(Suit::Pentacles.element(), Element::Earth);
    }
}
