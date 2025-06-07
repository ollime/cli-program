use ratatui::{
    text::{Line, Span},
    widgets::{Widget, Paragraph, Wrap},
    buffer::Buffer,
    layout::Rect,
    style::{Style, Modifier, Color},
};

pub struct TextInput<'a> {
    pub input_value: &'a String,
}

impl<'a> TextInput<'a> {
    pub fn new(input_value: &'a String) -> Self {
        TextInput {
            input_value
        }
    }
}

impl<'a> Widget for TextInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = Line::from(
            vec![
                Span::from(self.input_value.as_str()),
                // adds blinking cursor to screen
                Span::from(String::from("|"))
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::RAPID_BLINK)
                        ),
            ]
        );
        
        Paragraph::new(content)
            .wrap(Wrap {trim: true})
            .render(area, buf);
        }
}