use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub id: String,
    pub name: String,
    pub url: String,
    pub provider: String,
    pub added: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimeList {
    pub anime: Vec<Anime>,
    #[serde(default)]
    pub sort_by: SortBy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Name,
    #[default]
    Date,
    Provider,
}

impl SortBy {
    #[allow(dead_code)]
    pub fn next(&self) -> Self {
        match self {
            SortBy::Name => SortBy::Date,
            SortBy::Date => SortBy::Provider,
            SortBy::Provider => SortBy::Name,
        }
    }
}

impl AnimeList {
    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    fn path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("anime")
            .join("anime.json")
    }

    pub fn add(&mut self, name: String, url: String, provider: String) {
        let anime = Anime {
            id: Uuid::new_v4().to_string(),
            name,
            url,
            provider,
            added: Utc::now(),
        };
        self.anime.push(anime);
        self.sort();
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let len_before = self.anime.len();
        self.anime.retain(|a| a.id != id);
        self.anime.len() < len_before
    }

    pub fn sort(&mut self) {
        match self.sort_by {
            SortBy::Name => self
                .anime
                .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortBy::Date => self.anime.sort_by(|a, b| b.added.cmp(&a.added)),
            SortBy::Provider => self.anime.sort_by(|a, b| a.provider.cmp(&b.provider)),
        }
    }

    #[allow(dead_code)]
    pub fn toggle_sort(&mut self) {
        self.sort_by = self.sort_by.next();
        self.sort();
    }
}
