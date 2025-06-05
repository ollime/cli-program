use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Widget, Block, Paragraph, Padding, Wrap, Tabs},
    symbols:: border, symbols,
    buffer::Buffer,
    layout::Rect,
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Ratatui Simple Template")
        .bold()
        .blue()
        .centered();

        let hello_text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";

        let content = Text::from(
            vec![
                Line::from(hello_text).yellow(),
                Line::from(self.counter.to_string()).blue(),
                Line::from("this is a test"),
            ]
        );

        let block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::new(5, 5, 5, 5))
            .title(title);

        // let paragraph = Paragraph::new(content)
        //     .block(block)
        //     .wrap(Wrap {trim: true})
        //     .centered();

        let tabs = Tabs::new(vec!["tab1".red(), "tab2".blue(), "tab3".yellow()])
            .block(block)
            .select(2)
            .divider(symbols::DOT)
            .padding("->", "<-");

        tabs.render(area, buf);
    }
}