use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    pub domain: String,
    #[serde(default)]
    pub strip_pattern: String,
    #[serde(default = "default_format")]
    pub display_format: DisplayFormat,
}

fn default_format() -> DisplayFormat {
    DisplayFormat::Hyphen
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayFormat {
    #[default]
    Hyphen,
    Slash,
    CamelCase,
    Original,
}

impl ProviderConfig {
    pub fn new(domain: &str) -> Self {
        Self {
            domain: domain.to_string(),
            strip_pattern: "[0-9]+-".to_string(),
            display_format: DisplayFormat::Hyphen,
        }
    }

    pub fn extract_name(&self, url: &str) -> String {
        let path = url
            .strip_prefix("https://")
            .or_else(|| url.strip_prefix("http://"))
            .unwrap_or(url);

        let path = path.split('/').nth(1).unwrap_or("");

        let name = if self.strip_pattern.is_empty() {
            path.to_string()
        } else {
            let re = regex::Regex::new(&self.strip_pattern)
                .unwrap_or_else(|_| regex::Regex::new("[0-9]+-").unwrap());
            re.replace(path, "").to_string()
        };

        if let Some(idx) = name.find(".html") {
            let name = &name[..idx];
            match self.display_format {
                DisplayFormat::Slash => name.replace('-', "/"),
                DisplayFormat::CamelCase => name
                    .split('-')
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    })
                    .collect(),
                DisplayFormat::Original => path.to_string(),
                DisplayFormat::Hyphen => name.to_string(),
            }
        } else {
            match self.display_format {
                DisplayFormat::Slash => name.replace('-', "/"),
                DisplayFormat::CamelCase => name
                    .split('-')
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    })
                    .collect(),
                DisplayFormat::Original => path.to_string(),
                DisplayFormat::Hyphen => name.to_string(),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfigs {
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
}

impl ProviderConfigs {
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
            .join("providers.json")
    }

    pub fn get_or_create(&mut self, url: &str) -> (String, &ProviderConfig) {
        let domain = extract_domain(url);
        let provider_name = domain.split('.').next().unwrap_or(&domain).to_string();

        if !self.providers.contains_key(&provider_name) {
            let config = ProviderConfig::new(&domain);
            self.providers.insert(provider_name.clone(), config);
        }

        let config = self.providers.get(&provider_name).unwrap();
        (provider_name, config)
    }

    pub fn get_config(&self, provider: &str) -> Option<&ProviderConfig> {
        self.providers.get(provider)
    }

    pub fn update_config(
        &mut self,
        provider: &str,
        strip_pattern: Option<String>,
        display_format: Option<DisplayFormat>,
    ) {
        if let Some(config) = self.providers.get_mut(provider) {
            if let Some(pattern) = strip_pattern {
                config.strip_pattern = pattern;
            }
            if let Some(format) = display_format {
                config.display_format = format;
            }
        }
    }
}

fn extract_domain(url: &str) -> String {
    url.split('/').nth(2).unwrap_or(url).to_string()
}
