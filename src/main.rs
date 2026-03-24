use clap::{Parser, Subcommand, ValueEnum};
use crossterm::style::Stylize;
use ui::theme::ColorStyle;

mod cards;
mod deck;
mod journal;
mod spreads;
mod tui;
mod ui;

use cards::{ArcanaType, Card, DrawnCard, Suit};
use deck::{utils, Deck};
use spreads::{get_by_name, Reading, Spread, CELTIC_CROSS, FIVE_CARD_CROSS, MIND_BODY_SPIRIT, SITUATION_ACTION_OUTCOME, SINGLE, THREE_CARD};
use ui::theme::style;

/// Arcana - A beautiful terminal tarot application
#[derive(Parser)]
#[command(name = "arcana")]
#[command(about = "🔮 A rich, beautiful terminal tarot application")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the interactive TUI (default)
    #[command(alias = "t")]
    Tui,

    /// Show detailed information about a specific card
    #[command(alias = "c")]
    Card {
        /// The name of the card (e.g., "The Fool", "Ace of Wands")
        name: Option<String>,

        /// List all Major Arcana cards
        #[arg(short, long, conflicts_with = "suit")]
        major: bool,

        /// List all cards of a specific suit
        #[arg(short, long, value_enum)]
        suit: Option<SuitArg>,
    },

    /// List all cards in the deck
    #[command(alias = "ls")]
    List {
        /// Show only Major Arcana
        #[arg(short, long, conflicts_with = "suit")]
        major: bool,

        /// Show only cards of a specific suit
        #[arg(short, long, value_enum, conflicts_with = "major")]
        suit: Option<SuitArg>,

        /// Show detailed information for each card
        #[arg(short, long)]
        long: bool,
    },

    /// Draw a tarot reading
    #[command(alias = "r")]
    Read {
        /// Type of spread to use
        #[arg(short, long, value_enum, default_value = "single")]
        spread: SpreadType,

        /// Save reading to journal
        #[arg(short, long)]
        journal: bool,

        /// Disable card reversals
        #[arg(long)]
        no_reversals: bool,

        /// Custom spread name (alternative to --spread)
        #[arg(long, conflicts_with = "spread")]
        spread_name: Option<String>,
    },

    /// Show available tarot spreads
    Spreads,
}

#[derive(Clone, Copy, ValueEnum)]
enum SuitArg {
    Wands,
    Cups,
    Swords,
    Pentacles,
}

impl From<SuitArg> for Suit {
    fn from(arg: SuitArg) -> Self {
        match arg {
            SuitArg::Wands => Suit::Wands,
            SuitArg::Cups => Suit::Cups,
            SuitArg::Swords => Suit::Swords,
            SuitArg::Pentacles => Suit::Pentacles,
        }
    }
}

#[derive(Clone, Copy, ValueEnum, PartialEq)]
enum SpreadType {
    Single,
    Three,
    Situation,
    MindBodySpirit,
    Five,
    Celtic,
}

