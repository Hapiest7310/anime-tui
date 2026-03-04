mod core;
#[cfg(feature = "tui")]
mod tui;

use clap::{Parser, Subcommand};
use core::commands;
use core::config::ProviderConfigs;
use core::models::AnimeList;

#[derive(Parser)]
#[command(name = "anime")]
#[command(about = "Anime list manager - CLI and TUI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long)]
    tui: bool,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        url: String,
        #[arg(short, long)]
        name: Option<String>,
    },
    List {
        #[arg(short, long, default_value = "name")]
        fields: String,
        #[arg(short, long, default_value = "name")]
        sort: String,
        #[arg(long, default_value = "false")]
        reverse: bool,
    },
    Edit {
        id: String,
        name: String,
    },
    Delete {
        id: String,
    },
    Watch {
        id: String,
    },
    Sort {
        field: String,
        #[arg(long, default_value = "false")]
        reverse: bool,
    },
}

fn run_cli() -> Result<(), String> {
    let cli = Cli::parse();

    let mut anime_list = AnimeList::load();
    let mut providers = ProviderConfigs::load();

    if let Some(command) = cli.command {
        match command {
            Commands::Add { url, name } => {
                commands::add_anime(&url, name.as_deref(), &mut anime_list, &mut providers)?;
            }
            Commands::List {
                fields,
                sort,
                reverse,
            } => {
                let field_list: Vec<&str> = fields.split(',').map(|s| s.trim()).collect();
                let result = commands::list_anime(&anime_list, &field_list, &sort, reverse)?;
                println!("{}", result);
            }
            Commands::Edit { id, name } => {
                commands::edit_anime(&id, &name, &mut anime_list)?;
            }
            Commands::Delete { id } => {
                commands::delete_anime(&id, &mut anime_list)?;
            }
            Commands::Watch { id } => {
                commands::watch_anime(&id, &anime_list)?;
            }
            Commands::Sort { field, reverse } => {
                commands::set_sort(&field, reverse, &mut anime_list)?;
            }
        }

        anime_list.save().map_err(|e| e.to_string())?;
        providers.save().map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if TUI mode is requested
    #[cfg(feature = "tui")]
    if args.contains(&"--tui".to_string()) || (args.len() == 1) {
        let mut anime_list = AnimeList::load();
        let mut providers = ProviderConfigs::load();

        if let Err(e) = tui::run_tui(&mut anime_list, &mut providers) {
            eprintln!("TUI Error: {}", e);
            eprintln!("\nFalling back to CLI mode...");
            if let Err(e) = run_cli() {
                eprintln!("CLI Error: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    #[cfg(not(feature = "tui"))]
    if args.contains(&"--tui".to_string()) {
        eprintln!("TUI not available. Compile with --features tui to enable.");
        std::process::exit(1);
    }

    // CLI mode
    if let Err(e) = run_cli() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
