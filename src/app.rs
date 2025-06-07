use color_eyre::Result;
use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum CurrentScreen {
    #[default]
    #[strum(to_string = "Main")]
    Main,
    #[strum(to_string = "Tab 1")]
    Tab1,
    #[strum(to_string = "Tab 2")]
    Tab2,
    #[strum(to_string = "Tab 3")]
    Tab3,
}

pub enum TextData {
    Value
}

/// The main application which holds the state and logic of the application.
// #[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    pub current_screen: CurrentScreen,
    pub input_value: String,
    pub can_edit: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            current_screen: CurrentScreen::Main,
            input_value: String::from("initial value"),
            can_edit: true,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Exits the program
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            
            (_, KeyCode::Right | KeyCode::Char('[')) => self.next_tab(),
            (_, KeyCode::Left | KeyCode::Char(']')) => self.previous_tab(),
                        
            // TODO: implement better way to enter edit mode
            (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                self.can_edit = !self.can_edit;
            },
            (_, KeyCode::Char(value)) => {
                if self.can_edit {
                    self.input_value.push(value);
                }
            },
            (_, KeyCode::Backspace) => {
                if self.can_edit {
                    self.input_value.pop();
                }
            },
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    // TODO: install strum
    pub fn next_tab(&mut self) {
        self.current_screen = self.current_screen.next();
    }

    pub fn previous_tab(&mut self) {
        self.current_screen = self.current_screen.previous();
    }

    pub fn get_input_value(&self) -> &str {
        &self.input_value
    }
}

impl CurrentScreen {
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}