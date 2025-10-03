//! Started off from https://github.com/ratatui/ratatui/blob/2b0a044cedfc3f58c99ef8ac21f83d20432c2144/examples/apps/todo-list/src/main.rs

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph, StatefulWidget, Widget, Wrap,
    },
};

use crate::git_data::GitData;

const KEYBINDS: [&str; 2] = [
    "(↑/k) move up | (↓/j) move down | (←/h) move left | (→/l) move right",
    "g/G to go top/bottom | (Esc/q) quit",
];

#[derive(Debug)]
pub struct App {
    state: ListState,
    diff_scroll: u16,
    max_diff_scroll: u16,
    focused_window: FocusedWindow,
    base_path: String,
    items: Vec<(String, GitData)>,
}

#[derive(Debug, PartialEq)]
enum FocusedWindow {
    PathList,
    DiffPreview,
}

impl App {
    pub fn new(base_path: String, git_data: Vec<(String, GitData)>) -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            diff_scroll: 0,
            max_diff_scroll: 0,
            focused_window: FocusedWindow::PathList,
            base_path,
            items: git_data,
        }
    }

    fn select_next(&mut self) {
        match self.focused_window {
            FocusedWindow::PathList => {
                self.state.select_next();
                self.diff_scroll = 0; // Reset diff scroll when changing selection
            }
            FocusedWindow::DiffPreview => {
                self.diff_scroll = self.diff_scroll.saturating_add(1);
            }
        }
    }

    fn select_previous(&mut self) {
        match self.focused_window {
            FocusedWindow::PathList => {
                self.state.select_previous();
                self.diff_scroll = 0;
            }
            FocusedWindow::DiffPreview => {
                self.diff_scroll = self.diff_scroll.saturating_sub(1);
            }
        }
    }

    fn select_first(&mut self) {
        match self.focused_window {
            FocusedWindow::PathList => {
                self.state.select_first();
                self.diff_scroll = 0;
            }
            FocusedWindow::DiffPreview => {
                self.diff_scroll = 0;
            }
        }
    }

    fn select_last(&mut self) {
        match self.focused_window {
            FocusedWindow::PathList => {
                self.state.select_last();
                self.diff_scroll = 0;
            }
            FocusedWindow::DiffPreview => {
                self.diff_scroll = self.max_diff_scroll;
            }
        }
    }

    fn hover_path_list(&mut self) {
        self.focused_window = FocusedWindow::PathList;
    }

    fn hover_diff_preview(&mut self) {
        self.focused_window = FocusedWindow::DiffPreview;
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(event) = event::read()?
                && let KeyEventKind::Press = event.kind
            {
                // Skip to line number with :?
                match event.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                    KeyCode::Char('h') | KeyCode::Left => self.hover_path_list(),
                    KeyCode::Char('l') | KeyCode::Right => self.hover_diff_preview(),
                    KeyCode::Char('g') | KeyCode::PageUp => self.select_first(),
                    KeyCode::Char('G') | KeyCode::PageDown => self.select_last(),
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

        let list_layout = Layout::horizontal([
            Constraint::Percentage(50), // Path list
            Constraint::Percentage(50), // Diff preview
        ])
        .split(layout[1]);

        self.render_header(frame, layout[0]);
        self.render_list(frame, list_layout[0]);
        self.render_diff_window(frame, list_layout[1]);
        self.render_footer(frame, layout[2]);
    }

    fn render_header(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let header = Paragraph::new(format!("Viewing git repos in {}", self.base_path))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))
            .wrap(Wrap { trim: false });

        frame.render_widget(header, area);
    }

    fn render_list(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let list_items = self
            .items
            .iter()
            .map(|(repo_path, git_data)| {
                let mut text = Text::raw(format!("{repo_path} .. "));
                if git_data.status.contains("nothing to commit")
                    && !git_data.status.contains("branch is ahead")
                {
                    text.push_span(Span::styled(
                        "CLEAN",
                        Style::new().fg(Color::Green).add_modifier(Modifier::ITALIC),
                    ))
                } else if git_data.status.contains("no changes added to commit") {
                    text.push_span(Span::styled(
                        "DIRTY (changes not added)",
                        Style::new().fg(Color::Rgb(255, 184, 108)), // orange
                    ))
                } else if git_data.status.contains("Changes to be committed") {
                    text.push_span(Span::styled(
                        "DIRTY (changes added, not committed)",
                        Style::new().fg(Color::Red),
                    ))
                } else if git_data.status.contains("branch is ahead")
                    || git_data.status.contains("diverged")
                {
                    text.push_span(Span::styled(
                        "DIRTY (changes committed, not pushed)",
                        Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ))
                } else {
                    text.push_span(Span::styled("UNKNOWN", Style::new().fg(Color::Yellow)))
                };

                ListItem::new(text)
            })
            .collect::<Vec<_>>();

        let list = List::new(list_items)
            .highlight_style(match self.focused_window {
                FocusedWindow::PathList => Style::new().add_modifier(Modifier::DIM),
                FocusedWindow::DiffPreview => Style::new(),
            })
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.state);
    }

    fn render_diff_window(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let diff_str = if let Some(i) = self.state.selected() {
            self.max_diff_scroll = self.items[i].1.diff.lines().count() as u16;
            &self.items[i].1.diff
        } else {
            "Nothing selected..."
        };

        let colored_diff = {
            let lines: Vec<Line<'_>> = diff_str
                .lines()
                .map(|line| {
                    let style = match line.chars().next() {
                        Some('+') => Style::default().fg(Color::Green),
                        Some('-') => Style::default().fg(Color::Red),
                        Some('@') => Style::default().add_modifier(Modifier::ITALIC),
                        Some('d') if line.starts_with("diff --git") => {
                            Style::default().add_modifier(Modifier::BOLD)
                        }
                        Some('i') if line.starts_with("index") => {
                            Style::default().add_modifier(Modifier::BOLD)
                        }
                        _ => Style::default(),
                    };

                    Line::from(Span::styled(line.to_string(), style))
                })
                .collect();

            Text::from(lines)
        };

        Paragraph::new(colored_diff)
            .block(
                Block::new()
                    .title(Line::raw(" Diff Preview ").centered().add_modifier(
                        match self.focused_window {
                            FocusedWindow::PathList => Modifier::HIDDEN,
                            FocusedWindow::DiffPreview => Modifier::BOLD,
                        },
                    ))
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .padding(Padding::horizontal(1)),
            )
            .scroll((self.diff_scroll, 0))
            .wrap(Wrap { trim: false })
            .render(area, frame.buffer_mut());
    }

    fn render_footer(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let footer = Paragraph::new(Text::from_iter(KEYBINDS))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded));

        frame.render_widget(footer, area);
    }
}
