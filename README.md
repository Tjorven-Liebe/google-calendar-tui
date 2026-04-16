# Google Calendar TUI

Eine leistungsstarke Terminal-Benutzeroberfläche (TUI) in Rust zur Visualisierung von Google Calendar Terminen in einer grafischen Wochenansicht. Das Tool bietet eine effiziente Alternative zum Browser und integriert sich nahtlos in Linux-Workflows.

## Features

- **Grafisches Wochen-Grid**: Zeitgenaue Darstellung von Terminen (Skala von 08:00 bis 20:00 Uhr).
- **Multi-Kalender Support**: Lädt alle mit dem Account verknüpften Kalender (z. B. Privat, Uni, Feiertage).
- **Interaktive Auswahl**: Kalender können über ein In-App-Menü dynamisch ein- und ausgeblendet werden.
- **Automatisches OAuth2**: Bezieht Access-Tokens über den Browser und verwaltet Refresh-Tokens in einer lokalen Konfigurationsdatei.
- **Performanz**: Geschrieben in Rust auf Basis von `ratatui` für minimale CPU-Last und hohe Reaktivität.

## Installation

### 1. API-Voraussetzungen
1. Erstelle ein Projekt in der [Google Cloud Console](https://console.cloud.google.com/).
2. Aktiviere die **Google Calendar API**.
3. Erstelle unter **Credentials** eine **OAuth 2.0 Client ID** vom Typ "Desktop App".
4. Füge `http://localhost:8080` als autorisierte Redirect-URI hinzu.

### 2. Konfiguration anlegen
Erstelle die Datei `~/.config/google_creds.json` und fülle sie mit deinen Daten:

```json
{
  "client_id": "DEINE_ID.apps.googleusercontent.com",
  "client_secret": "DEIN_SECRET",
  "refresh_token": ""
}
```

### 3. Build & Install
Führe im Projektverzeichnis folgenden Befehl aus, um die App systemweit verfügbar zu machen:

```bash
cargo install --path .
```

Stelle sicher, dass `~/.cargo/bin` in deiner `$PATH`-Variable enthalten ist.

## Nutzung

Starte die App mit dem Namen deines Cargo-Projekts (Standard: `google-calendar-tui`):

```bash
google-calendar-tui
```

### Tastatursteuerung

| Taste | Funktion |
| :--- | :--- |
| `q` | Anwendung sicher beenden |
| `s` | Kalender-Auswahlmenü öffnen/schließen |
| `↑` / `↓` | Navigation im Auswahlmenü |
| `Space` / `Enter` | Gewählten Kalender aktivieren oder deaktivieren |

## Projektstruktur

- `main.rs`: Orchestrierung, Terminal-Setup und Event-Loop.
- `auth.rs`: Handhabung der OAuth2-Flows und Token-Verwaltung.
- `google.rs`: API-Anfragen für Kalenderlisten und Events.
- `app.rs`: Zentraler State der Anwendung.
- `ui.rs`: Layout-Definition und grafisches Rendering des Grids.

## Fehlerbehebung

- **Unauthorized / invalid_client**: Überprüfe die `client_id` in der JSON-Datei. Kopiere sie am besten erneut aus der Cloud Console.
- **Leere Anzeige**: Drücke `s` und stelle sicher, dass deine Kalender mit `[X]` markiert sind. Die App speichert den Status für den nächsten Start.
- **Browser-Fehler**: Wenn der Browser-Auth-Flow nicht startet, stelle sicher, dass der Port `8080` nicht von einer anderen Anwendung belegt ist.
