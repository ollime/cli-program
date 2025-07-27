use ratatui::{
    style::{Stylize, Color},
    text::{Line, Span},
    widgets::{Widget, Block, Paragraph, Padding, Tabs,
        List, ListItem, ListState, Borders, Clear, Wrap,
        HighlightSpacing},
    buffer::Buffer,
    layout::{Rect, Flex},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE}
    }
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
                Constraint::Max(3), // top 2 lines for title block
                Constraint::Fill(1),
            ])
            .split(area);

        self.render_title(title_layout[0], buf);

        if (self.can_select_tab) {
            self.render_tab_list(title_layout[1], buf); // render tab titles
        }
        else {
            self.render_note(title_layout[1], buf); // render a specific note
        }
    }
}

impl App {
    fn render_tab_list(&self, area: Rect, buf: &mut Buffer) {
        const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(Color::Cyan);
        const NORMAL_ROW_BG: Color = SLATE.c950;
        const ALT_ROW_BG_COLOR: Color = SLATE.c900;
        const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
        const TEXT_FG_COLOR: Color = SLATE.c200;
        const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
       let block = Block::new()
            .title(Line::raw("Use [ and ] to navigate tabs.").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let tabs = &self.tabs;
        let current_screen_index = self.current_tab_index;
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

        ratatui::prelude::StatefulWidget::render(tabs_widget, area, buf, &mut state);
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("TUI program title")
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
                Span::from("edit mode: "),
                Span::from(format!("{}", match self.can_edit {
                    true => "on",
                    false => "off"
                })).style(match self.can_edit {
                    true => Color::Green,
                    false => Color::Red,
                }),
                "  -  ".white(),
                Span::from(format!("line: {}", self.get_line_count())),
                "  -  ".white(),
                Span::from(format!("character count: {}", self.get_character_count()))
            ]
        );
        text_display.render(title_block_area, buf);
    }

    pub fn get_character_count(&self) -> usize {
        let current_tab_index = self.current_screen as usize;
        let data = self.tab_data.get(&current_tab_index).expect("No data found for tab 0");
        data.len()
    }

    pub fn get_line_count(&self) -> usize {
        let current_tab_index = self.current_screen as usize;
        let data = self.tab_data.get(&current_tab_index).expect("No data found for tab 0");
        let data = data.split('\n');
        data.count()
    }

    fn render_note(&self, area: Rect, buf: &mut Buffer) {
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

    // renders popup area
    fn render_export_popup(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Export as file");
        let paragraph = Paragraph::new("
        Files are saved in the folder named 'data'
    \n\n[0] Styled HTML - Export as styled .html
    [1] Plain HTML - Export as .html with minimal styling
    [2] Text - Exports as .txt
    [3] Markdown - Exports as .md

    [Q] - Close prompt
            \n\nTo open a file, use the Ctrl + O command.")
            .block(block).wrap(Wrap { trim: true });
        let popup_area = App::popup_area(area, 90, 90);
        Clear.render(popup_area, buf);
        paragraph.render(popup_area, buf);
    }

    //             \n\nOPEN FILE
    // [3] Styled HTML - Opens .html file with default styles in browser
    // [4] Plain HTML - Opens .html file with minimal styling in file explorer. This file can then be modified with custom styles.


    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
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
                Constraint::Max(3),
                Constraint::Fill(1)
            ])
            .split(area);

        // render text
        let text = "Commands\n* if edit mode is off\n";
        Paragraph::new(text)
            .render(main_layout[0], buf);

        // list rendering
        let list_items: BTreeMap<&str, &str> = BTreeMap::from([
            ("[ ]", "Switch tab"),
            ("* Left/Right", "Switch tab"),
            ("* Up/Down", "Switch sidebar options"),
            ("Ctrl + E", "Change edit mode"),
            ("Ctrl + S", "Export/save"),
            ("Ctrl + O", "Open HTML file")
        ]);

        let list_items: Vec<ListItem> = list_items
            .into_iter()
            .map(|item| ListItem::new(
                Line::from(
                    vec![
                        match item.0.contains("*") {
                            true => item.0.magenta().bold(),
                            false => item.0.yellow().bold(), // using Stylize syntax
                        },
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
        let text = app.tab_data.get(&current_tab_index).expect("No text found for tab 0");
        let cursor_pos = app.cursor_pos;

        TextInput::new(text, cursor_pos)
            .render(area, buf);
    }
}