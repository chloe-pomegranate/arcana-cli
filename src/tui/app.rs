//! TUI Application state and event loop

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::Color,
    widgets::ListState,
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

use crate::cards::{ArcanaType, Card, Suit};
use crate::deck::Deck;
use crate::journal::{Journal, JournalEntry};
use crate::spreads::{self, Reading, Spread};
use crate::tui::screens;

/// Screens in the TUI application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    #[allow(dead_code)]
    NewReading,
    SpreadSelection,
    ShuffleAnimation,
    CardReveal,
    ReadingComplete,
    CardBrowser,
    CardDetail,
    Journal,
    JournalDetail,
    Help,
    #[allow(dead_code)]
    Quit,
}

/// Menu items for the home screen
#[derive(Debug, Clone, Copy)]
pub enum MenuItem {
    NewReading,
    BrowseCards,
    Journal,
    Help,
    Quit,
}

impl MenuItem {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuItem::NewReading => "🔮  New Reading",
            MenuItem::BrowseCards => "📖  Browse Cards",
            MenuItem::Journal => "📓  Journal",
            MenuItem::Help => "❓  Help",
            MenuItem::Quit => "👋  Quit",
        }
    }
}

/// Filter options for card browser
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardFilter {
    All,
    Major,
    Suit(Suit),
}

/// Main application state
pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    
    // Home menu
    pub menu_items: Vec<MenuItem>,
    pub menu_state: ListState,
    
    // Card browser
    pub cards: Vec<&'static Card>,
    pub card_list_state: ListState,
    pub card_filter: CardFilter,
    pub selected_card: Option<&'static Card>,
    
    // Reading
    pub deck: Deck,
    pub selected_spread: Option<&'static Spread>,
    pub reading: Option<Reading>,
    pub reveal_index: usize,
    pub spread_options: Vec<&'static Spread>,
    pub spread_list_state: ListState,
    
    // Animation
    pub shuffle_progress: f32,
    
    // Theme
    pub theme: AppTheme,
    
    // Journal
    pub journal: Journal,
    pub journal_entries: Vec<JournalEntry>,
    pub journal_list_state: ListState,
    pub selected_journal_entry: Option<(JournalEntry, String)>,
}

/// Catppuccin Mocha theme for TUI
pub struct AppTheme {
    #[allow(dead_code)]
    pub base: Color,
    #[allow(dead_code)]
    pub mantle: Color,
    pub surface0: Color,
    pub surface1: Color,
    pub text: Color,
    pub subtext0: Color,
    pub lavender: Color,
    pub mauve: Color,
    pub pink: Color,
    pub red: Color,
    pub peach: Color,
    pub green: Color,
    pub yellow: Color,
    pub sky: Color,
    #[allow(dead_code)]
    pub teal: Color,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self {
            base: Color::Rgb(30, 30, 46),
            mantle: Color::Rgb(24, 24, 37),
            surface0: Color::Rgb(49, 50, 68),
            surface1: Color::Rgb(69, 71, 90),
            text: Color::Rgb(205, 214, 244),
            subtext0: Color::Rgb(166, 173, 200),
            lavender: Color::Rgb(180, 190, 254),
            mauve: Color::Rgb(203, 166, 247),
            pink: Color::Rgb(245, 194, 231),
            red: Color::Rgb(243, 139, 168),
            peach: Color::Rgb(250, 179, 135),
            green: Color::Rgb(166, 227, 161),
            yellow: Color::Rgb(249, 226, 175),
            sky: Color::Rgb(137, 220, 235),
            teal: Color::Rgb(148, 226, 213),
        }
    }
}

impl AppTheme {
    pub fn suit_color(&self, suit: Suit) -> Color {
        match suit {
            Suit::Wands => self.peach,
            Suit::Cups => self.pink,
            Suit::Swords => self.red,
            Suit::Pentacles => self.green,
        }
    }

    #[allow(dead_code)]
    pub fn arcana_color(&self, arcana: ArcanaType) -> Color {
        match arcana {
            ArcanaType::Major => self.mauve,
            ArcanaType::Minor => self.lavender,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));
        
        let mut card_list_state = ListState::default();
        card_list_state.select(Some(0));
        
        let mut spread_list_state = ListState::default();
        spread_list_state.select(Some(0));
        
        let all_cards: Vec<_> = Deck::iter_all_cards().collect();
        
        let spread_options = vec![
            &spreads::SINGLE,
            &spreads::THREE_CARD,
            &spreads::SITUATION_ACTION_OUTCOME,
            &spreads::MIND_BODY_SPIRIT,
            &spreads::FIVE_CARD_CROSS,
            &spreads::CELTIC_CROSS,
        ];
        
