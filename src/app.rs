use std::collections::HashSet;
use crate::google::{CalendarEntry, CalendarEvent};

pub struct App {
    pub all_calendars: Vec<CalendarEntry>,
    pub active_calendar_ids: HashSet<String>,
    pub calendar_events: Vec<CalendarEvent>,
    pub show_selection: bool,
    pub cursor: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            all_calendars: Vec::new(),
            active_calendar_ids: HashSet::new(),
            calendar_events: Vec::new(),
            show_selection: false,
            cursor: 0,
        }
    }

    pub fn toggle_calendar(&mut self, id: String) {
        if self.active_calendar_ids.contains(&id) {
            self.active_calendar_ids.remove(&id);
        } else {
            self.active_calendar_ids.insert(id);
        }
    }
}