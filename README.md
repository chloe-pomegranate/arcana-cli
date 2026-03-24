# 🔮 Arcana

> A beautiful, feature-rich terminal tarot application built in Rust

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

Arcana is a comprehensive tarot reading application that works beautifully as both a quick CLI tool and an immersive TUI (Terminal User Interface) experience. Complete with all 78 cards, multiple spread types, card reversals, and everything you need to read your tarot!

## ✨ Features

### 🎴 Complete Tarot Deck
- **78 cards** with rich, detailed meanings
- **22 Major Arcana** - from The Fool to The World
- **56 Minor Arcana** - 14 cards in each of the 4 suits
  - 🔥 **Wands** (Fire) - Action, creativity, passion
  - 💧 **Cups** (Water) - Emotions, relationships, intuition
  - ⚔️ **Swords** (Air) - Intellect, conflict, truth
  - 🌿 **Pentacles** (Earth) - Material world, career, health

### 🎯 Six Spread Types
| Spread | Cards | Purpose |
|--------|-------|---------|
| **Single Card** | 1 | Quick insight or daily guidance |
| **Three-Card** | 3 | Past / Present / Future |
| **Situation-Action-Outcome** | 3 | Problem-solving guidance |
| **Mind-Body-Spirit** | 3 | Holistic wellness view |
| **Five-Card Cross** | 5 | Situation analysis with challenge and potential |
| **Celtic Cross** | 10 | Deep, comprehensive insight |

### 🖥️ Dual Interface

#### Interactive TUI (Default)
- **Beautiful Catppuccin Mocha-inspired theme** throughout
- **Animated shuffle** with visual progress
- **Card-by-card reveal** - unveil each card with intention
- **Visual spread layouts** - see cards arranged in their positions
- **ASCII art** for Major Arcana cards
- **Card browser** with filtering by Major Arcana or suit
- **Journal system** - save and browse past readings

#### CLI Mode
- Quick lookups for any card
- Draw readings directly from command line
- List cards with filters
- Colored terminal output
- Pipe-friendly for scripting

### 🎨 Rich Card Data
Each card includes:
- 3-5 descriptive keywords
- Detailed upright meaning
- Detailed reversed meaning
- Full paragraph description
- Element association
- Astrological correspondence
- Numerology
- Yes/No/Maybe indicator

### 📓 Journal System
- Save readings to `~/.arcana/journal/`
- Markdown format for easy reading
- Browse past readings in TUI
- Automatic timestamps

## 📦 Installation

```bash
cargo build --release
```

The binary will be at `target/release/arcana`. You can copy it to your PATH:
```bash
cp target/release/arcana ~/.local/bin/
```

### Requirements
- Rust 1.70+ (for building from source)
- Terminal with Unicode and 256-color support

## 🚀 Usage

### Launch the TUI
```bash
arcana              # Launch interactive TUI
```

### CLI Commands

#### View a specific card
```bash
arcana card "The Fool"
arcana card "The Magician"
arcana card "Ace of Wands"
arcana card "Queen of Cups"
```

#### List cards
```bash
arcana list                    # All 78 cards
arcana list --major            # Major Arcana only
arcana list --suit wands       # Wands suit only
arcana list --suit cups        # Cups suit only
arcana list --long             # Detailed view with meanings
```

#### Draw a reading
```bash
# Single card (default)
arcana read

# Three-card spread
arcana read --spread three

# Celtic Cross (10 cards)
arcana read --spread celtic

# Five-card cross
arcana read --spread five

# Situation-Action-Outcome
arcana read --spread situation

# Mind-Body-Spirit
arcana read --spread holistic

# Disable reversals (all cards upright)
arcana read --no-reversals
```

#### View available spreads
```bash
arcana spreads
```

## 🎮 TUI Navigation

### Global Keys
| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` | Select / Confirm |
| `Tab` | Cycle filters |
| `q` / `Esc` | Quit or go back |

### During Readings
| Key | Action |
|-----|--------|
| `Space` / `→` | Reveal next card |
| `s` | Save reading to journal (after complete) |

## 🗂️ Project Structure

```
arcana/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── ascii/               # ASCII art for Major Arcana
│   │   └── mod.rs           # 22 card illustrations
│   ├── cards/               # Card definitions
│   │   ├── mod.rs           # Core types and structures
│   │   ├── major.rs         # 22 Major Arcana definitions
│   │   └── minor.rs         # 56 Minor Arcana definitions
│   ├── deck.rs              # Deck operations (shuffle, draw)
│   ├── journal.rs           # Journal save/load system
│   ├── spreads/             # Spread definitions
│   │   └── mod.rs           # 6 spread types + Reading struct
│   ├── tui/                 # Terminal UI
│   │   ├── mod.rs
│   │   ├── app.rs           # App state and event loop
│   │   ├── screens/         # All TUI screens
│   │   │   ├── home.rs
│   │   │   ├── spread_selection.rs
│   │   │   ├── shuffle.rs
│   │   │   ├── card_reveal.rs
│   │   │   ├── reading_complete.rs
│   │   │   ├── card_browser.rs
│   │   │   ├── card_detail.rs
│   │   │   ├── help.rs
│   │   │   └── journal.rs
│   │   └── widgets/         # Reusable widgets
│   │       ├── mod.rs
│   │       └── spread_layout.rs
│   └── ui/                  # Theme and styling
│       ├── mod.rs
│       └── theme.rs         # Catppuccin Mocha colors
├── Cargo.toml
└── README.md
```

## 🧪 Development

### Running tests
```bash
cargo test
```

### Running in development mode
```bash
cargo run
```

### Building release
```bash
cargo build --release
```

## 📝 Card Data

- **Keywords** - Quick reference for interpretation
- **Upright meaning** - The positive, direct interpretation
- **Reversed meaning** - The blocked, inverted, or shadow interpretation
- **Full description** - Deeper narrative context
- **Astrology** - Planetary and zodiac associations
- **Numerology** - Number significance
- **Yes / No** - Answer of yes / no / maybe per-card

## 📊 Stats

- **78 cards** with complete data
- **6 spread types**

## 🔮 Example Session

```bash
$ arcana
# [TUI launches - select "New Reading"]
# [Choose "Three-Card Spread"]
# [Watch shuffle animation]
# [Press Space to reveal each card...]

# Card 1 - Past: The Hermit
# "A period of withdrawal and introspection..."

# Card 2 - Present: Three of Cups
# "Celebrate with friends and community..."

# Card 3 - Future: The Star
# "Hope shines in the darkness..."

# [Press 's' to save to journal]
```

## 📄 License

MIT License

## 🙏 Acknowledgments

- [Rider-Waite-Smith Tarot](https://en.wikipedia.org/wiki/Rider-Waite_tarot) - The classic deck that inspired this project
- [Catppuccin](https://catppuccin.com/) - The prettiest color palette
- [ratatui](https://github.com/ratatui-org/ratatui) - The most wonderful TUI library