impl SpreadType {
    fn as_spread(&self) -> &'static Spread {
        match self {
            SpreadType::Single => &SINGLE,
            SpreadType::Three => &THREE_CARD,
            SpreadType::Situation => &SITUATION_ACTION_OUTCOME,
            SpreadType::MindBodySpirit => &MIND_BODY_SPIRIT,
            SpreadType::Five => &FIVE_CARD_CROSS,
            SpreadType::Celtic => &CELTIC_CROSS,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tui => {
            if let Err(e) = tui::run_tui() {
                eprintln!("TUI error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Card { name, major, suit } => cmd_card(name, major, suit),
        Commands::List { major, suit, long } => cmd_list(major, suit, long),
        Commands::Read {
            spread,
            journal,
            no_reversals,
            spread_name,
        } => cmd_read(spread, journal, no_reversals, spread_name),
        Commands::Spreads => cmd_spreads(),
    }
}

/// Handle the `card` subcommand
fn cmd_card(name: Option<String>, major: bool, suit: Option<SuitArg>) {
    if major {
        print_header("MAJOR ARCANA CARDS");
        println!();

        for card in utils::major_arcana() {
            print_card_line(card);
        }
        return;
    }

    if let Some(suit_arg) = suit {
        let suit: Suit = suit_arg.into();
        print_header(&format!("{:?} CARDS", suit).to_uppercase());
        println!();

        for card in utils::by_suit(suit) {
            print_card_line(card);
        }
        return;
    }

    // Show specific card
    if let Some(name) = name {
        if let Some(card) = Deck::find_by_name(&name) {
            print_card_detail(card);
        } else {
            // Try searching
            let results = Deck::search_by_name(&name);
            if results.is_empty() {
                eprintln!("{}", format!("❌ Card not found: '{}'", name).red());
                eprintln!();
                eprintln!("Tip: Use 'arcana list' to see all available cards.");
                std::process::exit(1);
            } else if results.len() == 1 {
                print_card_detail(results[0]);
            } else {
                println!("{}", format!("🔮 Multiple cards match '{}'", name).mauve());
                println!();
                for card in results {
                    print_card_line(card);
                }
            }
        }
    } else {
        // No name provided, show help
        println!("{}", "Usage: arcana card <CARD_NAME>".bold());
        println!();
        println!("Options:");
        println!("  --major       List all Major Arcana cards");
        println!("  --suit <SUIT> List all cards of a specific suit (wands, cups, swords, pentacles)");
        println!();
        println!("Examples:");
        println!("  arcana card \"The Fool\"");
        println!("  arcana card \"Ace of Wands\"");
        println!("  arcana card --major");
        println!("  arcana card --suit cups");
    }
}

/// Handle the `list` subcommand
fn cmd_list(major: bool, suit: Option<SuitArg>, long: bool) {
    let cards: Vec<&Card> = if major {
        utils::major_arcana().iter().collect()
    } else if let Some(suit_arg) = suit {
        utils::by_suit(suit_arg.into()).collect()
    } else {
        Deck::iter_all_cards().collect()
    };

    if major {
        print_header("MAJOR ARCANA (22 cards)");
    } else if let Some(s) = suit {
        print_header(&format!("{:?} (14 cards)", Suit::from(s)).to_uppercase());
    } else {
        print_header("COMPLETE DECK (78 cards)");
    }
    println!();

    if long {
        for (i, card) in cards.iter().enumerate() {
            if i > 0 {
                println!("{}", repeat_char('─', 66));
                println!();
            }
            print_card_detail(card);
        }
    } else {
        // Group by arcana type
        let major_cards: Vec<_> = cards
            .iter()
            .filter(|c| c.arcana == ArcanaType::Major)
            .copied()
            .collect();
        let minor_cards: Vec<_> = cards
            .iter()
            .filter(|c| c.arcana == ArcanaType::Minor)
            .copied()
            .collect();

        if !major_cards.is_empty() && (major || suit.is_none()) {
            print_subheader("MAJOR ARCANA");
            for card in &major_cards {
                print_card_line(card);
            }
            println!();
        }

        if !minor_cards.is_empty() && !major {
            // Group by suit
            for suit in [Suit::Wands, Suit::Cups, Suit::Swords, Suit::Pentacles] {
                let suit_cards: Vec<_> = minor_cards
                    .iter()
                    .filter(|c| c.suit == Some(suit))
                    .copied()
                    .collect();
                if !suit_cards.is_empty() {
                    print_subheader(&suit.to_string().to_uppercase());
                    for card in &suit_cards {
                        print_card_line(card);
                    }
                    println!();
                }
            }
        }
    }

    println!();
    println!("{}", "💡 Tip: Use 'arcana card <name>' for detailed information.".subtext());
}

/// Handle the `read` subcommand
fn cmd_read(
    spread_type: SpreadType,
    _journal: bool,
    no_reversals: bool,
    spread_name: Option<String>,
) {
    // Get the spread
    let spread: &'static Spread = if let Some(name) = spread_name {
        match get_by_name(&name) {
            Some(s) => s,
            None => {
                eprintln!("{}", format!("❌ Unknown spread: '{}'", name).red());
                eprintln!();
                eprintln!("Use 'arcana spreads' to see available spreads.");
                std::process::exit(1);
            }
        }
    } else {
        spread_type.as_spread()
    };

    let mut deck = Deck::new();
    deck.set_allow_reversals(!no_reversals);

    // Create header
    print_header(&spread.name.to_uppercase());
    println!();
    println!("{}", spread.description.subtext());
    println!();

    // Shuffle animation with colors
    print_shuffle_animation();
    println!();

    // Draw cards
    let drawn = deck.draw_many(spread.card_count());
    let reading = Reading::new(spread, drawn);

    // Display reading based on spread layout
    match spread.layout {
        spreads::SpreadLayout::Single => display_single_reading(&reading),
        spreads::SpreadLayout::Linear => display_linear_reading(&reading),
        spreads::SpreadLayout::Cross => display_cross_reading(&reading),
        spreads::SpreadLayout::CelticCross => display_celtic_cross_reading(&reading),
    }

    // Footer
    println!();
    println!("{}", "✨ Reading complete. May these cards guide your path.".mauve().bold());

    // Save to journal if requested
    if _journal {
        // TODO: Implement journal saving in Phase 5
        println!();
        println!("{}", "💾 Journal saving will be available in Phase 5.".yellow());
    }
}

/// Display a single card reading
fn display_single_reading(reading: &Reading) {
    if let Some((card, position)) = reading.get_card_at(0) {
        print_drawn_card(position.name, card);
    }
}

/// Display a linear (horizontal) spread
fn display_linear_reading(reading: &Reading) {
    for (i, (card, position)) in reading
        .drawn
        .iter()
        .zip(reading.spread.positions.iter())
        .enumerate()
    {
        if i > 0 {
            println!();
        }
        print_drawn_card(position.name, card);
    }
}

/// Display a cross spread (5 cards)
fn display_cross_reading(reading: &Reading) {
    // Show cards with their positions
    for (i, (card, position)) in reading
        .drawn
        .iter()
        .zip(reading.spread.positions.iter())
        .enumerate()
    {
        if i > 0 {
            println!();
        }
        print_drawn_card(position.name, card);
    }

    // Show ASCII layout
    println!();
    println!("{}", "Spread Layout:".bold().underlined());
    println!();
    println!("         (3) Past              ");
    println!("            │                  ");
    println!("    (4) Future ┼ (1) Present ┼ (2) Challenge");
    println!("            │                  ");
    println!("         (5) Potential         ");
    println!();
}

/// Display Celtic Cross reading with layout
fn display_celtic_cross_reading(reading: &Reading) {
    // Show all cards with positions
    for (i, (card, position)) in reading
        .drawn
        .iter()
        .zip(reading.spread.positions.iter())
        .enumerate()
    {
        if i > 0 {
            println!();
        }
        print_drawn_card(position.name, card);
    }

    // Show ASCII layout
    if reading.card_count() >= 10 {
        println!();
        println!(
            "{}",
            "Celtic Cross Layout:".bold().underlined()
        );
        println!();

        let card_name = |idx: usize| -> String {
            if let Some(card) = reading.drawn.get(idx) {
                truncate(&card.card.name, 13)
            } else {
                "???".to_string()
            }
        };

        println!("                    ┌─────┐");
        println!("                    │  5  │ {}", card_name(4));
        println!("          ┌─────┼─────┼─────┐         ┌─────┐");
        println!("          │  4  │1/2 │  6  │         │ 10  │");
        println!(
            "          │ {:13} │ {:13} │         │ {:13} │",
            card_name(3),
            card_name(5),
            card_name(9)
        );
        println!("          └─────┼─────┼─────┘         ├─────┤");
        println!("                │  3  │               │  9  │");
        println!(
            "                │ {:13} │               │ {:13} │",
            card_name(2),
            card_name(8)
        );
        println!("                └─────┘               ├─────┤");
        println!("                                      │  8  │");
        println!(
            "                                      │ {:13} │",
            card_name(7)
        );
        println!("                                      ├─────┤");
        println!("                                      │  7  │");
        println!(
            "                                      │ {:13} │",
            card_name(6)
        );
        println!("                                      └─────┘");
        println!();
        println!("Card 1 crosses Card 2 (The Challenge)");
    }
}

/// Handle the `spreads` subcommand
fn cmd_spreads() {
    print_header("AVAILABLE SPREADS");
    println!();

    for (i, spread) in spreads::all_spreads().iter().enumerate() {
        if i > 0 {
            println!();
        }

        print_boxed_header(spread.name, &format!("{} cards", spread.card_count()));
        println!("  {}", spread.description.subtext());
        println!();

        for position in spread.positions.iter() {
            println!(
                "  {} {} — {}",
                "•".mauve(),
                position.name.sky().bold(),
                position.description.subtext()
            );
        }
    }

    println!();
    println!("{}", "💡 Tip: Use --no-reversals to draw all cards upright.".subtext());
}

// ============================================================================
// Display Helpers
// ============================================================================

/// Print a header with box drawing
fn print_header(title: &str) {
    let width = 66;
    let title_text = format!(" 🔮 {} ", title);
    let title_len = title_text.len();
    let padding = (width - title_len) / 2;

    println!(
        "{}{}{}",
        "╔".mauve(),
        repeat_char('═', width).mauve(),
        "╗".mauve()
    );
    println!(
        "{}{}{}{}{}",
        "║".mauve(),
        " ".repeat(padding),
        title_text.bold(),
        " ".repeat(width - padding - title_len),
        "║".mauve()
    );
    println!(
        "{}{}{}",
        "╚".mauve(),
        repeat_char('═', width).mauve(),
        "╝".mauve()
    );
}

/// Print a subheader
fn print_subheader(title: &str) {
    println!(
        "{}",
        format!("━━━ {} ━━━", title).lavender()
    );
}

/// Print a boxed header for spread display
fn print_boxed_header(title: &str, subtitle: &str) {
    let header = format!(" {} ({})", title, subtitle);
    let header_len = header.len();
    println!(
        "┌{}┐",
        repeat_char('─', header_len + 2)
    );
    println!("│ {} │", header.lavender().bold());
    println!(
        "└{}┘",
        repeat_char('─', header_len + 2)
    );
}

/// Print a card in list format
fn print_card_line(card: &Card) {
    let symbol = match card.arcana {
        ArcanaType::Major => "★".mauve().to_string(),
        ArcanaType::Minor => {
            if let Some(suit) = card.suit {
                match suit {
                    Suit::Wands => "🔥".peach().to_string(),
                    Suit::Cups => "💧".pink().to_string(),
                    Suit::Swords => "⚔️".red().to_string(),
                    Suit::Pentacles => "🌿".green().to_string(),
                }
            } else {
                "○".lavender().to_string()
            }
        }
    };

    let display = style::card_name(card);
    let keywords = if card.keywords.len() >= 3 {
        format!(
            "{} | {} | {}",
            card.keywords[0], card.keywords[1], card.keywords[2]
        )
    } else {
        card.keywords.join(" | ")
    };

    println!(
        "  {} {:<35} {}",
        symbol,
        display,
        keywords.subtext()
    );
}

/// Calculate visible length of a string (excluding ANSI escape codes)
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c.is_alphabetic() {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

/// Print a line with content that has a left border and right border at exact width
fn print_content_line(left_border: &str, content: &str, right_border: &str, width: usize) {
    let visible = visible_len(content);
    let padding = width.saturating_sub(visible);
    println!("{}{}{}{}", left_border, content, " ".repeat(padding), right_border);
}

/// Print detailed card information
fn print_card_detail(card: &Card) {
    let width = 66;
    let vbar = "│".lavender();
    let hbar = repeat_char('─', width).lavender();

    // Top border
    println!("╭{}╮", hbar);

    // Title - centered
    let title_plain = format!("✦ {}", card.display_name());
    let title_styled = format!("{} {}", "✦".bold(), card.display_name().bold());
    let title_padding = (width.saturating_sub(visible_len(&title_plain))) / 2;
    let title_line = format!("{}{}", " ".repeat(title_padding), title_styled);
    print_content_line(&vbar, &title_line, &vbar, width);

    // Separator
    println!("├{}┤", hbar);

    // Arcana type and suit info
    let arcana_info = if card.arcana == ArcanaType::Major {
        format!("Major Arcana {}", card.roman_numeral().unwrap_or(""))
    } else {
        format!(
            "Minor Arcana — {} ({})",
            card.suit.map(|s| s.to_string()).unwrap_or_default(),
            card.suit.map(|s| s.element().to_string()).unwrap_or_default()
        )
    };
    print_content_line(&vbar, &format!(" {}", arcana_info), &vbar, width);

    // Keywords
    let keywords_text = format!(" Keywords: {}", card.keywords_string());
    print_content_line(&vbar, &format!("  {}", keywords_text.subtext()), &vbar, width);

    // Separator
    println!("├{}┤", hbar);

    // Upright meaning
    let upright_wrapped = textwrap::fill(card.upright, width - 4);
    let mut upright_lines = upright_wrapped.lines();
    if let Some(first_line) = upright_lines.next() {
        let label = "Upright".green().bold();
        let line_content = format!(" {}: {}", label, first_line);
        print_content_line(&vbar, &line_content, &vbar, width);
    }
    for line in upright_lines {
        let line_content = format!("   {}", line);
        print_content_line(&vbar, &line_content, &vbar, width);
    }

    // Separator
    println!("├{}┤", hbar);

    // Reversed meaning
    let reversed_wrapped = textwrap::fill(card.reversed, width - 4);
    let mut reversed_lines = reversed_wrapped.lines();
    if let Some(first_line) = reversed_lines.next() {
        let label = "Reversed".yellow().bold();
        let line_content = format!(" {}: {}", label, first_line);
        print_content_line(&vbar, &line_content, &vbar, width);
    }
    for line in reversed_lines {
        let line_content = format!("   {}", line);
        print_content_line(&vbar, &line_content, &vbar, width);
    }

    // Separator
    println!("├{}┤", hbar);

    if let Some(element) = card.element {
        let line = format!(" {:<12} {}", "Element:", element.to_string().sky());
        print_content_line(&vbar, &line, &vbar, width);
    }
    if let Some(astrology) = card.astrology {
        let line = format!(" {:<12} {}", "Astrology:", astrology.sky());
        print_content_line(&vbar, &line, &vbar, width);
    }
    if let Some(numerology) = card.numerology {
        let line = format!(" {:<12} {}", "Numerology:", numerology.sky());
        print_content_line(&vbar, &line, &vbar, width);
    }

    // Yes/No always shows
    let yes_no_line = format!(" {:<12} {}", "Yes/No:", card.yes_or_no.to_string().sky());
    print_content_line(&vbar, &yes_no_line, &vbar, width);

    // Bottom border
    println!("╰{}╯", hbar);
}

/// Print a drawn card in reading context
fn print_drawn_card(position_name: &str, drawn: &DrawnCard) {
    let width = 66;

    // Position header
    println!(
        "{}{}{}",
        "┌".lavender(),
        repeat_char('─', width).lavender(),
        "┐".lavender()
    );
    let pos_text = format!(" Position: {} ", position_name);
    let pos_len = pos_text.len();
    let padding = (width - pos_len) / 2;
    println!(
        "{}{}{}{}{}",
        "│".lavender(),
        " ".repeat(padding),
        pos_text.sky().bold(),
        " ".repeat(width - padding - pos_len),
        "│".lavender()
    );
    println!(
        "{}{}{}",
        "├".lavender(),
        repeat_char('─', width).lavender(),
        "┤".lavender()
    );

    // Card name with orientation
    let card_display = style::card_name(drawn.card);
    let orientation = if drawn.reversed {
        format!(" {}", style::reversed())
    } else {
        String::new()
    };
    let card_text = format!(" ✦ {}{}", card_display, orientation);
    let card_padding = (width - card_text.len().min(width)) / 2;
    println!(
        "{}{}{}{}{}",
        "│".lavender(),
        " ".repeat(card_padding),
        card_text,
        " ".repeat(width.saturating_sub(card_padding + card_text.len())),
        "│".lavender()
    );

    println!(
        "{}{}{}",
        "├".lavender(),
        repeat_char('─', width).lavender(),
        "┤".lavender()
    );

    // Keywords
    println!(
        "{} Keywords: {:<width$} {}",
        "│".lavender(),
        drawn.keywords().subtext(),
        "│".lavender(),
        width = width - 11
    );

    println!(
        "{}{}{}",
        "├".lavender(),
        repeat_char('─', width).lavender(),
        "┤".lavender()
    );

    // Meaning (upright or reversed)
    let meaning_label = if drawn.reversed {
        "Meaning (Reversed)".yellow().bold()
    } else {
        "Meaning".green().bold()
    };
    println!("{} {}:", "│".lavender(), meaning_label);
    let meaning_wrapped = textwrap::fill(drawn.meaning(), width - 4);
    for line in meaning_wrapped.lines() {
        println!("{}   {:<width$} {}", "│".lavender(), line, "│".lavender(), width = width - 3);
    }

    println!(
        "{}{}{}",
        "└".lavender(),
        repeat_char('─', width).lavender(),
        "┘".lavender()
    );
}

/// Print shuffle animation with colors
fn print_shuffle_animation() {
    use std::io::Write;
    use std::thread::sleep;
    use std::time::Duration;

    print!("{}", "Shuffling the deck".mauve());
    std::io::stdout().flush().unwrap();

    for _ in 0..3 {
        sleep(Duration::from_millis(300));
        print!("{}", ".".mauve());
        std::io::stdout().flush().unwrap();
    }

    sleep(Duration::from_millis(200));
    println!(" {}", "✓".green().bold());
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Helper to repeat a character
fn repeat_char(ch: char, count: usize) -> String {
    std::iter::repeat(ch).take(count).collect()
}

/// Truncate a string to max_len with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Simple text wrapping for descriptions
mod textwrap {
    pub fn fill(text: &str, width: usize) -> String {
        let mut result = String::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(&current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&current_line);
        }

        result
    }
}
