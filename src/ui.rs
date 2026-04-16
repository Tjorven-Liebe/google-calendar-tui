use ratatui::{prelude::*, widgets::*};
use chrono::{Datelike, Timelike};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Header
    f.render_widget(
        Paragraph::new(" 's': Kalender wählen | 'q': Beenden ")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).fg(Color::Cyan)),
        chunks[0]
    );

    // Kalender-Grid (Zeitskala + 7 Wochentage)
    let grid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(7), // Zeitspalte
            Constraint::Percentage(14), Constraint::Percentage(14), Constraint::Percentage(14),
            Constraint::Percentage(14), Constraint::Percentage(14), Constraint::Percentage(14),
            Constraint::Percentage(14),
        ])
        .split(chunks[1]);

    // 1. Zeitskala (08:00 - 20:00)
    let hours: Vec<ListItem> = (8..21).map(|h| ListItem::new(format!("{:02}:00", h))).collect();
    f.render_widget(List::new(hours).block(Block::default().borders(Borders::RIGHT)), grid[0]);

    // 2. Wochentage und Events
    let days = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];
    for (i, name) in days.iter().enumerate() {
        let day_area = grid[i + 1];
        f.render_widget(Block::default().borders(Borders::LEFT).title(*name), day_area);

        // Nutzt jetzt die Cache-Funktion aus app.rs
        for event in app.get_visible_events() {
            let dt = event.start_dt().with_timezone(&chrono::Local);

            // Prüfen, ob das Event zum aktuellen Wochentag passt (1-basiert)
            if (dt.weekday().number_from_monday() as usize) == (i + 1) {
                let hour = dt.hour();

                // Nur anzeigen, wenn im sichtbaren Zeitfenster
                if hour >= 8 && hour <= 20 {
                    let y_pos = day_area.y + (hour - 8) as u16 + 1;

                    f.render_widget(
                        Paragraph::new(event.summary.as_str())
                            .style(Style::default().bg(Color::Blue).fg(Color::White)),
                        Rect::new(
                            day_area.x + 1,
                            y_pos,
                            day_area.width.saturating_sub(1),
                            1
                        )
                    );
                }
            }
        }
    }

    // Overlay für die Kalender-Auswahl
    if app.show_selection {
        let block = Block::default()
            .title(" Kalender Auswahl ")
            .borders(Borders::ALL)
            .bg(Color::Black);

        let items: Vec<ListItem> = app.all_calendars.iter().enumerate().map(|(i, cal)| {
            let check = if app.active_calendar_ids.contains(&cal.id) { "[X] " } else { "[ ] " };
            let style = if i == app.cursor {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!("{}{}", check, cal.summary)).style(style)
        }).collect();

        let popup_area = centered_rect(60, 60, area);
        f.render_widget(Clear, popup_area); // Wichtig, um den Kalender dahinter zu überlagern
        f.render_widget(List::new(items).block(block), popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]).split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]).split(popup_layout[1])[1]
}