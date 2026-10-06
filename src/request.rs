use crate::config::Config;
use topcoat::{Result, runtime::procedure};

pub async fn post_state(
    mode: &str,
    brightness: &str,
    speed: &str,
    r: &str,
    g: &str,
    b: &str,
) -> std::result::Result<String, String> {
    let body = serde_json::json!({
        "mode": mode,
        "brightness": brightness,
        "speed": speed,
        "r": r,
        "g": g,
        "b": b,
    });

    let config = Config::load();
    let response = reqwest::Client::new()
        .post(config.ip)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to submit update: {e}"))?;

    if !response.status().is_success() {
        return Err("Failed to submit update".to_string());
    }

    Ok("ok".to_string())
}

#[procedure]
pub async fn submit_state(
    mode: String,
    brightness: String,
    speed: String,
    r: String,
    g: String,
    b: String,
) -> Result<std::result::Result<String, String>> {
    println!("{}", mode);
    Ok(post_state(&mode, &brightness, &speed, &r, &g, &b).await)
}
