use color_eyre::Result;
use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use strum_macros::{Display, EnumIter, FromRepr};
use strum::IntoEnumIterator;
use std::{collections::HashMap, thread::current};

use crate::export::Export;

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
    pub show_popup: bool,
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
            show_popup: false,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        // initialize hashmap data
        CurrentScreen::iter().for_each(|tab_index| {
            self.tab_data.insert(
                tab_index as usize,
                String::from("")
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
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('q')) => if !self.can_edit { self.quit() } else {self.insert_char('q')}

            // Switching current screen
            (_, KeyCode::Char('[')) => self.previous_tab(),
            (_, KeyCode::Char(']')) => self.next_tab(),
            
            // Horizontal arrows can be used to either switch tab or move cursor (when in edit mode)
            (_, KeyCode::Left) => {
                if !self.can_edit {
                    self.previous_tab()
                }
                else if self.cursor_pos > 0 { // must be 1 or greater to prevent sign error
                    self.cursor_pos = self.cursor_pos - 1;
                }
            }
            (_, KeyCode::Right) => {
                let size = self.get_character_count();
                if !self.can_edit {
                    self.next_tab()
                }
                else if self.cursor_pos < size { // cannot exceed the current text size
                    self.cursor_pos = self.cursor_pos + 1;
                }
            }
            
            // Navigates text input
            (_, KeyCode::Up) => {
                if self.can_edit {
                    self.previous_line()
                }
            }
            (_, KeyCode::Down) => {
                if self.can_edit {
                    self.next_line()
                }
            }

            // Export/save
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                self.show_popup = !self.show_popup
            }
            (_, KeyCode::Char('y')) => {
                if self.show_popup { // confirm popup and save data
                    if self.current_screen.to_string() != "Main" {
                        let current_tab_index = self.current_screen as usize;
                        let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
                        let _ = Export::export_as_html(current_tab_data, self.current_screen.to_string());
                        self.show_popup = false; // close popup
                    }
                }
                else {
                    self.insert_char('y');
                }
            }
            (_, KeyCode::Char('n')) => {
                if self.show_popup {
                    self.show_popup = false;
                }
                else {
                    self.insert_char('n');
                }
            }

            // Toggle edit mode
            (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                self.can_edit = !self.can_edit;
            },

            // Editing text input
            (_, KeyCode::Char(value)) => self.insert_char(value),
            (_, KeyCode::Enter) => self.insert_newline(),
            (_, KeyCode::Backspace) => self.delete_char(),
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

    fn next_line(&mut self) {
        // get characters after current cursor_pos (max should be max # of cols?)
        // then find next \n character
        // then find num of characters between cursor_pos and \n
        // add that num to cursor_pos

        let current_tab_index = self.current_screen as usize;
        let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
        let str_slice = &current_tab_data[self.cursor_pos..]; // string after cursor_pos

        if let Some(newline_index) = str_slice.find('\n') {
            let line_diff = &str_slice[..newline_index].len(); // difference between cursor_pos and \n
            
            let next_str_slice = &str_slice[(newline_index + 1)..];
            // to put cursor at the end of the next line
            // if it can't find the next \n char, uses the last char instead (to ensure last line still works)
            let next_new_line = next_str_slice.find('\n').unwrap_or(next_str_slice.len());

            // moves cursor
            self.cursor_pos = self.cursor_pos + line_diff + next_new_line + 1;
        }
    }
    
    fn previous_line(&mut self) {
        // get characters after current cursor_pos (max should be max # of cols?)
        // then find next \n character
        // then find num of characters between cursor_pos and \n
        // add that num to cursor_pos
        
        let current_tab_index = self.current_screen as usize;
        let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
        let str_slice = &current_tab_data[..self.cursor_pos]; // string before cursor_pos
        
        if let Some(newline_index) = str_slice.rfind('\n') {
            let line_diff = &str_slice[newline_index..].len(); // difference between cursor_pos and \n
            self.cursor_pos = self.cursor_pos - line_diff;
        }
    }

    fn insert_char(&mut self, value: char) {
        if self.can_edit {
            let current_tab_index = self.current_screen as usize;
            // cannot be first index (main tab)
            if current_tab_index > 0 {
                let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
                let mut new_content = current_tab_data.clone();

                // returns cursor_pos or new length depending on which one is smaller
                // if cursor_pos is smaller, places at location in text
                // if length is smaller, then places cursor at end of text
                let cursor = self.cursor_pos.min(new_content.len());

                new_content.insert(cursor, value);
                self.tab_data.insert(current_tab_index, new_content);
                self.cursor_pos = cursor + 1;
            }
        }
    }

    fn delete_char(&mut self) {
        if self.can_edit {
            let current_tab_index = self.current_screen as usize;
            let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();

            if current_tab_data.len() > 0 { // cannot delete if there is no text
                let mut new_content = current_tab_data.clone();

                if self.cursor_pos < current_tab_data.len() {
                    let remove_pos = self.cursor_pos - 1;
                    new_content.remove(remove_pos);
                    self.cursor_pos = self.cursor_pos - 1;
                }
                else {
                    new_content = new_content
                        .strip_suffix(|_: char| true)
                        .unwrap()
                        .to_string();
                }

                self.tab_data.insert(
                    current_tab_index,
                    new_content
                );
            }
        }
    }

    fn insert_newline(&mut self) {
        if self.can_edit {
            let current_tab_index = self.current_screen as usize;
            let current_tab_data = self.tab_data.get(&current_tab_index).unwrap();
            let mut new_content = current_tab_data.clone();
            
            let cursor = self.cursor_pos.min(new_content.len());
            new_content.insert(cursor, '\n');
            self.tab_data.insert(current_tab_index, new_content);
            self.cursor_pos = cursor + 1;
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