use ratatui::{
    buffer::Buffer, layout::{Flex, Rect}, style::{palette::{tailwind::SLATE}, Color, Stylize}, text::{Line, Span}, widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Widget, Wrap}
};
use ratatui::prelude::*;

use crate::app::App;
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
                Constraint::Max(3), // top 2 lines for title block
                Constraint::Fill(1),
            ])
            .split(area);

        self.render_title(title_layout[0], buf);

        if self.can_select_tab {
            self.render_tab_list(title_layout[1], buf); // render tab titles
        }
        else {
            self.render_note(title_layout[1], buf); // render a specific note
        }

        if self.show_popup {
            self.render_export_popup(area, buf);
        }
        
    }
}

impl App {
    fn render_tab_list(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1), // top 3 lines for commands info
                Constraint::Fill(1),
            ])
            .split(area);
        
        const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(Color::Cyan);
        const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
       let block = Block::new()
            .title(Line::raw("Use [ and ] to navigate tabs.").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE);

        let items: Vec<ListItem> = self.tabs
            .iter()
            .map(|tab| ListItem::new(tab.tab_name.clone()))
            .collect();

        let mut state = ListState::default();
        state.select(Some(self.current_tab_index));

        // render the list
        let tabs_widget = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        ratatui::prelude::StatefulWidget::render(tabs_widget, layout[1], buf, &mut state);

        let command_info = Line::from("\nPress Spacebar to open note")
            .bold()
            .white()
            .bg(Color::Cyan)
            .centered();
        command_info.render(layout[0], buf); // render tab titles
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("skynote")
            .bold()
            .cyan()
            .centered();
        let title_block = Block::bordered()
            .title(title)
            .borders(Borders::TOP | Borders::BOTTOM)
            .padding(Padding::horizontal(1));
        let title_block_area = title_block.inner(area);
        title_block.render(area, buf);

        let text_display = Line::from(
            vec![
                Span::from("edit: "),
                Span::from(format!("{}", match self.can_edit {
                    true => "on",
                    false => "off"
                })).style(match self.can_edit {
                    true => Color::Green,
                    false => Color::Red,
                }),
                "  -  ".white(),
                Span::from(format!("lines: {}", self.get_line_count())),
                "  -  ".white(),
                Span::from(format!("character count: {}", self.get_character_count())),
                "  -  ".white(),
                Span::from(format!("{}", match self.can_update_tab_name {
                    true => "editing tab name",
                    false => ""
                })).style(match self.can_update_tab_name {
                    true => Color::Green,
                    false => Color::Red,
                }),
            ]
        );
        text_display.render(title_block_area, buf);
    }

    pub fn get_character_count(&self) -> usize {
        let data = &self.tabs[self.current_tab_index].text;
        data.len()
    }

    pub fn get_line_count(&self) -> usize {
        let data = self.tabs[self.current_tab_index].text.split('\n');
        data.clone().count()
    }

    fn render_note(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(2), // top 2 lines for title block
                Constraint::Fill(1),
                Constraint::Max(1)
            ])
            .split(area);

        let text = self.tabs[self.current_tab_index].text.clone();
        let cursor_pos = self.cursor_pos;

        let block = Block::new()
            .title(Line::raw(self.tabs[self.current_tab_index].tab_name.clone()).bold().cyan())
            .style(Style::default().bg(Color::Black));
        block.render(layout[0], buf);
        
        TextInput::new(&text, cursor_pos)
            .render(layout[1], buf);

        let text = "Press esc or CTRL + C to return to notes list";
        Line::from(text)
            .render(layout[2], buf);
    }

    // renders popup area
    fn render_export_popup(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Export as file");
        let paragraph = Paragraph::new("
        Files are saved in the folder named 'data'
    \n\n[0] Styled HTML - Export as styled .html
    [1] Plain HTML - Export as .html with minimal styling
    [2] Text - Exports as .txt

    [Q] - Close prompt
            \n\nTo open a file, use the Ctrl + O command.")
            .block(block).wrap(Wrap { trim: true });
        let popup_area = App::popup_area(area, 90, 90);
        Clear.render(popup_area, buf);
        paragraph.render(popup_area, buf);
    }

    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}