use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::Button;

#[component]
pub fn App() -> impl IntoView {
    let (brightness, set_brightness) = signal(0);
    let (status, set_status) = signal(String::new());

    let on_click = move |_| {
        let value = brightness.get(); // read once, outside the future
        spawn_local(async move {
            match submit_brightness(value).await {
                Ok(()) => set_status.set("sent".to_string()),
                Err(e) => set_status.set(e),
            }
        });
    };

    view! {
        <div class="flex flex-col items-center gap-4 p-8">
            <p class="text-2xl font-semibold text-slate-800">
                "brightness: " {brightness}
            </p>
            <input
                type="range" min="0" max="100"
                prop:value=brightness
                on:input=move |ev| {
                    set_brightness.set(event_target_value(&ev).parse().unwrap_or(0));
                }
            />
            <Button
                class="p-2 rounded-lg cursor-pointer hover:bg-blue-200 bg-blue-100 dark:bg-indigo-500 dark:hover:bg-indigo-400 text-black border-black"
                text="Set"
                on_click=Callback::new(on_click)//move |_| set_count.update(|n| *n += 1))
            />
        </div>
    }
}

async fn submit_brightness(brightness: i32) -> Result<(), String> {
    let body = serde_json::json!({ "brightness": brightness, "email": "sofushl@proton.me", "message": "testbrightness", "subject": "testbrightness"});
    let response = Request::post("https://sofus.privatedns.org/email")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| format!("build request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("send: {e}"))?;

    if !response.ok() {
        return Err(format!("server returned {}", response.status()));
    }
    Ok(())
}
