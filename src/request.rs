use crate::{
    config::Config,
    state::{LedCommand, LedState},
};
use topcoat::{Result, runtime::procedure};

#[procedure]
pub async fn submit_state(state: LedState) -> Result<String> {
    let Ok(command) = LedCommand::try_from(state) else {
        return Ok("Invalid value".into());
    };

    println!("{command:?}");

    let config = Config::load();
    let sent = reqwest::Client::new()
        .post(config.picoip)
        .json(&command)
        .send()
        .await
        .and_then(|r| r.error_for_status());

    Ok(match sent {
        Ok(_) => "ok".into(),
        Err(e) => format!("Failed to submit update: {e}"),
    })
}
