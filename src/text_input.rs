use ratatui::{
    text::{Text},
    widgets::{Widget, Paragraph, Wrap},
    buffer::Buffer,
    layout::Rect,
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
        Paragraph::new(Text::from(self.input_value.as_str()))
            .wrap(Wrap {trim: true})
            .centered()
            .render(area, buf);
    }
}