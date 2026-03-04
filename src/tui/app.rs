#[cfg(feature = "tui")]
use super::theme::Theme;
#[cfg(feature = "tui")]
use crate::core::models::SortBy;
#[cfg(feature = "tui")]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
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
    SelectFields,
    DeleteConfirm,
}

#[cfg(feature = "tui")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Name,
    Uuid,
    Provider,
    Date,
    Url,
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
    pub theme: Theme,
    pub selected_fields: Vec<Field>,
    pub field_options: Vec<Field>,
    pub field_selection_state: ListState,
    pub selected_for_delete: Vec<usize>,
    pub delete_mode: bool,
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
            theme: Theme::detect(),
            selected_fields: vec![Field::Name, Field::Provider, Field::Date],
            field_options: vec![
                Field::Name,
                Field::Uuid,
                Field::Provider,
                Field::Date,
                Field::Url,
            ],
            field_selection_state: ListState::default().with_selected(Some(0)),
            selected_for_delete: Vec::new(),
            delete_mode: false,
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

    pub fn toggle_field_selection(&mut self, field: Field) {
        if self.selected_fields.contains(&field) {
            self.selected_fields.retain(|f| f != &field);
        } else {
            self.selected_fields.push(field);
        }
    }

    pub fn toggle_delete_selection(&mut self, index: usize) {
        if self.selected_for_delete.contains(&index) {
            self.selected_for_delete.retain(|i| i != &index);
        } else {
            self.selected_for_delete.push(index);
        }
    }

    pub fn clear_delete_selection(&mut self) {
        self.selected_for_delete.clear();
        self.delete_mode = false;
    }

    pub fn get_field_value(&self, anime: &crate::core::models::Anime, field: &Field) -> String {
        match field {
            Field::Name => anime.name.clone(),
            Field::Uuid => anime.id.clone(),
            Field::Provider => anime.provider.clone(),
            Field::Date => anime.added.format("%Y-%m-%d").to_string(),
            Field::Url => anime.url.clone(),
        }
    }

    pub fn get_field_header(&self, field: &Field) -> &'static str {
        match field {
            Field::Name => "Name",
            Field::Uuid => "UUID",
            Field::Provider => "Provider",
            Field::Date => "Added",
            Field::Url => "URL",
        }
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
        View::List => {
            if app.delete_mode {
                " Select Anime to Delete "
            } else {
                " Anime List "
            }
        }
        View::Add => " Add Anime ",
        View::Edit => " Edit Name ",
        View::Sort => " Sort By ",
        View::Watch => " Watch ",
        View::SelectFields => " Select Fields ",
        View::DeleteConfirm => " Confirm Delete ",
    };

    let block = Block::default()
        .title(title)
        .style(app.theme.header())
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
        View::SelectFields => render_select_fields(frame, app, area),
        View::DeleteConfirm => render_delete_confirm(frame, app, area),
    }
}

