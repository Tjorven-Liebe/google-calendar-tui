mod auth;
mod google;
mod app;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::*,
    execute,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Konfiguration laden
    let cred_path = format!("{}/.config/google_creds.json", std::env::var("HOME")?);
    let config_data = fs::read_to_string(&cred_path)?;
    let mut config: auth::Config = serde_json::from_str(&config_data)?;

    // 2. OAuth2-Check: Falls kein Refresh-Token da ist, Browser-Auth starten
    if config.refresh_token.is_empty() {
        config.refresh_token = auth::perform_browser_auth(&config.client_id, &config.client_secret)?;
        fs::write(&cred_path, serde_json::to_string_pretty(&config)?)?;
    }

    // 3. App-Zustand initialisieren und Daten parallel laden
    let mut app = app::App::new();
    let token = auth::get_access_token(&config)?;

    // Liste aller verfügbaren Kalender holen
    app.all_calendars = google::fetch_calendar_list(&token);

    // IDs der Kalender sammeln, die laut Google "selected" sind
    let initial_ids: Vec<String> = app.all_calendars
        .iter()
        .filter(|c| c.selected)
        .map(|c| c.id.clone())
        .collect();

    // Diese IDs als aktiv markieren
    for id in &initial_ids {
        app.active_calendar_ids.insert(id.clone());
    }

    // PARALLELES LADEN: Alle initialen Kalender-Events gleichzeitig abrufen
    let results = google::fetch_events_parallel(&token, initial_ids);
    for (id, events) in results {
        app.cache.insert(id, events);
    }

    // 4. Terminal-Setup (Raw Mode & Alternate Screen)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 5. Main Loop
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Nur auf Tastendruck reagieren (verhindert doppeltes Triggering unter manchen Terminals)
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('s') => app.show_selection = !app.show_selection,
                        KeyCode::Up => {
                            if app.show_selection && app.cursor > 0 {
                                app.cursor -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.show_selection && app.cursor < app.all_calendars.len() - 1 {
                                app.cursor += 1;
                            }
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if app.show_selection {
                                let id = app.all_calendars[app.cursor].id.clone();
                                if app.active_calendar_ids.contains(&id) {
                                    app.active_calendar_ids.remove(&id);
                                } else {
                                    app.active_calendar_ids.insert(id.clone());
                                    // CACHING: Nur laden, wenn noch nicht im Speicher
                                    if !app.cache.contains_key(&id) {
                                        // Einzelnen Kalender nachladen (synchron, da Nutzeraktion)
                                        let events = google::fetch_events_for(&token, &id);
                                        app.cache.insert(id, events);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // 6. Cleanup: Terminal wieder in Normalzustand versetzen
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}