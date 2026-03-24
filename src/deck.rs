use crate::cards::{minor::MINOR_ARCANA, major::MAJOR_ARCANA, Card, DrawnCard};
use rand::{seq::SliceRandom, thread_rng, Rng};

/// A complete 78-card tarot deck with shuffle, draw, and reversal capabilities
#[derive(Debug, Clone)]
pub struct Deck {
    /// Indices into the static ALL_CARDS array, in current order
    cards: Vec<usize>,
    /// Position of the next card to draw
    position: usize,
    /// Whether to include reversals in draws
    allow_reversals: bool,
    /// Probability of reversal (0.0 to 1.0)
    reversal_probability: f64,
}



impl Deck {
    /// Probability of reversal (default: 0.15 = 15%)
    pub const DEFAULT_REVERSAL_PROBABILITY: f64 = 0.15;

    /// Create a new deck with all 78 cards
    pub fn new() -> Self {
        let mut deck = Self {
            cards: (0..78).collect(),
            position: 0,
            allow_reversals: true,
            reversal_probability: Self::DEFAULT_REVERSAL_PROBABILITY,
        };
        deck.shuffle();
        deck
    }

    /// Create a new deck without shuffling (for testing)
    #[allow(dead_code)]
    pub fn new_ordered() -> Self {
        Self {
            cards: (0..78).collect(),
            position: 0,
            allow_reversals: true,
            reversal_probability: Self::DEFAULT_REVERSAL_PROBABILITY,
        }
    }

    /// Create a Major Arcana only deck (22 cards)
    #[allow(dead_code)]
    pub fn new_major_arcana_only() -> Self {
        let mut deck = Self {
            cards: (0..22).collect(),
            position: 0,
            allow_reversals: true,
            reversal_probability: Self::DEFAULT_REVERSAL_PROBABILITY,
        };
        deck.shuffle();
        deck
    }

    /// Get a card by its index in the combined deck
    pub fn get_card(index: usize) -> Option<&'static Card> {
        if index < 22 {
            MAJOR_ARCANA.get(index)
        } else if index < 78 {
            MINOR_ARCANA.get(index - 22)
        } else {
            None
        }
    }



    /// Iterate over all cards
    pub fn iter_all_cards() -> impl Iterator<Item = &'static Card> {
        MAJOR_ARCANA.iter().chain(MINOR_ARCANA.iter())
    }

    /// Get the total number of cards
    #[allow(dead_code)]
    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }

    /// Get the number of cards remaining in the deck
    pub fn remaining(&self) -> usize {
        self.cards.len().saturating_sub(self.position)
    }

    /// Get the number of cards already drawn
    #[allow(dead_code)]
    pub fn drawn(&self) -> usize {
        self.position
    }

    /// Check if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    /// Enable or disable reversals
    pub fn set_allow_reversals(&mut self, allow: bool) {
        self.allow_reversals = allow;
    }

    /// Set the reversal probability (0.0 to 1.0)
    #[allow(dead_code)]
    pub fn set_reversal_probability(&mut self, probability: f64) {
        self.reversal_probability = probability.clamp(0.0, 1.0);
    }

    /// Shuffle the deck, resetting the position to 0
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        self.position = 0;
    }

    /// Perform a ritual shuffle (3 shuffles with cuts)
    #[allow(dead_code)]
    pub fn ritual_shuffle(&mut self) {
        for _ in 0..3 {
            self.shuffle();
            self.cut();
        }
    }

    /// Cut the deck at a random position
    #[allow(dead_code)]
    pub fn cut(&mut self) {
        let mut rng = thread_rng();
        let cut_point = rng.gen_range(0..self.cards.len());
        self.cards.rotate_left(cut_point);
    }

    /// Determine if a card should be reversed
    fn should_reverse(&self) -> bool {
        if !self.allow_reversals {
            return false;
        }
        let mut rng = thread_rng();
        rng.gen_bool(self.reversal_probability)
    }

    /// Draw a single card from the deck
    pub fn draw(&mut self) -> Option<DrawnCard> {
        if self.is_empty() {
            return None;
        }

        let card_index = self.cards[self.position];
        self.position += 1;

        let card = Self::get_card(card_index)?;
        let reversed = self.should_reverse();

        Some(DrawnCard::new(card, reversed))
    }

    /// Draw multiple cards from the deck
    pub fn draw_many(&mut self, count: usize) -> Vec<DrawnCard> {
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.draw() {
                drawn.push(card);
            } else {
                break;
            }
        }
        drawn
    }

    /// Peek at the next card without drawing it
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&'static Card> {
        if self.is_empty() {
            return None;
        }
        Self::get_card(self.cards[self.position])
    }

    /// Reset the deck position without reshuffling
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Reset and shuffle the deck
    pub fn reset_and_shuffle(&mut self) {
        self.reset();
        self.shuffle();
    }

    /// Search for cards by name (case-insensitive partial match)
    pub fn search_by_name(query: &str) -> Vec<&'static Card> {
        let query_lower = query.to_lowercase();
        Self::iter_all_cards()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get a card by exact name (case-insensitive)
    pub fn find_by_name(name: &str) -> Option<&'static Card> {
        let name_lower = name.to_lowercase();
        Self::iter_all_cards().find(|c| c.name.to_lowercase() == name_lower)
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for working with cards
pub mod utils {
    use super::*;
    use crate::cards::{ArcanaType, Suit};

