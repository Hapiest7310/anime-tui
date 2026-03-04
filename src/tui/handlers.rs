#[cfg(feature = "tui")]
use crate::core::{config::ProviderConfigs, models::AnimeList};
#[cfg(feature = "tui")]
use crossterm::event::KeyCode;
#[cfg(feature = "tui")]
use std::process::Command;

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
            if app.selected_anime().is_some() {
                app.view = super::app::View::Edit;
                app.input.clear();
            }
        }
        KeyCode::Char('d') => {
            if let Some(anime) = app.selected_anime() {
                let id = anime.id.clone();
                anime_list.remove(&id);
                app.list = anime_list.anime.clone();
                app.table_state.select(Some(0));
                app.set_message("Anime removed");
            }
        }
        KeyCode::Char('s') => {
            app.view = super::app::View::Sort;
        }
        KeyCode::Char('w') => {
            if app.selected_anime().is_some() {
                app.view = super::app::View::Watch;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
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
                let (provider_name, config) = providers.get_or_create(&url);
                let name = config.extract_name(&url);
                anime_list.add(name, url, provider_name);
                app.list = anime_list.anime.clone();
                app.set_message("Anime added");
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
                launch_webapp(&anime.url);
                app.set_message("Launched!");
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
fn launch_webapp(url: &str) {
    let _ = Command::new("omarchy-launch-webapp").arg(url).spawn();
}
