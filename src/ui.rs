use ratatui::{
    style::{Stylize, Color},
    text::{Line, Text, Span},
    widgets::{Widget, Block, Paragraph, Padding, Wrap, Tabs,
        List, ListItem, ListState, Borders},
    buffer::Buffer,
    layout::Rect,
};
use strum::IntoEnumIterator;
use ratatui::prelude::*;
use std::collections::BTreeMap;

use crate::app::App;
use crate::app::CurrentScreen;
use crate::text_input::TextInput;

impl Widget for &App {
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(2), // top 2 lines for title block
                Constraint::Fill(1),
            ])
            .split(area);

        self.render_title(title_layout[0], buf);

        let content_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(75)
                ])
                .split(title_layout[1]);

        // render left_block contents
        let left_block = Block::bordered();
        let left_block_area = left_block.inner(content_layout[0]); // grab the area inside the block
        left_block.render(content_layout[0], buf); // render block ui
        self.render_side_bar(left_block_area, buf);
        
        let right_block = Block::bordered(); // block ui
        let right_block_area = right_block.inner(content_layout[1]); // grab the area inside the block
        right_block.render(content_layout[1], buf); // render block ui

        // render right_block contents
        let right_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Max(2),
                    Constraint::Fill(1)
                ])
                .split(right_block_area);
        self.render_tabs(right_layout[0], buf); // render tab titles
        match self.current_screen { // render tab content
            CurrentScreen::Main => self.current_screen.render_main_tab(right_layout[1], buf),
            CurrentScreen::Tab1 => self.current_screen.render_tab(self, right_layout[1], buf),
            CurrentScreen::Tab2 => self.current_screen.render_tab(self, right_layout[1], buf),
            CurrentScreen::Tab3 => self.current_screen.render_tab(self, right_layout[1], buf),
        }
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let tabs_block = Block::bordered()
            .borders(Borders::BOTTOM);
        let tab_titles = CurrentScreen::iter().map(CurrentScreen::title);
        let current_screen_index = self.current_screen as usize;
        Tabs::new(tab_titles)
            .block(tabs_block)
            .select(current_screen_index) // sets tab index
            .divider(" ")
            .render(area, buf);
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("TUI program title")
            .bold()
            .cyan()
            .centered();
        let title_block = Block::bordered()
            .title(title)
            .borders(Borders::TOP)
            .padding(Padding::horizontal(1));
        let title_block_area = title_block.inner(area);
        title_block.render(area, buf);

        let text_display = Line::from(
            vec![
                Span::from("edit mode: "),
                Span::from(format!("{}", match self.can_edit {
                    true => "on",
                    false => "off"
                })).style(match self.can_edit {
                    true => Color::Green,
                    false => Color::Red,
                }),
                "  -  ".white(),
                Span::from("line: 2"),
                "  -  ".white(),
                Span::from(format!("character count: {}", self.get_character_count()))
            ]
        );
        text_display.render(title_block_area, buf);
    }

    fn get_character_count(&self) -> usize {
        let current_tab_index = self.current_screen as usize;
        let data = self.tab_data.get(&current_tab_index).expect("No data found for tab 0");
        data.len()
    }

    fn render_side_bar(&self, area: Rect, buf: &mut Buffer) {
        let side_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                    Constraint::Max(2),
                    Constraint::Fill(1),
                ]
            )
            .split(area);

        let tab_title = self.current_screen.to_string();
        Line::from(tab_title)
            .yellow()
            .render(side_layout[0], buf);
        
        // list rendering
        let list_items = vec![
            "info",
            "notes",
            "export",
        ];
        let list_items: Vec<ListItem> = list_items
            .into_iter()
            .map(|item| ListItem::new(
                Line::from(item)
            ))
            .collect();

        // render list inside the block
        Widget::render(&List::new(list_items), side_layout[1], buf);
    }
}

impl CurrentScreen {
    fn title(self) -> Line<'static> {
        format!(" {self} ")
            .into()
    }

    fn render_main_tab(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(2),
                Constraint::Fill(1)
            ])
            .split(area);

        // render text
        let text = "Commands";
        Paragraph::new(text)
            .render(main_layout[0], buf);

        // list rendering
        let list_items: BTreeMap<&str, &str> = BTreeMap::from([
            ("[ ]", "Switch tab"),
            ("Left/Right", "Switch tab (edit mode: off)"),
            ("Ctrl + E", "Change edit mode"),
        ]);

        let list_items: Vec<ListItem> = list_items
            .into_iter()
            .map(|item| ListItem::new(
                Line::from(
                    vec![
                        item.0.yellow().bold(), // using Stylize syntax
                        " ".into(),
                        item.1.into()
                    ]
                )
            ))
            .collect();

        // render list inside the block
        Widget::render(&List::new(list_items), main_layout[1], buf);
    }
    
    fn render_tab(self, app: &App, area: Rect, buf: &mut Buffer) {
        let current_tab_index = app.current_screen as usize;
        let data = app.tab_data.get(&current_tab_index).expect("No data found for tab 0");

        TextInput::new(data)
            .render(area, buf);
    }
}