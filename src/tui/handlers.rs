#[cfg(feature = "tui")]
use crate::core::{
    commands,
    config::ProviderConfigs,
    launcher::{Launcher, OmarchyLauncher},
    models::AnimeList,
};
#[cfg(feature = "tui")]
use crossterm::event::KeyCode;

#[cfg(feature = "tui")]
pub fn handle_key(
    key: crossterm::event::KeyEvent,
    app: &mut super::app::App,
    anime_list: &mut AnimeList,
    providers: &mut ProviderConfigs,
) -> std::io::Result<()> {
    match app.view {
        super::app::View::List => handle_list_view(app, anime_list, providers, &key)?,
        super::app::View::Add => handle_add_view(app, anime_list, providers, &key)?,
        super::app::View::Edit => handle_edit_view(app, anime_list, &key)?,
        super::app::View::Sort => handle_sort_view(app, anime_list, &key)?,
        super::app::View::Watch => handle_watch_view(app, &key)?,
        super::app::View::SelectFields => handle_select_fields(app, &key)?,
        super::app::View::DeleteConfirm => handle_delete_confirm(app, anime_list, &key)?,
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_list_view(
    app: &mut super::app::App,
    anime_list: &mut AnimeList,
    _providers: &mut ProviderConfigs,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    match key.code {
        KeyCode::Char('a') => {
            app.view = super::app::View::Add;
            app.input.clear();
        }
        KeyCode::Char('e') => {
            if !app.delete_mode && app.selected_anime().is_some() {
                app.view = super::app::View::Edit;
                app.input.clear();
            }
        }
        KeyCode::Char('d') => {
            if app.delete_mode {
                // In delete mode, 'd' does nothing (use space to select)
            } else {
                // Single delete (quick delete without confirmation)
                if let Some(anime) = app.selected_anime() {
                    let id = anime.id.clone();
                    match commands::delete_anime(&id, anime_list) {
                        Ok(_) => {
                            app.list = anime_list.anime.clone();
                            app.table_state.select(Some(0));
                            app.set_message("Anime removed");
                        }
                        Err(e) => {
                            app.set_message(format!("Error: {}", e));
                        }
                    }
                }
            }
        }
        KeyCode::Char('D') => {
            // Shift+D equivalent: Toggle delete mode
            app.delete_mode = !app.delete_mode;
            if app.delete_mode {
                app.clear_delete_selection();
                app.set_message("Delete mode enabled - Space to select, Enter to confirm");
            } else {
                app.clear_delete_selection();
                app.set_message("Delete mode disabled");
            }
        }
        KeyCode::Char('f') => {
            if !app.delete_mode {
                app.view = super::app::View::SelectFields;
                app.field_selection_state.select(Some(0));
            }
        }
        KeyCode::Char('s') => {
            if !app.delete_mode {
                app.view = super::app::View::Sort;
            }
        }
        KeyCode::Char('w') => {
            if !app.delete_mode && app.selected_anime().is_some() {
                app.view = super::app::View::Watch;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Char(' ') => {
            if app.delete_mode {
                // In delete mode, space toggles selection
                if let Some(idx) = app.table_state.selected() {
                    app.toggle_delete_selection(idx);
                }
            }
        }
        KeyCode::Enter => {
            if app.delete_mode && !app.selected_for_delete.is_empty() {
                // Show confirmation dialog
                app.view = super::app::View::DeleteConfirm;
            }
        }
        KeyCode::Esc => {
            if app.delete_mode {
                app.delete_mode = false;
                app.clear_delete_selection();
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_add_view(
    app: &mut super::app::App,
    anime_list: &mut AnimeList,
    providers: &mut ProviderConfigs,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    match key.code {
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Enter => {
            let url = app.input.trim().to_string();
            if !url.is_empty() {
                match commands::add_anime(&url, None, anime_list, providers) {
                    Ok(anime) => {
                        app.list = anime_list.anime.clone();
                        app.set_message(format!("Added: {}", anime.name));
                    }
                    Err(e) => {
                        app.set_message(format!("Error: {}", e));
                    }
                }
            }
            app.view = super::app::View::List;
            app.input.clear();
        }
        KeyCode::Esc => {
            app.view = super::app::View::List;
            app.input.clear();
        }
        _ => {}
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_edit_view(
    app: &mut super::app::App,
    anime_list: &mut AnimeList,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    match key.code {
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Enter => {
            let new_name = app.input.trim().to_string();
            if !new_name.is_empty() {
                if let Some(anime) = app.selected_anime() {
                    if let Some(a) = anime_list.anime.iter_mut().find(|a| a.id == anime.id) {
                        a.name = new_name;
                    }
                    anime_list.sort();
                    app.list = anime_list.anime.clone();
                    app.set_message("Name updated");
                }
            }
            app.view = super::app::View::List;
            app.input.clear();
        }
        KeyCode::Esc => {
            app.view = super::app::View::List;
            app.input.clear();
        }
        _ => {}
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_sort_view(
    app: &mut super::app::App,
    anime_list: &mut AnimeList,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    use crate::core::models::SortBy;

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Enter => {
            let idx = app.table_state.selected().unwrap_or(0);
            let sort_by = match idx {
                0 => SortBy::Name,
                1 => SortBy::Date,
                2 => SortBy::Provider,
                _ => SortBy::Date,
            };
            anime_list.sort_by = sort_by;
            anime_list.sort();
            app.list = anime_list.anime.clone();
            app.sort_by = sort_by;
            app.set_message(format!("Sorted by {:?}", sort_by));
            app.view = super::app::View::List;
            app.table_state.select(Some(0));
        }
        KeyCode::Esc => {
            app.view = super::app::View::List;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_watch_view(
    app: &mut super::app::App,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Some(anime) = app.selected_anime() {
                let launcher = OmarchyLauncher;
                match launcher.launch(&anime.url) {
                    Ok(_) => {
                        app.set_message("Launched!");
                    }
                    Err(e) => {
                        app.set_message(format!("Error: {}", e));
                    }
                }
            }
            app.view = super::app::View::List;
        }
        KeyCode::Esc => {
            app.view = super::app::View::List;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_select_fields(
    app: &mut super::app::App,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.field_selection_state.selected() {
                if selected > 0 {
                    app.field_selection_state.select(Some(selected - 1));
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.field_selection_state.selected() {
                if selected < app.field_options.len() - 1 {
                    app.field_selection_state.select(Some(selected + 1));
                }
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if let Some(idx) = app.field_selection_state.selected() {
                if let Some(field) = app.field_options.get(idx) {
                    app.toggle_field_selection(field.clone());
                }
            }
        }
        KeyCode::Esc => {
            app.view = super::app::View::List;
            app.set_message("Field selection updated");
        }
        _ => {}
    }
    Ok(())
}

#[cfg(feature = "tui")]
fn handle_delete_confirm(
    app: &mut super::app::App,
    anime_list: &mut AnimeList,
    key: &crossterm::event::KeyEvent,
) -> std::io::Result<()> {
    match key.code {
        KeyCode::Enter => {
            // Delete all selected anime in reverse order to avoid index shifting
            let mut indices = app.selected_for_delete.clone();
            indices.sort_by(|a, b| b.cmp(a)); // Sort in descending order

            let mut deleted_count = 0;
            for idx in indices {
                if idx < app.list.len() {
                    if let Some(anime) = app.list.get(idx) {
                        let id = anime.id.clone();
                        if commands::delete_anime(&id, anime_list).is_ok() {
                            deleted_count += 1;
                        }
                    }
                }
            }

            app.list = anime_list.anime.clone();
            app.clear_delete_selection();
            app.delete_mode = false;
            app.table_state.select(Some(0));
            app.set_message(format!("Deleted {} anime", deleted_count));
            app.view = super::app::View::List;
        }
        KeyCode::Esc => {
            app.view = super::app::View::List;
        }
        _ => {}
    }
    Ok(())
}
