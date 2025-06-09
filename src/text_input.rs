use ratatui::{
    text::{Line, Span},
    widgets::{Widget, Paragraph, Wrap},
    buffer::Buffer,
    layout::Rect,
    // style::{Style, Modifier, Color},
};

pub struct TextInput<'a> {
    input_value: &'a String,
    cursor_pos: usize
}

impl<'a> TextInput<'a> {
    pub fn new(input_value: &'a String, cursor_pos: usize) -> Self {
        TextInput {
            input_value,
            cursor_pos
        }
    }
}

impl<'a> Widget for TextInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let raw_text = self.input_value.as_str();

        // handle multiple lines
        let lines: Vec<_> = raw_text
            .split('\n')
            .collect();


            // blinking cursor
            // Span::from(String::from("|"))
            //         .style(
            //             Style::default()
            //                 .fg(Color::Yellow)
            //                 .add_modifier(Modifier::RAPID_BLINK)
            //             )

        // convert strings to Line widget
        let lines: Vec<Line<'_>> = lines.iter().map(
            |line| Line::from(*line)
        ).collect();
        
        // display as multiline paragraph
        Paragraph::new(lines)
            .wrap(Wrap {trim: true})
            .render(area, buf);
        }
}