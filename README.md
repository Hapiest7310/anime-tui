# anime-tui

A backend-agnostic anime list manager with CLI and TUI (Terminal UI) interfaces.

## What is it?

**anime-tui** is a lightweight, command-line tool for managing your anime watchlist. It supports:

- **CLI mode** - Add, list, edit, and delete anime from the command line
- **TUI mode** - Interactive terminal UI with keyboard navigation
- **Multiple providers** - Add anime from any URL (Crunchyroll, MyAnimeList, etc.)
- **Auto-detection** - Anime names are extracted automatically from URLs
- **Multi-select delete** - Delete multiple anime at once with confirmation
- **Custom columns** - Choose which fields to display (name, date, provider, URL, etc.)
- **Local storage** - All data stored in `~/.local/share/anime/` as JSON
- **No cloud** - Works offline, respects your privacy

## Quick Start

```bash
# Add anime
anime add "https://crunchyroll.com/naruto"
anime add "https://myanimelist.net/anime/1" -n "Cowboy Bebop"

# List your anime
anime list
anime list -f name,provider,date

# Launch interactive TUI
anime --tui

# Edit and delete
anime edit "naruto" "Naruto Shippuden"
anime delete "naruto"

# Open anime URL in browser
anime watch "attack-on-titan"
```

## TUI Keyboard Controls

### List View
- `↑/↓` or `k/j` - Navigate
- `a` - Add anime
- `e` - Edit selected
- `d` - Delete selected
- `Shift+D` - Multi-select delete
- `s` - Sort by name/date/provider
- `w` - Watch (open URL)
- `f` - Select which fields to display
- `q` - Quit

### Delete Mode (Shift+D)
- `Space` - Select/deselect
- `Enter` - Confirm deletion
- `Esc` - Cancel

### Field Selection
- `↑/↓` - Navigate fields
- `Space/Enter` - Toggle visibility
- `Esc` - Done

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# CLI only (no TUI dependencies)
cargo build --no-default-features
```

## Installation

```bash
cargo install --path .
```

## Requirements

- Rust 1.56+ (2021 edition)
- `omarchy-launch-webapp` (for opening URLs in browser)

## Data Storage

All data is stored locally in `~/.local/share/anime/`:

- `anime.json` - Your anime list
- `providers.json` - Provider-specific configurations

You can edit these files directly if needed.

## Features

- ✅ Add anime from any provider URL
- ✅ Auto-extract anime names from URLs
- ✅ List with filtering and sorting
- ✅ Edit anime names
- ✅ Delete anime (single or multi-select)
- ✅ Open anime URLs in browser
- ✅ Interactive TUI with theme auto-detection
- ✅ Custom column display
- ✅ Local-first, no cloud sync

## Architecture

The app separates concerns into clean layers:

- **Core logic** (`src/core/`) - Backend-agnostic business logic
- **CLI** (`src/main.rs`) - Command-line interface
- **TUI** (`src/tui/`) - Terminal UI (optional feature)

This means the core can be reused for web APIs, desktop apps, or other interfaces without modification.

## License

MIT
