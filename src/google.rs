use serde::Deserialize;
use chrono::{DateTime, Utc};
use rayon::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct CalendarEntry {
    pub id: String,
    pub summary: String,
    #[serde(default)]
    pub selected: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CalendarEvent {
    pub summary: String,
    pub start: EventTime,
    #[serde(default)]
    pub calendar_id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EventTime {
    #[serde(alias = "dateTime", alias = "date")]
    pub time: String,
}

impl CalendarEvent {
    pub fn start_dt(&self) -> DateTime<Utc> {
        let t = if self.start.time.len() == 10 {
            format!("{}T00:00:00Z", self.start.time)
        } else {
            self.start.time.clone()
        };
        DateTime::parse_from_rfc3339(&t)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    }
}

pub fn fetch_calendar_list(token: &str) -> Vec<CalendarEntry> {
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .bearer_auth(token).send();

    if let Ok(response) = res {
        let json: serde_json::Value = response.json().unwrap_or_default();
        return serde_json::from_value(json["items"].clone()).unwrap_or_default();
    }
    vec![]
}

pub fn fetch_events_parallel(token: &str, ids: Vec<String>) -> Vec<(String, Vec<CalendarEvent>)> {
    ids.into_par_iter()
        .map(|id| {
            let events = fetch_events_for(token, &id);
            (id, events)
        })
        .collect()
}

pub fn fetch_events_for(token: &str, calendar_id: &str) -> Vec<CalendarEvent> {
    let client = reqwest::blocking::Client::new();
    let now = Utc::now();
    let time_min = (now - chrono::Duration::days(1)).format("%Y-%m-%dT00:00:00Z").to_string();

    let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events", urlencoding::encode(calendar_id));
    let res = client.get(url)
        .bearer_auth(token)
        .query(&[("singleEvents", "true"), ("timeMin", &time_min)])
        .send();

    if let Ok(response) = res {
        let json: serde_json::Value = response.json().unwrap_or_default();
        if let Ok(mut events) = serde_json::from_value::<Vec<CalendarEvent>>(json["items"].clone()) {
            for e in &mut events { e.calendar_id = calendar_id.to_string(); }
            return events;
        }
    }
    vec![]
}