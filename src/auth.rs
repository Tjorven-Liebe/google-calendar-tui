use serde::{Deserialize, Serialize};
use std::process::Command;
use tiny_http::{Server, Response};
use url::Url;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub fn get_access_token(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let params = [
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
        ("refresh_token", config.refresh_token.as_str()),
        ("grant_type", "refresh_token"),
    ];

    let res = client.post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()?;

    let status = res.status();
    let text = res.text()?;
    let token_res: TokenResponse = serde_json::from_str(&text)?;

    if let Some(token) = token_res.access_token {
        Ok(token)
    } else {
        Err(format!(
            "Google API Fehler (Status {}): {} - {}",
            status,
            token_res.error.unwrap_or_default(),
            token_res.error_description.unwrap_or_default()
        ).into())
    }
}

pub fn perform_browser_auth(client_id: &str, client_secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let redirect_uri = "http://localhost:8080";
    let scope = "https://www.googleapis.com/auth/calendar.readonly";
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
        client_id, redirect_uri, scope
    );

    Command::new("xdg-open").arg(&auth_url).spawn()?;
    let server = Server::http("127.0.0.1:8080").map_err(|e| e.to_string())?;

    if let Some(request) = server.incoming_requests().next() {
        let url = Url::parse(&format!("http://localhost{}", request.url()))?;
        let code = url.query_pairs()
            .find(|(k, _)| k == "code")
            .map(|(_, v)| v.into_owned())
            .ok_or("Code nicht gefunden")?;

        request.respond(Response::from_string("Erfolg! Du kannst das Fenster schließen."))?;

        let client = reqwest::blocking::Client::new();
        let res = client.post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code.as_str()),
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("redirect_uri", redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()?;

        let json: serde_json::Value = res.json()?;
        return Ok(json["refresh_token"].as_str().ok_or("Kein Refresh Token erhalten")?.to_string());
    }
    Err("Authentifizierung abgebrochen".into())
}