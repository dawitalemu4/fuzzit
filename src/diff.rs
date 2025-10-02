// use std::{collections::HashMap, path::PathBuf};

// use color_eyre::eyre::Result;
// use ratatui::{
//     DefaultTerminal, Frame,
//     crossterm::event::{KeyCode, KeyModifiers},
//     layout::{Constraint, Layout, Margin, Rect},
//     style::{self, Color, Modifier, Style, Stylize},
//     text::Text,
//     widgets::{
//         Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
//         ScrollbarState, Table, TableState,
//     },
// };

// use crate::git_data::GitData;

// const ITEM_HEIGHT: usize = 4;
// const INFO_TEXT: [&str; 1] =
//     ["(Esc) quit | (↑)/k move up | (↓)/j move down | (←)/h move left | (→)/l move right"];

// pub struct App {
//     state: TableState,
//     items: HashMap<String, GitData>,
//     longest_item_lens: (u16, u16, u16), // order is (name, address, email)
//     scroll_state: ScrollbarState,
//     colors: TableColors,
//     color_index: usize,
// }

// impl App {
//     pub fn new(base_path: PathBuf, git_data: HashMap<PathBuf, GitData>) -> Self {
//         Self {
//             state: TableState::default().with_selected(0),
//             longest_item_lens: constraint_len_calculator(&data_vec),
//             scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
//             colors: TableColors::new(&PALETTES[0]),
//             color_index: 0,
//             items: git_data,
//         }
//     }

//     pub fn next_row(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.items.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//         self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
//     }

//     pub fn previous_row(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.items.len() - 1
//                 } else {
//                     i - 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//         self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
//     }

//     pub fn next_column(&mut self) {
//         self.state.select_next_column();
//     }

//     pub fn previous_column(&mut self) {
//         self.state.select_previous_column();
//     }

//     pub const fn next_color(&mut self) {
//         self.color_index = (self.color_index + 1) % PALETTES.len();
//     }

//     pub const fn previous_color(&mut self) {
//         let count = PALETTES.len();
//         self.color_index = (self.color_index + count - 1) % count;
//     }

//     pub const fn set_colors(&mut self) {
//         self.colors = TableColors::new(&PALETTES[self.color_index]);
//     }

//     pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
//         loop {
//             terminal.draw(|frame| self.render(frame))?;

//             if let Some(key) = event::read()?.as_key_press_event() {
//                 let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
//                 match key.code {
//                     KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
//                     KeyCode::Char('j') | KeyCode::Down => self.next_row(),
//                     KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
//                     KeyCode::Char('l') | KeyCode::Right if shift_pressed => self.next_color(),
//                     KeyCode::Char('h') | KeyCode::Left if shift_pressed => {
//                         self.previous_color();
//                     }
//                     KeyCode::Char('l') | KeyCode::Right => self.next_column(),
//                     KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
//                     _ => {}
//                 }
//             }
//         }
//     }

//     fn render(&mut self, frame: &mut Frame) {
//         let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
//         let rects = frame.area().layout_vec(&layout);

//         self.set_colors();

//         self.render_table(frame, rects[0]);
//         self.render_scrollbar(frame, rects[0]);
//         self.render_footer(frame, rects[1]);
//     }

//     fn render_table(&mut self, frame: &mut Frame, area: Rect) {
//         let header_style = Style::default()
//             .fg(self.colors.header_fg)
//             .bg(self.colors.header_bg);
//         let selected_row_style = Style::default()
//             .add_modifier(Modifier::REVERSED)
//             .fg(self.colors.selected_row_style_fg);
//         let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
//         let selected_cell_style = Style::default()
//             .add_modifier(Modifier::REVERSED)
//             .fg(self.colors.selected_cell_style_fg);

//         let header = ["Name", "Address", "Email"]
//             .into_iter()
//             .map(Cell::from)
//             .collect::<Row>()
//             .style(header_style)
//             .height(1);
//         let rows = self.items.iter().enumerate().map(|(i, data)| {
//             let color = match i % 2 {
//                 0 => self.colors.normal_row_color,
//                 _ => self.colors.alt_row_color,
//             };
//             let item = data.ref_array();
//             item.into_iter()
//                 .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
//                 .collect::<Row>()
//                 .style(Style::new().fg(self.colors.row_fg).bg(color))
//                 .height(4)
//         });
//         let bar = " █ ";
//         let t = Table::new(
//             rows,
//             [
//                 // + 1 is for padding.
//                 Constraint::Length(self.longest_item_lens.0 + 1),
//                 Constraint::Min(self.longest_item_lens.1 + 1),
//                 Constraint::Min(self.longest_item_lens.2),
//             ],
//         )
//         .header(header)
//         .row_highlight_style(selected_row_style)
//         .column_highlight_style(selected_col_style)
//         .cell_highlight_style(selected_cell_style)
//         .highlight_symbol(Text::from(vec![
//             "".into(),
//             bar.into(),
//             bar.into(),
//             "".into(),
//         ]))
//         .bg(self.colors.buffer_bg)
//         .highlight_spacing(HighlightSpacing::Always);
//         frame.render_stateful_widget(t, area, &mut self.state);
//     }

//     fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
//         frame.render_stateful_widget(
//             Scrollbar::default()
//                 .orientation(ScrollbarOrientation::VerticalRight)
//                 .begin_symbol(None)
//                 .end_symbol(None),
//             area.inner(Margin {
//                 vertical: 1,
//                 horizontal: 1,
//             }),
//             &mut self.scroll_state,
//         );
//     }

//     fn render_footer(&self, frame: &mut Frame, area: Rect) {
//         let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
//             .style(
//                 Style::new()
//                     .fg(self.colors.row_fg)
//                     .bg(self.colors.buffer_bg),
//             )
//             .centered()
//             .block(
//                 Block::bordered()
//                     .border_type(BorderType::Double)
//                     .border_style(Style::new().fg(self.colors.footer_border_color)),
//             );
//         frame.render_widget(info_footer, area);
//     }
// }

// pub async fn display(base_path: PathBuf, git_data: HashMap<PathBuf, GitData>) -> Result<()> {
//     if let Some(base_path) = base_path.to_str() {
//         println!("Iterating git repos from {base_path}\n");
//     }

//     for (path, git_data) in git_data {
//         if let Some(repo_path) = path.to_str() {
//             if git_data.status.contains("nothing to commit") {
//                 println!("{repo_path} .. {}", "CLEAN".green().italic());
//             } else if git_data.status.contains("no changes added to commit") {
//                 println!(
//                     "{repo_path} .. {} (changes not added)",
//                     "DIRTY".fg_rgb::<255, 184, 108>()
//                 ); // orange
//             } else if git_data.status.contains("Changes to be committed") {
//                 println!(
//                     "{repo_path} .. {} (changes added, not committed)",
//                     "DIRTY".red()
//                 );
//             } else if git_data.status.contains("Your branch is ahead of") {
//                 println!(
//                     "{repo_path} .. {} (changes committed, not pushed)",
//                     "DIRTY".red().bold()
//                 );
//             } else {
//                 println!("{repo_path} .. {}", "UNKNOWN".yellow());
//             }
//         }
//     }

//     Ok(())
// }
