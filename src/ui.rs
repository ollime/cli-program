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

        // Renders double nested blocks
        let outer_block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::horizontal(1))
            .title(title);
        let inner_block = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::new(5, 5, 5, 5));
        let outer_area = area;
        // By defining inner_area inside of outer_block, the rendered inner_block
        // will be inside of outer_block.
        let inner_area = outer_block.inner(outer_area); // gets area inside of outer_block
        
        // Renders text inside of inner_block
        let paragraph = Paragraph::new(content)
            .wrap(Wrap {trim: true})
            .centered();
        let paragraph_area = inner_block.inner(inner_area);

        inner_block.render(inner_area, buf); // renders inner_block in outer_block
        paragraph.render(paragraph_area, buf); // renders paragraph in inner_block

        let tabs = Tabs::new(vec!["tab1".red(), "tab2".blue(), "tab3".yellow()])
            .block(outer_block)
            .select(2)
            .divider(symbols::DOT)
            .padding("->", "<-");

        tabs.render(area, buf);
    }
}

// 124fn render_email(selected_index: usize, area: Rect, buf: &mut Buffer) {
// 125    let theme = THEME.email;
// 126    let email = EMAILS.get(selected_index);
// 127    let block = Block::new()
// 128        .style(theme.body)
// 129        .padding(Padding::new(2, 2, 0, 0))
// 130        .borders(Borders::TOP)
// 131        .border_type(BorderType::Thick);
// 132    let inner = block.inner(area);
// 133    block.render(area, buf);
// 134    if let Some(email) = email {
// 135        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
// 136        let [headers_area, body_area] = vertical.areas(inner);
// 137        let headers = vec![
// 138            Line::from(vec![
// 139                "From: ".set_style(theme.header),
// 140                email.from.set_style(theme.header_value),
// 141            ]),
// 142            Line::from(vec![
// 143                "Subject: ".set_style(theme.header),
// 144                email.subject.set_style(theme.header_value),
// 145            ]),
// 146            "-".repeat(inner.width as usize).dim().into(),
// 147        ];
// 148        Paragraph::new(headers)
// 149            .style(theme.body)
// 150            .render(headers_area, buf);
// 151        let body = email.body.lines().map(Line::from).collect_vec();
// 152        Paragraph::new(body)
// 153            .style(theme.body)
// 154            .render(body_area, buf);
// 155    } else {
// 156        Paragraph::new("No email selected").render(inner, buf);
// 157    }
// 158}