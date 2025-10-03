//! Started off from https://github.com/ratatui/ratatui/blob/2b0a044cedfc3f58c99ef8ac21f83d20432c2144/examples/apps/todo-list/src/main.rs

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, StatefulWidget,
    },
};

use crate::git_data::GitData;

const ITEM_HEIGHT: usize = 4;
const INFO_TEXT: [&str; 2] = [
    "(↑/k) move up | (↓/j) move down | g/G to go top/bottom | (Enter) view diff",
    "(Esc/q) quit",
];

const HEADER_STYLE: Style = Style::new().fg(Color::Black).bg(Color::DarkGray);
const FOOTER_STYLE: Style = Style::new().fg(Color::Black).bg(Color::DarkGray);
const ROW_STYLE: Style = Style::new().fg(Color::DarkGray).bg(Color::Black);
const SELECTED_ROW_STYLE: Style = Style::new()
    .fg(Color::Black)
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct App {
    state: ListState,
    scroll_state: ScrollbarState,
    base_path: String,
    items: Vec<(String, GitData)>,
}

impl App {
    pub fn new(base_path: String, git_data: Vec<(String, GitData)>) -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            scroll_state: ScrollbarState::new((git_data.len() - 1) * ITEM_HEIGHT),
            base_path,
            items: git_data,
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn select_first(&mut self) {
        self.state.select_first();
    }

    fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                    KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                    KeyCode::Char('G') | KeyCode::End => self.select_last(),
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame<'_>) {
        let rect = frame.area();
        let layout = Layout::vertical([
            Constraint::Length(3), // Header
            Constraint::Fill(1),   // Content
            Constraint::Length(4), // Footer
        ])
        .split(rect);

        let diff_window = Layout::horizontal([Constraint::Fill(1)]).split(layout[1]);
        let diff_window_inner = Layout::vertical([
            Constraint::Length(3), // Diff window header
            Constraint::Fill(1),   // Diff content
        ])
        .split(diff_window[0]);

        self.render_header(frame, layout[0]);
        self.render_list(frame, layout[1]);
        // self.render_diff_window(frame, diff_window_inner, buffer);
        self.render_scrollbar(frame, layout[1]);
        self.render_footer(frame, layout[2]);
    }

    fn render_header(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let header = Paragraph::new(format!("Viewing git repos in {}", self.base_path))
            .style(HEADER_STYLE)
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(Color::Gray)),
            );

        frame.render_widget(header, area);
    }

    fn render_list(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let list_items = self
            .items
            .iter()
            .map(|(repo_path, git_data)| {
                let mut text = Text::raw(repo_path);
                if git_data.status.contains("nothing to commit") {
                    text.push_span(Span::styled(
                        " .. CLEAN",
                        Style::new().fg(Color::Green).add_modifier(Modifier::ITALIC),
                    ))
                } else if git_data.status.contains("no changes added to commit") {
                    text.push_span(Span::styled(
                        " .. DIRTY (changes not added)",
                        Style::new().fg(Color::Rgb(255, 184, 108)), // orange
                    ))
                } else if git_data.status.contains("Changes to be committed") {
                    text.push_span(Span::styled(
                        " .. DIRTY (changes added, not committed)",
                        Style::new().fg(Color::Red),
                    ))
                } else if git_data.status.contains("Your branch is ahead of") {
                    text.push_span(Span::styled(
                        " .. DIRTY (changes committed, not pushed)",
                        Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ))
                } else {
                    text.push_span(Span::styled(" .. UNKNOWN", Style::new().fg(Color::Yellow)))
                };

                ListItem::new(text)
            })
            .collect::<Vec<_>>();

        let list = List::new(list_items)
            .style(ROW_STYLE)
            .highlight_style(SELECTED_ROW_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.state);
    }

    // fn render_diff_window(&mut self, frame: &mut Frame<'_>, area: Rect) {
    //     // We get the info depending on the item's state.
    //     let info = if let Some(i) = self.todo_list.state.selected() {
    //         match self.todo_list.items[i].status {
    //             Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
    //             Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
    //         }
    //     } else {
    //         "Nothing selected...".to_string()
    //     };

    //     // We show the list item's info under the list in this paragraph
    //     let block = Block::new()
    //         .title(Line::raw("TODO Info").centered())
    //         .borders(Borders::TOP)
    //         .border_set(symbols::border::EMPTY)
    //         .border_style(TODO_HEADER_STYLE)
    //         .bg(NORMAL_ROW_BG)
    //         .padding(Padding::horizontal(1));

    //     // We can now render the item info
    //     Paragraph::new(info)
    //         .block(block)
    //         .fg(TEXT_FG_COLOR)
    //         .wrap(Wrap { trim: false })
    //         .render(area, frame.buffer_mut());
    // }

    fn render_scrollbar(&mut self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(FOOTER_STYLE)
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(Color::Gray)),
            );

        frame.render_widget(info_footer, area);
    }
}
