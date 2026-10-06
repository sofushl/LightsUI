use topcoat::{
    Result,
    runtime::{Event, Signal},
    view::{View, component, view},
};

use crate::request::submit_state;

#[component]
pub async fn number_input(display: &str, value: &Signal<String>) -> Result<impl View> {
    Ok(view! {
        <div class="flex justify-between">
            <div class="">(display)</div>
            <input
                type="range"
                min="0"
                max="255"
                step="1"
                :value=(value.get())
                @input=$(|e: Event| value.set(e.target.value))
            >
        </div>
    })
}

#[component]
pub async fn string_input(display: &str, value: &Signal<String>) -> Result<impl View> {
    Ok(view! {
        <div class="flex justify-between">
            <div class="">(display)</div>
            <input :value=(value.get()) @input=$(|e: Event| value.set(e.target.value))>
        </div>
    })
}

#[component]
pub async fn submit_button(
    display: &str,
    mode: &Signal<String>,
    brightness: &Signal<String>,
    speed: &Signal<String>,
    r: &Signal<String>,
    g: &Signal<String>,
    b: &Signal<String>,
) -> Result<impl View> {
    Ok(view! {
        <div class="p-5 flex justify-center">
            <button
                class="px-4 py-3 rounded-lg bg-blue-100"
                @click=$(async |_e| {
                    let _result = submit_state(
                        mode.get(),
                        brightness.get(),
                        speed.get(),
                        r.get(),
                        g.get(),
                        b.get(),
                    ).await;
                })
            >
                (display)
            </button>
        </div>
    })
}