    /// Get all Major Arcana cards
    pub fn major_arcana() -> &'static [Card] {
        MAJOR_ARCANA
    }

    /// Get all Minor Arcana cards
    #[allow(dead_code)]
    pub fn minor_arcana() -> &'static [Card] {
        MINOR_ARCANA
    }

    /// Get cards by arcana type
    #[allow(dead_code)]
    pub fn by_arcana_type(arcana: ArcanaType) -> impl Iterator<Item = &'static Card> {
        Deck::iter_all_cards().filter(move |c| c.arcana == arcana)
    }

    /// Get cards by suit
    pub fn by_suit(suit: Suit) -> impl Iterator<Item = &'static Card> {
        Deck::iter_all_cards().filter(move |c| c.suit == Some(suit))
    }

    /// Format a card for display in CLI
    #[allow(dead_code)]
    pub fn format_card(card: &Card, reversed: bool) -> String {
        let orientation = if reversed { " (Reversed)" } else { "" };
        format!("{}{}", card.display_name(), orientation)
    }

    /// Format a drawn card for display
    #[allow(dead_code)]
    pub fn format_drawn_card(drawn: &DrawnCard) -> String {
        format_card(drawn.card, drawn.reversed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{ArcanaType, Suit};

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new_ordered();
        assert_eq!(deck.total_cards(), 78);
        assert_eq!(deck.remaining(), 78);
        assert_eq!(deck.drawn(), 0);
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck::new_ordered();
        let drawn = deck.draw().unwrap();
        assert_eq!(deck.remaining(), 77);
        assert_eq!(deck.drawn(), 1);
        // First card in ordered deck is The Fool (index 0)
        assert_eq!(drawn.card.name, "The Fool");
    }

    #[test]
    fn test_draw_many() {
        let mut deck = Deck::new_ordered();
        let drawn = deck.draw_many(5);
        assert_eq!(drawn.len(), 5);
        assert_eq!(deck.remaining(), 73);
    }

    #[test]
    fn test_deck_empty() {
        let mut deck = Deck::new_ordered();
        // Draw all cards
        for _ in 0..78 {
            assert!(deck.draw().is_some());
        }
        assert!(deck.is_empty());
        assert!(deck.draw().is_none());
    }

    #[test]
    fn test_reset() {
        let mut deck = Deck::new_ordered();
        deck.draw_many(10);
        assert_eq!(deck.remaining(), 68);
        deck.reset();
        assert_eq!(deck.remaining(), 78);
        assert_eq!(deck.drawn(), 0);
    }

    #[test]
    fn test_major_arcana_only() {
        let deck = Deck::new_major_arcana_only();
        assert_eq!(deck.total_cards(), 22);
    }

    #[test]
    fn test_get_card() {
        assert_eq!(Deck::get_card(0).unwrap().name, "The Fool");
        assert_eq!(Deck::get_card(21).unwrap().name, "The World");
        assert_eq!(Deck::get_card(22).unwrap().name, "Ace of Wands");
        assert_eq!(Deck::get_card(77).unwrap().name, "King of Pentacles");
        assert!(Deck::get_card(78).is_none());
    }

    #[test]
    fn test_search_by_name() {
        let results = Deck::search_by_name("fool");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "The Fool");

        let results = Deck::search_by_name("king");
        assert_eq!(results.len(), 4); // King of each suit
    }

    #[test]
    fn test_find_by_name() {
        assert_eq!(Deck::find_by_name("The Fool").unwrap().number, 0);
        assert_eq!(Deck::find_by_name("the fool").unwrap().number, 0);
        assert_eq!(
            Deck::find_by_name("King of Cups").unwrap().suit,
            Some(Suit::Cups)
        );
        assert!(Deck::find_by_name("Not a Card").is_none());
    }

    #[test]
    fn test_shuffle() {
        let mut deck = Deck::new_ordered();
        let before: Vec<usize> = deck.cards.clone();
        deck.shuffle();
        // Very unlikely to be in the same order after shuffling
        // (though technically possible, probability is 1/78!)
        assert_ne!(deck.cards, before);
        assert_eq!(deck.position, 0);
    }

    #[test]
    fn test_reversal_probability() {
        let mut deck = Deck::new();
        deck.set_reversal_probability(1.0);
        
        // Draw many cards, all should be reversed
        let drawn = deck.draw_many(10);
        assert!(drawn.iter().all(|c| c.reversed));

        let mut deck = Deck::new();
        deck.set_reversal_probability(0.0);
        
        // Draw many cards, none should be reversed
        let drawn = deck.draw_many(10);
        assert!(drawn.iter().all(|c| !c.reversed));
    }

    #[test]
    fn test_utils() {
        assert_eq!(utils::major_arcana().len(), 22);
        assert_eq!(utils::minor_arcana().len(), 56);
        assert_eq!(utils::by_arcana_type(ArcanaType::Major).count(), 22);
        assert_eq!(utils::by_arcana_type(ArcanaType::Minor).count(), 56);
        assert_eq!(utils::by_suit(Suit::Wands).count(), 14);
    }
}
