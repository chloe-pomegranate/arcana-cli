//! Tarot spread definitions and reading orchestration

use crate::cards::{DrawnCard, SpreadPosition};

/// A tarot spread with defined positions
#[derive(Debug, Clone)]
pub struct Spread {
    pub name: &'static str,
    pub description: &'static str,
    pub positions: &'static [SpreadPosition],
    pub layout: SpreadLayout,
}

/// Visual layout type for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpreadLayout {
    /// Single card, no special layout
    Single,
    /// Linear layout (horizontal)
    Linear,
    /// Cross formation
    Cross,
    /// Celtic Cross (complex 10-card layout)
    CelticCross,
}

impl Spread {
    /// Get the number of cards in this spread
    pub fn card_count(&self) -> usize {
        self.positions.len()
    }

    /// Get a position by index
    pub fn get_position(&self, index: usize) -> Option<&SpreadPosition> {
        self.positions.get(index)
    }
}

/// Single card spread - quick insight
pub const SINGLE: Spread = Spread {
    name: "Single Card",
    description: "A quick insight or answer to a specific question",
    positions: &[SpreadPosition {
        name: "The Card",
        description: "The energy, message, or answer you need right now",
    }],
    layout: SpreadLayout::Single,
};

/// Three-card spread - past, present, future
pub const THREE_CARD: Spread = Spread {
    name: "Three-Card Spread",
    description: "A journey through time: what was, what is, and what will be",
    positions: &[
        SpreadPosition {
            name: "Past",
            description: "What has led to this moment. The foundation and influences that shaped your current situation.",
        },
        SpreadPosition {
            name: "Present",
            description: "The current situation. Where you stand right now and the energy surrounding you.",
        },
        SpreadPosition {
            name: "Future",
            description: "Where things are heading. The likely outcome if current energies continue.",
        },
    ],
    layout: SpreadLayout::Linear,
};

/// Three-card spread alternative: situation, action, outcome
pub const SITUATION_ACTION_OUTCOME: Spread = Spread {
    name: "Situation-Action-Outcome",
    description: "Understanding your situation, what to do, and the result",
    positions: &[
        SpreadPosition {
            name: "Situation",
            description: "The current circumstances and context of your question.",
        },
        SpreadPosition {
            name: "Action",
            description: "The recommended approach or action to take.",
        },
        SpreadPosition {
            name: "Outcome",
            description: "The likely result if you take the suggested action.",
        },
    ],
    layout: SpreadLayout::Linear,
};

/// Three-card spread alternative: mind, body, spirit
pub const MIND_BODY_SPIRIT: Spread = Spread {
    name: "Mind-Body-Spirit",
    description: "A holistic view of your current state",
    positions: &[
        SpreadPosition {
            name: "Mind",
            description: "Your mental state, thoughts, and intellectual perspective.",
        },
        SpreadPosition {
            name: "Body",
            description: "Your physical state, health, and material circumstances.",
        },
        SpreadPosition {
            name: "Spirit",
            description: "Your spiritual state, intuition, and higher guidance.",
        },
    ],
    layout: SpreadLayout::Linear,
};

/// Five-card cross spread
pub const FIVE_CARD_CROSS: Spread = Spread {
    name: "Five-Card Cross",
    description: "A comprehensive view of your situation with challenge and potential",
    positions: &[
        SpreadPosition {
            name: "Present",
            description: "The heart of the matter. Your current situation and central energy.",
        },
        SpreadPosition {
            name: "Challenge",
            description: "What's working against you. Obstacles, difficulties, or opposition.",
        },
        SpreadPosition {
            name: "Past",
            description: "Foundation of the situation. What has led to where you are now.",
        },
        SpreadPosition {
            name: "Future",
            description: "What's coming next. Near-term developments and energies approaching.",
        },
        SpreadPosition {
            name: "Potential",
            description: "The best possible outcome. What can be achieved with awareness and effort.",
        },
    ],
    layout: SpreadLayout::Cross,
};

/// Celtic Cross - the classic 10-card spread
pub const CELTIC_CROSS: Spread = Spread {
    name: "Celtic Cross",
    description: "The classic ten-card spread for deep insight into any situation",
    positions: &[
        SpreadPosition {
            name: "Present",
            description: "The current situation. The heart of the matter and your central energy right now.",
        },
        SpreadPosition {
            name: "Challenge",
            description: "The immediate obstacle or challenge. What crosses you, creates difficulty, or needs attention.",
        },
        SpreadPosition {
            name: "Foundation",
            description: "The root cause. What underlies the situation, often unconscious or from the distant past.",
        },
        SpreadPosition {
            name: "Recent Past",
            description: "What's just happened. Recent events that are still influencing the present.",
        },
        SpreadPosition {
            name: "Crown",
            description: "The best that can be achieved. Your highest potential or goal in this situation.",
        },
        SpreadPosition {
            name: "Near Future",
            description: "What's coming soon. Events and energies approaching in the short term.",
        },
        SpreadPosition {
            name: "Self",
            description: "Your attitude and approach. How you see yourself and your role in this situation.",
        },
        SpreadPosition {
            name: "Environment",
            description: "External influences. How others see you and the energies of people around you.",
        },
        SpreadPosition {
            name: "Hopes & Fears",
            description: "What you hope for or fear. Your expectations, anxieties, and desires regarding the outcome.",
        },
        SpreadPosition {
            name: "Outcome",
            description: "The likely result. Where the current path leads if energies continue unchanged.",
        },
    ],
    layout: SpreadLayout::CelticCross,
};

