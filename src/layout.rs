use topcoat::{
    Result,
    router::{Slot, layout},
    tailwind,
    view::{View, view},
};

#[layout("/")]
pub async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"lightsUI"</title>
                topcoat::dev::script()
                topcoat::runtime::script()
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>

            <body class="bg-amber-50 text-stone-900 min-h-screen">
                <nav class="px-6 py-4 flex gap-6">
                    <a class="font-semibold" href="/">"Dashboard"</a>
                    <a
                        class="font-stone-700"
                        href="https://github.com/sofushl/rgbTriangles"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "rgbTriangles"
                    </a>
                    <a
                        class="font-stone-700"
                        href="https://github.com/sofushl/LightsUI"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "LightsUI"
                    </a>
                </nav>
                <main class="max-w-2xl mx-auto px-6 py-10">(slot)</main>
            </body>
        </html>
    })
}
