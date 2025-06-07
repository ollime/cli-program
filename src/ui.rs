use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Widget, Block, Paragraph, Padding, Wrap, Tabs},
    symbols:: border, symbols,
    buffer::Buffer,
    layout::Rect,
};
use strum::IntoEnumIterator;

use crate::app::App;
use crate::app::CurrentScreen;

impl Widget for &App {
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("TUI program title")
            .bold()
            .blue()
            .centered();
        
        // Renders double nested blocks
        let outer_block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::horizontal(1))
            .title(title);
        let inner_block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::new(5, 5, 5, 5));
        // By defining inner_area inside of outer_block, the rendered inner_block
        // will be inside of outer_block.
        let inner_area = outer_block.inner(area); // gets area inside of outer_block
        
        // Renders text inside of inner_block
        let paragraph_area = inner_block.inner(inner_area);

        outer_block.render(area, buf);
        inner_block.render(inner_area, buf); // renders inner_block in outer_block
        // paragraph.render(paragraph_area, buf); // renders paragraph in inner_block

        self.render_tabs(inner_area, buf);
        self.current_screen.render(paragraph_area, buf);
    }
}

impl Widget for CurrentScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
                // let paragraph = Paragraph::new(Text::from(self.input_value.as_str()))
        //     .wrap(Wrap {trim: true})
        //     .centered();
        match self {
            Self::Main => self.render_main_tab(area, buf),
            Self::Tab1 => self.render_tab(area, buf, String::from("tab1")),
            Self::Tab2 => self.render_tab(area, buf, String::from("test")),
            Self::Tab3 => self.render_tab(area, buf, String::from("!!!")),
        }
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

    // content for each tab
    fn render_tab(self, area: Rect, buf: &mut Buffer, text: String) {
        Paragraph::new(text)
            .render(area, buf);
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
}