        Self {
            screen: Screen::Home,
            should_quit: false,
            menu_items: vec![
                MenuItem::NewReading,
                MenuItem::BrowseCards,
                MenuItem::Journal,
                MenuItem::Help,
                MenuItem::Quit,
            ],
            menu_state,
            cards: all_cards,
            card_list_state,
            card_filter: CardFilter::All,
            selected_card: None,
            deck: Deck::new(),
            selected_spread: None,
            reading: None,
            reveal_index: 0,
            spread_options,
            spread_list_state,
            shuffle_progress: 0.0,
            theme: AppTheme::default(),
            journal: Journal::default(),
            journal_entries: Vec::new(),
            journal_list_state: ListState::default(),
            selected_journal_entry: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_menu_item(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => (i + 1) % self.menu_items.len(),
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    fn previous_menu_item(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.menu_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    fn next_card(&mut self) {
        let i = match self.card_list_state.selected() {
            Some(i) => (i + 1) % self.cards.len(),
            None => 0,
        };
        self.card_list_state.select(Some(i));
        self.selected_card = self.cards.get(i).copied();
    }

    fn previous_card(&mut self) {
        let i = match self.card_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.cards.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.card_list_state.select(Some(i));
        self.selected_card = self.cards.get(i).copied();
    }

    fn next_spread(&mut self) {
        let i = match self.spread_list_state.selected() {
            Some(i) => (i + 1) % self.spread_options.len(),
            None => 0,
        };
        self.spread_list_state.select(Some(i));
    }

    fn previous_spread(&mut self) {
        let i = match self.spread_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.spread_options.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.spread_list_state.select(Some(i));
    }

    fn select_menu_item(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            match self.menu_items[i] {
                MenuItem::NewReading => {
                    self.screen = Screen::SpreadSelection;
                    self.deck.reset_and_shuffle();
                }
                MenuItem::BrowseCards => {
                    self.screen = Screen::CardBrowser;
                    self.refresh_card_list();
                }
                MenuItem::Journal => {
                    self.screen = Screen::Journal;
                }
                MenuItem::Help => {
                    self.screen = Screen::Help;
                }
                MenuItem::Quit => {
                    self.should_quit = true;
                }
            }
        }
    }

    fn refresh_card_list(&mut self) {
        self.cards = match self.card_filter {
            CardFilter::All => Deck::iter_all_cards().collect(),
            CardFilter::Major => crate::deck::utils::major_arcana().iter().collect(),
            CardFilter::Suit(suit) => crate::deck::utils::by_suit(suit).collect(),
        };
        
        // Ensure selection is valid
        if self.cards.is_empty() {
            self.card_list_state.select(None);
            self.selected_card = None;
        } else {
            let i = self.card_list_state.selected().unwrap_or(0).min(self.cards.len() - 1);
            self.card_list_state.select(Some(i));
            self.selected_card = self.cards.get(i).copied();
        }
    }

    fn cycle_card_filter(&mut self) {
        self.card_filter = match self.card_filter {
            CardFilter::All => CardFilter::Major,
            CardFilter::Major => CardFilter::Suit(Suit::Wands),
            CardFilter::Suit(Suit::Wands) => CardFilter::Suit(Suit::Cups),
            CardFilter::Suit(Suit::Cups) => CardFilter::Suit(Suit::Swords),
            CardFilter::Suit(Suit::Swords) => CardFilter::Suit(Suit::Pentacles),
            CardFilter::Suit(Suit::Pentacles) => CardFilter::All,
        };
        self.refresh_card_list();
    }

    fn start_reading(&mut self) {
        if let Some(i) = self.spread_list_state.selected() {
            self.selected_spread = Some(self.spread_options[i]);
            self.screen = Screen::ShuffleAnimation;
            self.shuffle_progress = 0.0;
        }
    }

    fn complete_shuffle(&mut self) {
        if let Some(spread) = self.selected_spread {
            let drawn = self.deck.draw_many(spread.card_count());
            self.reading = Some(Reading::new(spread, drawn));
            self.reveal_index = 0;
            self.screen = Screen::CardReveal;
        }
    }

    fn reveal_next_card(&mut self) {
        if let Some(ref reading) = self.reading {
            if self.reveal_index < reading.card_count() - 1 {
                self.reveal_index += 1;
            } else {
                self.screen = Screen::ReadingComplete;
            }
        }
    }

    fn go_back(&mut self) {
        match self.screen {
            Screen::Home => {}
            Screen::SpreadSelection | Screen::CardBrowser | Screen::Journal | Screen::Help => {
                self.screen = Screen::Home;
            }
            Screen::JournalDetail => {
                self.screen = Screen::Journal;
                self.selected_journal_entry = None;
            }
            Screen::ShuffleAnimation | Screen::CardReveal | Screen::ReadingComplete => {
                self.screen = Screen::Home;
                self.reading = None;
                self.reveal_index = 0;
            }
            Screen::CardDetail => {
                self.screen = Screen::CardBrowser;
                self.selected_card = None;
            }
            Screen::Quit => {}
            Screen::NewReading => {
                self.screen = Screen::SpreadSelection;
            }
        }
    }

    pub fn save_current_reading(&mut self) -> Result<String, String> {
        if let Some(ref reading) = self.reading {
            match self.journal.save_reading(reading) {
                Ok(path) => Ok(format!("Saved to {}", path.display())),
                Err(e) => Err(format!("Failed to save: {}", e)),
            }
        } else {
            Err("No reading to save".to_string())
        }
    }

    pub fn load_journal_entries(&mut self) {
        if let Ok(entries) = self.journal.load_entries() {
            self.journal_entries = entries;
            // Update list state
            if !self.journal_entries.is_empty() {
                self.journal_list_state.select(Some(0));
            }
        }
    }

    fn next_journal_entry(&mut self) {
        if self.journal_entries.is_empty() {
            return;
        }
        let i = match self.journal_list_state.selected() {
            Some(i) => (i + 1) % self.journal_entries.len(),
            None => 0,
        };
        self.journal_list_state.select(Some(i));
    }

    fn previous_journal_entry(&mut self) {
        if self.journal_entries.is_empty() {
            return;
        }
        let i = match self.journal_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.journal_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.journal_list_state.select(Some(i));
    }

    fn view_journal_entry(&mut self) {
        if let Some(i) = self.journal_list_state.selected() {
            if let Some(entry) = self.journal_entries.get(i) {
                if let Ok(content) = self.journal.load_entry_content(&entry.id) {
                    self.selected_journal_entry = Some((entry.clone(), content));
                    self.screen = Screen::JournalDetail;
                }
            }
        }
    }

    fn on_tick(&mut self) {
        if self.screen == Screen::ShuffleAnimation {
            self.shuffle_progress += 0.05;
            if self.shuffle_progress >= 1.0 {
                self.complete_shuffle();
            }
        }
    }
}

/// Run the TUI application
pub fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| screens::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.screen {
                        Screen::Home => handle_home_keys(&mut app, key.code),
                        Screen::SpreadSelection => handle_spread_selection_keys(&mut app, key.code),
                        Screen::ShuffleAnimation => handle_shuffle_keys(&mut app, key.code),
                        Screen::CardReveal => handle_card_reveal_keys(&mut app, key.code),
                        Screen::ReadingComplete => handle_reading_complete_keys(&mut app, key.code),
                        Screen::CardBrowser => handle_card_browser_keys(&mut app, key.code),
                        Screen::CardDetail => handle_card_detail_keys(&mut app, key.code),
                        Screen::Help => handle_help_keys(&mut app, key.code),
                        Screen::Journal => handle_journal_keys(&mut app, key.code),
                        Screen::JournalDetail => handle_journal_detail_keys(&mut app, key.code),
                        Screen::Quit => {}
                        Screen::NewReading => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_home_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.should_quit = true,
        KeyCode::Down | KeyCode::Char('j') => app.next_menu_item(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_menu_item(),
        KeyCode::Enter => app.select_menu_item(),
        _ => {}
    }
}

fn handle_spread_selection_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
        KeyCode::Down | KeyCode::Char('j') => app.next_spread(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_spread(),
        KeyCode::Enter => app.start_reading(),
        _ => {}
    }
}

fn handle_shuffle_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
        _ => {}
    }
}

