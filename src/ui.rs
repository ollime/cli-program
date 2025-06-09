use ratatui::{
    style::{Stylize, Color},
    text::{Line, Text, Span},
    widgets::{Widget, Block, Paragraph, Padding, Wrap, Tabs,
        List, ListItem, ListState, Borders},
    symbols:: border, symbols,
    buffer::Buffer,
    layout::Rect,
};
use strum::IntoEnumIterator;
use ratatui::prelude::*;

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
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(2), // top 2 lines for title block
                Constraint::Fill(1),
            ])
            .split(area);

        self.render_title(outer_layout[0], buf);

        let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(75)
                ])
                .split(outer_layout[1]);

        // self.render_main_layout(outer_layout[0], buf);
        // self.render_side_bar(outer_layout[1], buf);
        
        Block::bordered()
            .render(inner_layout[0], buf);
        Block::bordered()
            .render(inner_layout[1], buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let tab_titles = CurrentScreen::iter().map(CurrentScreen::title);
        let current_screen_index = self.current_screen as usize;
        Tabs::new(tab_titles)
            .select(current_screen_index) // sets tab index
            .divider(symbols::DOT)
            .padding("->", "<-")
            .render(area, buf);
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("TUI program title")
            .bold()
            .blue()
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
                Span::from("word count: 32")
            ]
        );
        text_display.render(title_block_area, buf);
    }

    fn render_main_layout(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("TUI program title")
            .bold()
            .blue()
            .centered();

        // Renders double nested blocks
        let outer_block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::horizontal(1))
            .title(title)
            .title(
                Line::from(
                    vec![
                        Span::from("edit mode: "),
                        Span::from(format!("{}", match self.can_edit {
                            true => "on",
                            false => "off"
                        })).style(match self.can_edit {
                            true => Color::Green,
                            false => Color::Red,
                        })
                    ]
                ).right_aligned()
            );

        let inner_block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::new(2, 2, 1, 1));
        
        // By defining inner_area inside of outer_block, the rendered inner_block
        // will be inside of outer_block.
        let inner_area = outer_block.inner(area); // gets area inside of outer_block
        
        // Renders text inside of inner_block
        let paragraph_area = inner_block.inner(inner_area);

        outer_block.render(area, buf);
        inner_block.render(inner_area, buf); // renders inner_block in outer_block

        self.render_tabs(inner_area, buf);
        match self.current_screen {
            CurrentScreen::Main => self.current_screen.render_main_tab(paragraph_area, buf),
            CurrentScreen::Tab1 => self.current_screen.render_tab(self, paragraph_area, buf),
            CurrentScreen::Tab2 => self.current_screen.render_tab(self, paragraph_area, buf),
            CurrentScreen::Tab3 => self.current_screen.render_tab(self, paragraph_area, buf),
        }
    }

    fn render_side_bar(&self, area: Rect, buf: &mut Buffer) {
        // block rendering
        let block = Block::new()
            .padding(Padding::vertical(2));
        let inner_area = block.inner(area); // get area inside of the block
        block.render(area, buf); // render the block

        // list rendering
        let list_items = vec![
            "info",
            "export",
            "notes",
        ];
        let list_items: Vec<ListItem> = list_items
            .into_iter()
            .map(|item| ListItem::new(
                Line::from(
                    item.yellow() // using Stylize syntax
            )
            ))
            .collect();

        // render list inside the block
        Widget::render(&List::new(list_items), inner_area, buf);
    }
}

impl CurrentScreen {
    fn title(self) -> Line<'static> {
        format!(" {self} ")
            .into()
    }

    fn render_main_tab(self, area: Rect, buf: &mut Buffer) {
        let hello_text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";

        let content = Text::from(
            vec![
                Line::from(hello_text).yellow(),
                Line::from("this is a test"),
            ]
        );

        Paragraph::new(content)
            .wrap(Wrap {trim: true})
            .centered()
            .render(area, buf);
    }
    
    fn render_tab(self, app: &App, area: Rect, buf: &mut Buffer) {
        let current_tab_index = app.current_screen as usize;
        let data = app.tab_data.get(&current_tab_index).expect("No data found for tab 0");

        TextInput::new(data)
            .render(area, buf);
    }
}