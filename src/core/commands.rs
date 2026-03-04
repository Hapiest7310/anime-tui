use crate::core::config::ProviderConfigs;
use crate::core::launcher::Launcher;
use crate::core::models::{Anime, AnimeList, SortBy};

pub fn add_anime(
    url: &str,
    name: Option<&str>,
    anime_list: &mut AnimeList,
    providers: &mut ProviderConfigs,
) -> Result<Anime, String> {
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    let (provider_name, config) = providers.get_or_create(url);
    let anime_name = name
        .map(|s| s.to_string())
        .unwrap_or_else(|| config.extract_name(url));

    let anime = anime_list.add(anime_name, url.to_string(), provider_name);
    providers.save().map_err(|e| e.to_string())?;
    anime_list.save().map_err(|e| e.to_string())?;

    Ok(anime)
}

pub fn list_anime(
    anime_list: &AnimeList,
    sort_by: &str,
    reverse: bool,
) -> Result<Vec<Anime>, String> {
    let mut anime = anime_list.anime.clone();

    let sort_field = match sort_by {
        "date" => SortBy::Date,
        "provider" => SortBy::Provider,
        _ => SortBy::Name,
    };

    match sort_field {
        SortBy::Name => {
            if reverse {
                anime.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
            } else {
                anime.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }
        }
        SortBy::Date => {
            if reverse {
                anime.sort_by(|a, b| a.added.cmp(&b.added));
            } else {
                anime.sort_by(|a, b| b.added.cmp(&a.added));
            }
        }
        SortBy::Provider => {
            if reverse {
                anime.sort_by(|a, b| b.provider.cmp(&a.provider));
            } else {
                anime.sort_by(|a, b| a.provider.cmp(&b.provider));
            }
        }
    }

    Ok(anime)
}

pub fn edit_anime(id: &str, new_name: &str, anime_list: &mut AnimeList) -> Result<Anime, String> {
    if new_name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let found_index = anime_list
        .anime
        .iter()
        .position(|a| a.id == id || a.name.to_lowercase() == id.to_lowercase());

    match found_index {
        Some(idx) => {
            anime_list.anime[idx].name = new_name.to_string();
            anime_list.sort();
            anime_list.save().map_err(|e| e.to_string())?;
            Ok(anime_list.anime[idx].clone())
        }
        None => Err(format!("Anime not found: {}", id)),
    }
}

pub fn delete_anime(id: &str, anime_list: &mut AnimeList) -> Result<Anime, String> {
    let anime_id = anime_list
        .anime
        .iter()
        .find(|a| a.id == id || a.name.to_lowercase() == id.to_lowercase())
        .map(|a| a.id.clone());

    match anime_id {
        Some(id) => {
            let deleted = anime_list.remove(&id);
            anime_list.save().map_err(|e| e.to_string())?;
            Ok(deleted)
        }
        None => Err(format!("Anime not found: {}", id)),
    }
}

pub fn watch_anime(
    id: &str,
    anime_list: &AnimeList,
    launcher: &dyn Launcher,
) -> Result<Anime, String> {
    let found = anime_list
        .anime
        .iter()
        .find(|a| a.id == id || a.name.to_lowercase() == id.to_lowercase());

    match found {
        Some(anime) => {
            launcher.launch(&anime.url).map_err(|e| e.to_string())?;
            Ok(anime.clone())
        }
        None => Err(format!("Anime not found: {}", id)),
    }
}

pub fn set_sort(
    sort_by: &str,
    reverse: bool,
    anime_list: &mut AnimeList,
) -> Result<SortBy, String> {
    let sort = match sort_by {
        "date" => SortBy::Date,
        "provider" => SortBy::Provider,
        "name" => SortBy::Name,
        _ => return Err(format!("Invalid sort field: {}", sort_by)),
    };

    anime_list.sort_by = sort;
    if reverse {
        anime_list.sort_by = match sort {
            SortBy::Name => SortBy::Name,
            SortBy::Date => SortBy::Date,
            SortBy::Provider => SortBy::Provider,
        };
    }
    anime_list.save().map_err(|e| e.to_string())?;
    Ok(anime_list.sort_by)
}
