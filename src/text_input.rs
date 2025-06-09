use ratatui::{
    text::{Line, Span},
    widgets::{Widget, Paragraph, Wrap},
    buffer::Buffer,
    layout::Rect,
    style::{Style, Modifier, Color},
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
        let cursor_pos = self.cursor_pos;

        // Split into lines and build styled lines with cursor
        let mut lines: Vec<Line<'_>> = vec![];
        let mut char_count = 0;
        for line in raw_text.split('\n') {
            let line_len = line.chars().count();
            let mut spans = vec![];
            // If cursor is on this line
            if cursor_pos >= char_count && cursor_pos <= char_count + line_len {
                let rel_cursor = cursor_pos - char_count;
                let (before, after) = line.split_at(rel_cursor);
                spans.push(Span::from(before));
                spans.push(
                    Span::from("|").style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::RAPID_BLINK)
                    )
                );
                spans.push(Span::from(after));
            } else {
                spans.push(Span::from(line));
            }
            lines.push(Line::from(spans));
            char_count += line_len + 1; // +1 for the '\n'
        }
        Paragraph::new(lines)
            .wrap(Wrap {trim: true})
            .render(area, buf);
    }
}