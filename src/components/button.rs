use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(into)] text: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(default = "button")] button_type: &'static str,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] active: Option<Signal<bool>>,
) -> impl IntoView {
    let disabled = move || match active {
        Some(active) => !active.get(),
        None => false,
    };

    view! {
        <div class="block flex-col">
            <div class="grow"></div>
            <div class="flex-row">
                <div class="grow"></div>
                <button
                    type=button_type
                    disabled=disabled
                    on:click=move |_| {
                        if let Some(on_click) = on_click {
                            on_click.run(());
                        }
                    }
                    class=format!(
                        "block disabled:pointer-events-none disabled:opacity-60 {class}",
                    )
                >
                    {text}
                </button>
            </div>
        </div>
    }
}