#[cfg(feature = "tui")]
fn render_list(frame: &mut Frame, app: &App, area: Rect) {
    if app.list.is_empty() {
        let text = Paragraph::new("No anime yet. Press 'a' to add one.")
            .centered()
            .style(app.theme.hint_text());
        frame.render_widget(text, area);
        return;
    }

    let rows: Vec<Row> = app
        .list
        .iter()
        .enumerate()
        .map(|(idx, anime)| {
            let values: Vec<String> = app
                .selected_fields
                .iter()
                .map(|field| app.get_field_value(anime, field))
                .collect();

            // Add checkbox for delete mode
            let checkbox = if app.delete_mode {
                if app.selected_for_delete.contains(&idx) {
                    "[✓] "
                } else {
                    "[ ] "
                }
            } else {
                ""
            };

            let row_values = if app.delete_mode {
                std::iter::once(checkbox.to_string())
                    .chain(values)
                    .collect::<Vec<_>>()
            } else {
                values
            };

            Row::new(row_values)
        })
        .collect();

    let mut constraints = if app.delete_mode {
        vec![Constraint::Length(4)]
    } else {
        vec![]
    };

    constraints.extend(
        app.selected_fields
            .iter()
            .map(|field| match field {
                Field::Name => Constraint::Min(30),
                Field::Uuid => Constraint::Length(38),
                Field::Provider => Constraint::Length(12),
                Field::Date => Constraint::Length(12),
                Field::Url => Constraint::Min(20),
            })
            .collect::<Vec<_>>(),
    );

    let mut headers: Vec<&str> = if app.delete_mode { vec![""] } else { vec![] };

    headers.extend(
        app.selected_fields
            .iter()
            .map(|field| app.get_field_header(field))
            .collect::<Vec<_>>(),
    );

    let table = Table::new(rows, constraints)
        .block(Block::default().borders(Borders::NONE))
        .header(Row::new(headers).style(app.theme.header()))
        .row_highlight_style(app.theme.selected());

    let mut state = app.table_state;
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
            app.theme.hint_text(),
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
            app.theme.hint_text(),
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
        .highlight_style(app.theme.selected());

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
            app.theme.emphasized(),
        )),
        Line::from(""),
        Line::from(Span::raw(url)),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to launch, Esc to cancel",
            app.theme.hint_text(),
        )),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().title(" Watch ").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

#[cfg(feature = "tui")]
fn render_select_fields(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .field_options
        .iter()
        .map(|field| {
            let is_selected = app.selected_fields.contains(field);
            let checkbox = if is_selected { "[✓]" } else { "[ ]" };
            let field_name = app.get_field_header(field);
            ListItem::new(format!("{} {}", checkbox, field_name))
        })
        .collect();

    let mut list_state = app.field_selection_state;

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Select Fields to Display ")
                .borders(Borders::ALL),
        )
        .highlight_style(app.theme.selected());

    frame.render_stateful_widget(list, area, &mut list_state);
}

#[cfg(feature = "tui")]
fn render_delete_confirm(frame: &mut Frame, app: &App, area: Rect) {
    let count = app.selected_for_delete.len();
    let anime_names: Vec<String> = app
        .selected_for_delete
        .iter()
        .filter_map(|idx| app.list.get(*idx).map(|a| a.name.clone()))
        .collect();

    let text: Vec<Line> = vec![
        Line::from(format!("Delete {} anime(s)?", count)),
        Line::from(""),
        Line::from("Selected:"),
    ]
    .into_iter()
    .chain(
        anime_names
            .iter()
            .map(|name| Line::from(format!("  • {}", name))),
    )
    .chain(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to confirm, Esc to cancel",
            app.theme.hint_text(),
        )),
    ])
    .collect();

    let p = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Confirm Delete ")
                .borders(Borders::ALL),
        )
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
        View::List => {
            if app.delete_mode {
                "Space: select | Enter: confirm | Esc: cancel | ↑↓: navigate"
            } else {
                "↑↓: select | a: add | e: edit | d: delete | s: sort | w: watch | f: fields | Shift+D: multi-delete | q: quit"
            }
        }
        View::Add => "Enter URL | Esc: cancel",
        View::Edit => "Enter name | Esc: cancel",
        View::Sort => "↑↓: select | Enter: confirm | Esc: cancel",
        View::Watch => "Enter: launch | Esc: cancel",
        View::SelectFields => "↑↓: navigate | Space/Enter: toggle | Esc: done",
        View::DeleteConfirm => "Enter: confirm | Esc: cancel",
    };

    let left_text = if let Some(ref msg) = app.message {
        Span::styled(msg.clone(), app.theme.success_message())
    } else {
        Span::styled(help_text, app.theme.hint_text())
    };

    let left_block = Paragraph::new(left_text).block(Block::default());
    frame.render_widget(left_block, left);

    let sort_text = format!(" Sort: {:?} ", app.sort_by);
    let right_block = Block::default()
        .title(sort_text.as_str())
        .style(app.theme.header())
        .borders(Borders::ALL);
    frame.render_widget(right_block, right);
}
