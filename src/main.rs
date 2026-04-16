mod auth; mod google; mod app; mod ui;
use crossterm::{event::{self, Event, KeyCode}, terminal::*, execute};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cred_path = format!("{}/.config/google_creds.json", std::env::var("HOME")?);
    let mut config: auth::Config = serde_json::from_str(&fs::read_to_string(&cred_path)?)?;

    if config.refresh_token.is_empty() {
        config.refresh_token = auth::perform_browser_auth(&config.client_id, &config.client_secret)?;
        fs::write(&cred_path, serde_json::to_string_pretty(&config)?)?;
    }

    let mut app = app::App::new();
    let token = auth::get_access_token(&config)?; // Hier kommt jetzt die Fehlermeldung, falls was nicht stimmt

    app.all_calendars = google::fetch_calendar_list(&token);
    for cal in &app.all_calendars {
        if cal.selected {
            app.active_calendar_ids.insert(cal.id.clone());
            app.calendar_events.extend(google::fetch_events_for(&token, &cal.id));
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    loop {
        terminal.draw(|f| ui::render(f, &app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('s') => app.show_selection = !app.show_selection,
                        KeyCode::Up => if app.show_selection && app.cursor > 0 { app.cursor -= 1 },
                        KeyCode::Down => if app.show_selection && app.cursor < app.all_calendars.len() - 1 { app.cursor += 1 },
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if app.show_selection {
                                let id = app.all_calendars[app.cursor].id.clone();
                                app.toggle_calendar(id);
                                app.calendar_events.clear();
                                for active_id in &app.active_calendar_ids {
                                    app.calendar_events.extend(google::fetch_events_for(&token, active_id));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}