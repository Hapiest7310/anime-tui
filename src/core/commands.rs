use crate::core::config::ProviderConfigs;
use crate::core::models::{AnimeList, SortBy};
use std::process::Command;

pub fn add_anime(
    url: &str,
    name: Option<&str>,
    anime_list: &mut AnimeList,
    providers: &mut ProviderConfigs,
) -> Result<String, String> {
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    let (provider_name, config) = providers.get_or_create(url);
    let anime_name = name
        .map(|s| s.to_string())
        .unwrap_or_else(|| config.extract_name(url));

    anime_list.add(anime_name, url.to_string(), provider_name);
    providers.save().map_err(|e| e.to_string())?;
    anime_list.save().map_err(|e| e.to_string())?;

    Ok("Anime added successfully".to_string())
}

pub fn list_anime(
    anime_list: &AnimeList,
    fields: &[&str],
    sort_by: &str,
    reverse: bool,
) -> Result<String, String> {
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

    if anime.is_empty() {
        return Ok("No anime in list".to_string());
    }

    let mut lines = Vec::new();
    for a in &anime {
        let mut parts = Vec::new();
        for field in fields {
            match *field {
                "name" => parts.push(a.name.clone()),
                "uuid" => parts.push(a.id.clone()),
                "provider" => parts.push(a.provider.clone()),
                "date" => parts.push(a.added.format("%Y-%m-%d").to_string()),
                _ => {}
            }
        }
        lines.push(parts.join(" | "));
    }

    Ok(lines.join("\n"))
}

pub fn edit_anime(id: &str, new_name: &str, anime_list: &mut AnimeList) -> Result<String, String> {
    if new_name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let found = anime_list
        .anime
        .iter_mut()
        .find(|a| a.id == id || a.name.to_lowercase() == id.to_lowercase());

    match found {
        Some(anime) => {
            anime.name = new_name.to_string();
            anime_list.sort();
            anime_list.save().map_err(|e| e.to_string())?;
            Ok("Anime updated successfully".to_string())
        }
        None => Err(format!("Anime not found: {}", id)),
    }
}

pub fn delete_anime(id: &str, anime_list: &mut AnimeList) -> Result<String, String> {
    let anime_id = anime_list
        .anime
        .iter()
        .find(|a| a.id == id || a.name.to_lowercase() == id.to_lowercase())
        .map(|a| a.id.clone());

    match anime_id {
        Some(id) => {
            anime_list.remove(&id);
            anime_list.save().map_err(|e| e.to_string())?;
            Ok("Anime deleted successfully".to_string())
        }
        None => Err(format!("Anime not found: {}", id)),
    }
}

pub fn watch_anime(id: &str, anime_list: &AnimeList) -> Result<String, String> {
    let found = anime_list
        .anime
        .iter()
        .find(|a| a.id == id || a.name.to_lowercase() == id.to_lowercase());

    match found {
        Some(anime) => {
            launch_webapp(&anime.url);
            Ok(format!("Launched: {}", anime.name))
        }
        None => Err(format!("Anime not found: {}", id)),
    }
}

pub fn set_sort(
    sort_by: &str,
    reverse: bool,
    anime_list: &mut AnimeList,
) -> Result<String, String> {
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
    Ok(format!(
        "Sort set to {} ({})",
        sort_by,
        if reverse { "reverse" } else { "normal" }
    ))
}

fn launch_webapp(url: &str) {
    let _ = Command::new("omarchy-launch-webapp").arg(url).spawn();
}
