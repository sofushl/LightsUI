use crate::components::Button;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="flex flex-col items-center gap-4 p-8">
            <p class="text-2xl font-semibold text-slate-800">
                "Clicks: " {count}
            </p>
            <Button
                class="p-2 rounded-lg cursor-pointer hover:bg-blue-200 bg-blue-100 dark:bg-indigo-500 dark:hover:bg-indigo-400 text-black border-black"
                text="Submit"
                on_click=Callback::new(move |_| set_count.update(|n| *n += 1))
            />
        </div>
    }
}
