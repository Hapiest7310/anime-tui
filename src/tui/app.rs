#[cfg(feature = "tui")]
use crate::core::models::SortBy;
#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Row, Table, TableState},
    Frame,
};

#[cfg(feature = "tui")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    List,
    Add,
    Edit,
    Sort,
    Watch,
}

#[cfg(feature = "tui")]
pub struct App {
    pub view: View,
    pub list: Vec<crate::core::models::Anime>,
    pub sort_by: SortBy,
    pub table_state: TableState,
    pub input: String,
    pub message: Option<String>,
    #[allow(dead_code)]
    pub providers: Vec<String>,
}

#[cfg(feature = "tui")]
impl App {
    pub fn new(
        list: Vec<crate::core::models::Anime>,
        sort_by: SortBy,
        providers: Vec<String>,
    ) -> Self {
        Self {
            view: View::List,
            list,
            sort_by,
            table_state: TableState::default().with_selected(Some(0)),
            input: String::new(),
            message: None,
            providers,
        }
    }

    pub fn next(&mut self) {
        let len = self.list.len();
        if len == 0 {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let len = self.list.len();
        if len == 0 {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => (i + len - 1) % len,
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn selected_anime(&self) -> Option<&crate::core::models::Anime> {
        self.table_state.selected().and_then(|i| self.list.get(i))
    }

    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }
}

#[cfg(feature = "tui")]
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);
    render_body(frame, app, chunks[1]);
    render_footer(frame, app, chunks[2]);
}

#[cfg(feature = "tui")]
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let title = match app.view {
        View::List => " Anime List ",
        View::Add => " Add Anime ",
        View::Edit => " Edit Name ",
        View::Sort => " Sort By ",
        View::Watch => " Watch ",
    };

    let style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let block = Block::default()
        .title(title)
        .style(style)
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

#[cfg(feature = "tui")]
fn render_body(frame: &mut Frame, app: &App, area: Rect) {
    match app.view {
        View::List => render_list(frame, app, area),
        View::Add => render_add(frame, app, area),
        View::Edit => render_edit(frame, app, area),
        View::Sort => render_sort(frame, app, area),
        View::Watch => render_watch(frame, app, area),
    }
}

#[cfg(feature = "tui")]
fn render_list(frame: &mut Frame, app: &App, area: Rect) {
    if app.list.is_empty() {
        let text = Paragraph::new("No anime yet. Press 'a' to add one.")
            .centered()
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(text, area);
        return;
    }

    let rows: Vec<Row> = app
        .list
        .iter()
        .map(|anime| {
            let date = anime.added.format("%Y-%m-%d").to_string();
            Row::new(vec![anime.name.clone(), anime.provider.clone(), date])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(30),
            Constraint::Length(12),
            Constraint::Length(12),
        ],
    )
    .block(Block::default().borders(Borders::NONE))
    .header(
        Row::new(vec!["Name", "Provider", "Added"]).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .row_highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::REVERSED),
    );

    let mut state = app.table_state.clone();
    frame.render_stateful_widget(table, area, &mut state);
}

#[cfg(feature = "tui")]
fn render_add(frame: &mut Frame, app: &App, area: Rect) {
    let text = vec![
        Line::from("Enter the anime URL:"),
        Line::from(""),
        Line::from(Span::raw(format!("{}▌", app.input))),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to confirm, Esc to cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().title(" Add Anime ").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

#[cfg(feature = "tui")]
fn render_edit(frame: &mut Frame, app: &App, area: Rect) {
    let anime = app.selected_anime();
    let current_name = anime.map(|a| a.name.as_str()).unwrap_or("");

    let text = vec![
        Line::from(format!("Current name: {}", current_name)),
        Line::from(""),
        Line::from("Enter new name:"),
        Line::from(""),
        Line::from(Span::raw(format!("{}▌", app.input))),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to confirm, Esc to cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().title(" Edit Name ").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

#[cfg(feature = "tui")]
fn render_sort(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = vec![
        ListItem::new("Name"),
        ListItem::new("Date"),
        ListItem::new("Provider"),
    ];

    let mut list_state = ListState::default();
    if let Some(idx) = app.table_state.selected() {
        list_state.select(Some(idx));
    }

    let list = List::new(items)
        .block(Block::default().title(" Sort By ").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::REVERSED),
        );

    frame.render_stateful_widget(list, area, &mut list_state);
}

#[cfg(feature = "tui")]
fn render_watch(frame: &mut Frame, app: &App, area: Rect) {
    let anime = app.selected_anime();
    let url = anime.map(|a| a.url.as_str()).unwrap_or("");

    let text = vec![
        Line::from("Ready to watch:"),
        Line::from(""),
        Line::from(Span::styled(
            anime.map(|a| a.name.as_str()).unwrap_or("None"),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(Span::raw(url)),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to launch, Esc to cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().title(" Watch ").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

#[cfg(feature = "tui")]
fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let (left, right) = {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(40)])
            .split(area);
        (chunks[0], chunks[1])
    };

    let help_text = match app.view {
        View::List => "↑↓: select | a: add | e: edit | d: delete | s: sort | w: watch | q: quit",
        View::Add => "Enter URL | Esc: cancel",
        View::Edit => "Enter name | Esc: cancel",
        View::Sort => "↑↓: select | Enter: confirm | Esc: cancel",
        View::Watch => "Enter: launch | Esc: cancel",
    };

    let left_text = if let Some(ref msg) = app.message {
        Span::styled(msg.clone(), Style::default().fg(Color::Green))
    } else {
        Span::styled(help_text, Style::default().fg(Color::DarkGray))
    };

    let left_block = Paragraph::new(left_text).block(Block::default());
    frame.render_widget(left_block, left);

    let sort_text = format!(" Sort: {:?} ", app.sort_by);
    let right_block = Block::default()
        .title(sort_text.as_str())
        .style(Style::default().fg(Color::Cyan))
        .borders(Borders::ALL);
    frame.render_widget(right_block, right);
}
