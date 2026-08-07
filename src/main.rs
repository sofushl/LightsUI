use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="flex flex-col items-center gap-4 p-8">
            <p class="text-2xl font-semibold text-slate-800">
                "Clicks: " {count}
            </p>
            <button
                class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Click me"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
