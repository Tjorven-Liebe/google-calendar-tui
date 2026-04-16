use std::collections::{HashSet, HashMap};
use crate::google::{CalendarEntry, CalendarEvent};

pub struct App {
    pub all_calendars: Vec<CalendarEntry>,
    pub active_calendar_ids: HashSet<String>,
    // Der Cache speichert die Events pro Kalender-ID
    pub cache: HashMap<String, Vec<CalendarEvent>>,
    pub show_selection: bool,
    pub cursor: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            all_calendars: Vec::new(),
            active_calendar_ids: HashSet::new(),
            cache: HashMap::new(),
            show_selection: false,
            cursor: 0,
        }
    }

    // Das ist die Funktion, die in ui.rs gesucht wird:
    pub fn get_visible_events(&self) -> Vec<&CalendarEvent> {
        self.active_calendar_ids
            .iter()
            .filter_map(|id| self.cache.get(id))
            .flatten()
            .collect()
    }
}