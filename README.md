# anime-term-tool

A backend-agnostic anime list manager with both CLI and TUI interfaces.

## Overview

The project is designed with a **plugin architecture** where the core business logic is separated from the interface (CLI/TUI). This allows:

- **Backend agnostic**: Core logic knows nothing about how it's accessed
- **Multiple interfaces**: CLI for scripting, TUI for interactive use
- **Minimal dependencies**: CLI-only build has zero TUI dependencies
- **Plugin system**: Future interfaces (web, API) can be added as plugins

## Current Status

- **CLI**: Fully functional
- **TUI**: Not implemented yet
- **Plugin System**: Designed but not implemented

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Entry Point                          │
│                    (main.rs)                            │
└───────────────────────┬─────────────────────────────────┘
                        │
         ┌──────────────┴──────────────┐
         ▼                              ▼
┌─────────────────────┐    ┌─────────────────────┐
│       CLI Mode       │    │      TUI Mode       │
│    (clap/args)      │    │    (plugin)         │
│                     │    │  [not implemented] │
└─────────┬───────────┘    └─────────┬───────────┘
          │                           │
          └─────────────┬─────────────┘
                        ┌──────────────────── ▼
           ───┐
            │     Core Library       │
            │   (src/core/)         │
            ├───────────────────────┤
            │ - commands.rs         │
            │ - config.rs           │
            │ - models.rs           │
            └───────────────────────┘
                        │
                        ▼
            ┌───────────────────────┐
            │      Data Layer        │
            │  (JSON files)          │
            └───────────────────────┘
```

## Project Structure

```
anime-term-tool/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs           # Entry point, CLI parsing, dispatch
    └── core/
        ├── mod.rs       # Module declarations
        ├── commands.rs  # Business logic (add, list, edit, delete, watch, sort)
        ├── config.rs    # ProviderConfig, ProviderConfigs
        └── models.rs    # Anime, AnimeList, SortBy
```

## Data Structures

### Anime

```rust
pub struct Anime {
    pub id: String,           // UUID v4
    pub name: String,         // Friendly name (auto-generated or custom)
    pub url: String,           // Provider URL
    pub provider: String,     // Provider name (extracted from domain)
    pub added: DateTime<Utc>,  // Timestamp
}
```

### AnimeList

```rust
pub struct AnimeList {
    pub anime: Vec<Anime>,
    pub sort_by: SortBy,  // Name, Date, or Provider
}
```

### SortBy

```rust
pub enum SortBy {
    Name,
    Date,      // Default, newest first
    Provider,
}
```

### ProviderConfig

```rust
pub struct ProviderConfig {
    pub domain: String,
    pub strip_pattern: String,      // Regex to remove from URL (e.g., "[0-9]+-")
    pub display_format: DisplayFormat,  // Hyphen, Slash, CamelCase, Original
}
```

## CLI Usage

```bash
# Add anime from URL
anime add "https://jutsuz.org/123-naruto"
anime add "https://jutsuz.org/456-naruto" -n "Custom Name"

# List anime
anime list                                    # name only, sorted alphabetically
anime list -f name,date,provider,uuid        # all fields
anime list -s date                            # sort by date
anime list -s date --reverse                  # sort by date, oldest first

# Edit/Delete
anime edit "naruto" "New Name"
anime delete "naruto"

# Watch (opens URL in browser)
anime watch "naruto"

# Set default sort preference
anime sort date
anime sort name --reverse
```

### List Command Options

| Option | Description | Default |
|--------|-------------|---------|
| `-f, --fields` | Fields to display (name, date, provider, uuid) | name |
| `-s, --sort` | Sort field (name, date, provider) | name |
| `--reverse` | Reverse sort order | false |

## Configuration

Data is stored in `~/.local/share/anime/`:

```
~/.local/share/anime/
├── anime.json       # Anime list
└── providers.json   # Provider configurations
```

## Dependencies

### CLI-Only Build
- serde, serde_json
- chrono
- dirs
- regex
- uuid
- clap

### Full Build (with TUI)
- All of the above plus:
- ratatui
- crossterm
- atty

## Future Plans

### Phase 1: Core & CLI (Current)
- [x] Core business logic
- [x] CLI interface
- [x] Data persistence

### Phase 2: TUI Plugin
- [ ] Implement TUI using core commands
- [ ] Plugin trait in core
- [ ] Runtime detection of terminal

### Phase 3: Plugin System
- [ ] Formalize Plugin trait
- [ ] Dynamic plugin loading (optional)
- [ ] Multiple interface support

### Phase 4: Packaging
- [ ] AUR package for yay/pacman
- [ ] Release builds

## Package Info (for AUR)

- **Package Name**: anime-term-tool
- **Version**: 0.1.0
- **License**: MIT
- **Architecture**: x86_64
- **Dependencies**: None (static binary## Building)



```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Install locally
cargo install --path .
```

## Design Decisions

1. **Friendly Names**: Auto-generated from URL (stripped pattern), editable by user
2. **Sort Default**: `-s name` alphabetically (CLI), `date` newest first (persisted preference)
3. **Backend Agnostic**: Core has no dependencies on CLI/TUI - can be used as a library
4. **Plugin Architecture**: TUI is a plugin, not hardcoded
