#[cfg(feature = "tui")]
mod app;
#[cfg(feature = "tui")]
mod handlers;
#[cfg(feature = "tui")]
pub mod theme;

#[cfg(feature = "tui")]
use crate::core::{config::ProviderConfigs, models::AnimeList};
#[cfg(feature = "tui")]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(feature = "tui")]
use ratatui::{backend::CrosstermBackend, Terminal};
#[cfg(feature = "tui")]
use std::io;

#[cfg(feature = "tui")]
pub fn run_tui(
    anime_list: &mut AnimeList,
    providers: &mut ProviderConfigs,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new(
        anime_list.anime.clone(),
        anime_list.sort_by,
        providers.providers.keys().cloned().collect(),
    );

    let res = run_app(&mut terminal, &mut app, anime_list, providers);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    anime_list.save()?;
    providers.save()?;

    res.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[cfg(feature = "tui")]
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut app::App,
    anime_list: &mut AnimeList,
    providers: &mut ProviderConfigs,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| app::render(f, app))?;

        if app.message.is_some() {
            app.clear_message();
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                handlers::handle_key(key, app, anime_list, providers)?;

                if let crossterm::event::KeyCode::Char('q') = key.code {
                    if app.view == app::View::List {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
