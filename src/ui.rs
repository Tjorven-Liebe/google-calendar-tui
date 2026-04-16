use ratatui::{prelude::*, widgets::*};
use chrono::{Datelike, Timelike};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    f.render_widget(
        Paragraph::new(" 's': Kalender wählen | 'q': Beenden ")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).fg(Color::Cyan)),
        chunks[0]
    );

    let grid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(7),
            Constraint::Percentage(14), Constraint::Percentage(14), Constraint::Percentage(14),
            Constraint::Percentage(14), Constraint::Percentage(14), Constraint::Percentage(14),
            Constraint::Percentage(14),
        ])
        .split(chunks[1]);

    let hours: Vec<ListItem> = (8..21).map(|h| ListItem::new(format!("{:02}:00", h))).collect();
    f.render_widget(List::new(hours).block(Block::default().borders(Borders::RIGHT)), grid[0]);

    let days = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];
    for (i, name) in days.iter().enumerate() {
        let day_area = grid[i + 1];
        f.render_widget(Block::default().borders(Borders::LEFT).title(*name), day_area);

        for event in &app.calendar_events {
            if app.active_calendar_ids.contains(&event.calendar_id) {
                let dt = event.start_dt().with_timezone(&chrono::Local);
                if (dt.weekday().number_from_monday() as usize) == (i + 1) {
                    let hour = dt.hour();
                    if hour >= 8 && hour <= 20 {
                        let y_pos = day_area.y + (hour - 8) as u16 + 1;
                        f.render_widget(
                            Paragraph::new(event.summary.as_str())
                                .style(Style::default().bg(Color::Blue).fg(Color::White)),
                            Rect::new(day_area.x + 1, y_pos, day_area.width.saturating_sub(1), 1)
                        );
                    }
                }
            }
        }
    }

    if app.show_selection {
        let block = Block::default().title(" Kalender Auswahl ").borders(Borders::ALL).bg(Color::Black);
        let items: Vec<ListItem> = app.all_calendars.iter().enumerate().map(|(i, cal)| {
            let check = if app.active_calendar_ids.contains(&cal.id) { "[X] " } else { "[ ] " };
            let style = if i == app.cursor { Style::default().fg(Color::Yellow) } else { Style::default() };
            ListItem::new(format!("{}{}", check, cal.summary)).style(style)
        }).collect();

        let area = centered_rect(60, 60, area);
        f.render_widget(Clear, area);
        f.render_widget(List::new(items).block(block), area);
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