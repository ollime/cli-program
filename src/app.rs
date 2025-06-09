use color_eyre::Result;
use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use strum_macros::{Display, EnumIter, FromRepr};
use strum::IntoEnumIterator;
use std::collections::HashMap;

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

/// The main application which holds the state and logic of the application.
// #[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    pub current_screen: CurrentScreen,
    pub can_edit: bool,
    pub tab_data: HashMap<usize, String>,
    pub cursor_pos: usize,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            current_screen: CurrentScreen::Main,
            can_edit: true,
            tab_data: HashMap::new(),
            cursor_pos: 0,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        // initialize hashmap data
        CurrentScreen::iter().for_each(|tab_index| {
            self.tab_data.insert(
                tab_index as usize,
                String::new()
            );
        });

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
            
            (_, KeyCode::Char('[')) => self.previous_tab(),
            (_, KeyCode::Char(']')) => self.next_tab(),
            
            // TODO: move cursor back/forwards
            (_, KeyCode::Left) => {
                if !self.can_edit {
                    self.previous_tab()
                }
                // else if self.cursor_pos > 1 {
                //     self.move_cursor(self.cursor_pos - 1)
                // }
            }
            (_, KeyCode::Right) => {
                if !self.can_edit {
                    self.next_tab()
                }
                // else {
                //     self.move_cursor(self.cursor_pos + 1)
                // }
            }

            // TODO: implement better way to enter edit mode
            (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                self.can_edit = !self.can_edit;
            },

            // editing text input
            (_, KeyCode::Char(value)) => {
                if self.can_edit {
                    let current_tab_index = self.current_screen as usize;
                    
                    // cannot be first index (main tab)
                    if current_tab_index > 0 {
                        let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
                        let new_content = current_tab_data.clone() + &value.to_string();
                        
                        self.tab_data.insert(
                            current_tab_index,
                            new_content
                        );
                    }
                }
            },
            (_, KeyCode::Enter) => {
                if self.can_edit {
                    let current_tab_index = self.current_screen as usize;
                    let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
                    let new_content = current_tab_data.clone() + "\n";

                    self.tab_data.insert(
                        current_tab_index,
                        new_content
                    );
                }
            },
            (_, KeyCode::Backspace) => {
                if self.can_edit {
                    let current_tab_index = self.current_screen as usize;
                    let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();

                    if current_tab_data.len() > 0 {
                        let new_content = current_tab_data
                            .strip_suffix(|_: char| true)
                            .unwrap()
                            .to_string();
    
                        self.tab_data.insert(
                            current_tab_index,
                            new_content
                        );
                    }
                }
            },
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        self.current_screen = self.current_screen.next();
        self.cursor_pos = 0; // reset cursor position
    }

    pub fn previous_tab(&mut self) {
        self.current_screen = self.current_screen.previous();
        self.cursor_pos = 0; // reset cursor position
    }

    fn move_cursor(&mut self, new_index: usize) {
        let current_tab_index = self.current_screen as usize;
        if let Some(current_tab_data) = self.tab_data.get(&current_tab_index) {
            let mut new_content = current_tab_data.replace("\\|", "");
            // new_content.insert_str(new_index, "\\|");
            self.tab_data.insert(current_tab_index, new_content);
        }
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