/// Get a spread by name
pub fn get_by_name(name: &str) -> Option<&'static Spread> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "single" | "one" => Some(&SINGLE),
        "three" | "3" | "past-present-future" => Some(&THREE_CARD),
        "situation" | "situation-action-outcome" => Some(&SITUATION_ACTION_OUTCOME),
        "mind-body-spirit" | "holistic" => Some(&MIND_BODY_SPIRIT),
        "five" | "5" | "cross" => Some(&FIVE_CARD_CROSS),
        "celtic" | "celtic-cross" | "10" => Some(&CELTIC_CROSS),
        _ => None,
    }
}

/// Get all available spreads
pub fn all_spreads() -> &'static [&'static Spread] {
    &[
        &SINGLE,
        &THREE_CARD,
        &SITUATION_ACTION_OUTCOME,
        &MIND_BODY_SPIRIT,
        &FIVE_CARD_CROSS,
        &CELTIC_CROSS,
    ]
}

/// A completed reading with drawn cards matched to positions
#[derive(Debug, Clone)]
pub struct Reading {
    pub spread: &'static Spread,
    pub drawn: Vec<DrawnCard>,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub notes: Option<String>,
}

impl Reading {
    /// Create a new reading from a spread and drawn cards
    pub fn new(spread: &'static Spread, drawn: Vec<DrawnCard>) -> Self {
        Self {
            spread,
            drawn,
            timestamp: chrono::Local::now(),
            notes: None,
        }
    }

    /// Get the number of cards in this reading
    pub fn card_count(&self) -> usize {
        self.drawn.len()
    }

    /// Get a specific card and its position
    pub fn get_card_at(&self, index: usize) -> Option<(&DrawnCard, &SpreadPosition)> {
        self.drawn.get(index).zip(self.spread.get_position(index))
    }

    /// Format the reading as markdown for journaling
    pub fn to_markdown(&self) -> String {
        use std::fmt::Write;
        
        let mut output = String::new();
        
        writeln!(
            &mut output,
            "# Tarot Reading — {}",
            self.timestamp.format("%Y-%m-%d %H:%M")
        )
        .unwrap();
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "## Spread: {}", self.spread.name).unwrap();
        writeln!(&mut output).unwrap();
        
        if let Some(notes) = &self.notes {
            writeln!(&mut output, "## Notes").unwrap();
            writeln!(&mut output, "{}", notes).unwrap();
            writeln!(&mut output).unwrap();
        }
        
        for (i, (card, position)) in self
            .drawn
            .iter()
            .zip(self.spread.positions.iter())
            .enumerate()
        {
            writeln!(&mut output, "### {}. {} — {}", i + 1, position.name, card.card.name).unwrap();
            if card.reversed {
                writeln!(&mut output, "**Reversed**").unwrap();
            } else {
                writeln!(&mut output, "**Upright**").unwrap();
            }
            writeln!(&mut output).unwrap();
            writeln!(&mut output, "Keywords: {}", card.keywords()).unwrap();
            writeln!(&mut output).unwrap();
            writeln!(&mut output, "{}", card.meaning()).unwrap();
            writeln!(&mut output).unwrap();
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread_counts() {
        assert_eq!(SINGLE.card_count(), 1);
        assert_eq!(THREE_CARD.card_count(), 3);
        assert_eq!(FIVE_CARD_CROSS.card_count(), 5);
        assert_eq!(CELTIC_CROSS.card_count(), 10);
    }

    #[test]
    fn test_get_by_name() {
        assert_eq!(get_by_name("single").unwrap().name, "Single Card");
        assert_eq!(get_by_name("three").unwrap().name, "Three-Card Spread");
        assert_eq!(get_by_name("celtic").unwrap().name, "Celtic Cross");
        assert_eq!(get_by_name("5").unwrap().name, "Five-Card Cross");
        assert!(get_by_name("invalid").is_none());
    }

    #[test]
    fn test_all_spreads() {
        assert_eq!(all_spreads().len(), 6);
    }
}