fn handle_card_reveal_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right | KeyCode::Char('j') => {
            app.reveal_next_card();
        }
        _ => {}
    }
}

fn handle_reading_complete_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => app.go_back(),
        KeyCode::Char('s') => {
            match app.save_current_reading() {
                Ok(msg) => {
                    // Could show a toast notification here
                    eprintln!("\n{}\n", msg);
                }
                Err(e) => {
                    eprintln!("\n{}\n", e);
                }
            }
        }
        _ => {}
    }
}

fn handle_card_browser_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
        KeyCode::Down | KeyCode::Char('j') => app.next_card(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_card(),
        KeyCode::Tab => app.cycle_card_filter(),
        KeyCode::Enter => {
            if app.selected_card.is_some() {
                app.screen = Screen::CardDetail;
            }
        }
        _ => {}
    }
}

fn handle_card_detail_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => app.go_back(),
        _ => {}
    }
}

fn handle_help_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => app.go_back(),
        _ => {}
    }
}

fn handle_journal_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.go_back(),
        KeyCode::Down | KeyCode::Char('j') => app.next_journal_entry(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_journal_entry(),
        KeyCode::Enter => app.view_journal_entry(),
        _ => {}
    }
}

fn handle_journal_detail_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => app.go_back(),
        _ => {}
    }
}


